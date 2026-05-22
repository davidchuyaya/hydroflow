//! Threaded I/O: moves socket read/write to dedicated threads so the tick
//! thread only touches in-memory channels (no syscalls).
//!
//! If `HYDRO_NETWORKING_CORES=N` is set, spawns N I/O threads pinned to cores
//! 1..=N (main thread pins to core 0). Sockets are partitioned across threads
//! round-robin. If unset, uses a single unpinned I/O thread.
//!
//! # Calibration mode
//!
//! If `HYDRO_NET_CALIBRATE=<size>` is set, all sources return synthetic messages
//! of `<size>` bytes (never blocking), and all sinks count messages, printing
//! throughput every second. This isolates main-thread processing overhead from
//! any network or I/O thread effects.

use std::io;
use std::pin::Pin;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::task::{Context, Poll};
use std::time::Instant;

use bytes::{Bytes, BytesMut};
use futures::{Sink, SinkExt, Stream, StreamExt};
use tokio::sync::mpsc;

use crate::DynStreamSink;

struct IoThreadPool {
    handles: Vec<tokio::runtime::Handle>,
}

static IO_POOL: OnceLock<IoThreadPool> = OnceLock::new();
static NEXT_THREAD: AtomicUsize = AtomicUsize::new(0);

#[cfg(target_os = "linux")]
fn pin_thread_to_core(core: usize) {
    unsafe {
        let mut cpuset: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_SET(core, &mut cpuset);
        libc::sched_setaffinity(0, size_of_val(&cpuset), &cpuset);
    }
}

fn io_pool() -> &'static IoThreadPool {
    IO_POOL.get_or_init(|| {
        let networking_cores = std::env::var("HYDRO_NETWORKING_CORES").ok().map(|v| {
            v.parse::<usize>()
                .expect("HYDRO_NETWORKING_CORES must be a number")
        });
        let num_threads = networking_cores.unwrap_or(1);

        // Pin main Hydro logic thread to core 0 if networking cores are configured.
        if networking_cores.is_some() {
            #[cfg(target_os = "linux")]
            pin_thread_to_core(0);
        }

        let mut handles = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to create I/O runtime");
            let handle = rt.handle().clone();
            let core = i + 1; // cores 1..=N
            std::thread::Builder::new()
                .name(format!("hydro-io-{i}"))
                .spawn(move || {
                    #[cfg(target_os = "linux")]
                    if networking_cores.is_some() {
                        pin_thread_to_core(core);
                    }
                    rt.block_on(futures::future::pending::<()>())
                })
                .expect("failed to spawn I/O thread");
            handles.push(handle);
        }
        IoThreadPool { handles }
    })
}

/// Pick the next I/O thread in round-robin order.
fn next_handle() -> &'static tokio::runtime::Handle {
    let pool = io_pool();
    let idx = NEXT_THREAD.fetch_add(1, Ordering::Relaxed) % pool.handles.len();
    &pool.handles[idx]
}

// ─── Source (read) side ──────────────────────────────────────────────────────

/// A `Stream` backed by an in-memory channel. The actual socket reads happen
/// on an I/O thread which feeds items into this channel.
pub struct ChannelStreamInner {
    rx: mpsc::UnboundedReceiver<Result<BytesMut, io::Error>>,
}

/// Source stream: either a real channel-backed stream or a calibration stream.
pub enum ChannelStream {
    Channel(ChannelStreamInner),
    Calibrate(CalibrateStream),
}

impl Stream for ChannelStream {
    type Item = Result<BytesMut, io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.get_mut() {
            ChannelStream::Channel(inner) => inner.rx.poll_recv(cx),
            ChannelStream::Calibrate(s) => Pin::new(s).poll_next(cx),
        }
    }
}

impl Unpin for ChannelStream {}

/// Environment variable to enable calibration mode: `HYDRO_NET_CALIBRATE=<size>`.
pub const HYDRO_NET_CALIBRATE_ENV: &str = "HYDRO_NET_CALIBRATE";

/// Returns the calibration message size if `HYDRO_NET_CALIBRATE` is set.
fn calibrate_config() -> Option<usize> {
    static CFG: OnceLock<Option<usize>> = OnceLock::new();
    *CFG.get_or_init(|| {
        std::env::var(HYDRO_NET_CALIBRATE_ENV).ok().map(|v| {
            v.parse::<usize>()
                .expect("HYDRO_NET_CALIBRATE must be a number")
        })
    })
}

// ─── Calibration source ──────────────────────────────────────────────────────

/// A stream that always returns a fixed-size message (never Pending).
pub struct CalibrateStream {
    msg: BytesMut,
}

impl Stream for CalibrateStream {
    type Item = Result<BytesMut, io::Error>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(Ok(self.msg.clone())))
    }
}

impl Unpin for CalibrateStream {}

// ─── Calibration sink ────────────────────────────────────────────────────────

static CALIBRATE_SINK_COUNT: AtomicU64 = AtomicU64::new(0);
static CALIBRATE_REPORTER_STARTED: OnceLock<()> = OnceLock::new();

fn start_calibrate_reporter() {
    CALIBRATE_REPORTER_STARTED.get_or_init(|| {
        std::thread::Builder::new()
            .name("hydro-calibrate".into())
            .spawn(|| {
                let mut last = Instant::now();
                let mut last_count = 0u64;
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    let now = Instant::now();
                    let count = CALIBRATE_SINK_COUNT.load(Ordering::Relaxed);
                    let elapsed = now.duration_since(last).as_secs_f64();
                    let delta = count - last_count;
                    if delta > 0 {
                        println!(
                            "HYDRO_OPTIMIZE_THR: {:.2} requests/s",
                            delta as f64 / elapsed,
                        );
                    }
                    last = now;
                    last_count = count;
                }
            })
            .expect("failed to spawn calibrate reporter");
    });
}

/// A sink that counts messages for calibration (always ready, never blocks).
pub struct CalibrateSink;

impl Sink<Bytes> for CalibrateSink {
    type Error = io::Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, _item: Bytes) -> Result<(), Self::Error> {
        CALIBRATE_SINK_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

/// Offload the read half of a `DynStreamSink` to an I/O thread.
pub fn offload_source(mut source: DynStreamSink) -> ChannelStream {
    if let Some(size) = calibrate_config() {
        return ChannelStream::Calibrate(CalibrateStream {
            msg: BytesMut::from(&vec![0u8; size][..]),
        });
    }

    let (tx, rx) = mpsc::unbounded_channel();
    next_handle().spawn(async move {
        while let Some(item) = source.next().await {
            if tx.send(item).is_err() {
                break;
            }
        }
    });
    ChannelStream::Channel(ChannelStreamInner { rx })
}

// ─── Sink (write) side ───────────────────────────────────────────────────────

/// A `Sink` backed by an in-memory channel. The actual socket writes happen
/// on an I/O thread which drains items from this channel.
pub struct ChannelSinkInner {
    tx: mpsc::UnboundedSender<Bytes>,
}

/// Sink: either a real channel-backed sink or a calibration counter.
pub enum ChannelSink {
    Channel(ChannelSinkInner),
    Calibrate(CalibrateSink),
}

impl Sink<Bytes> for ChannelSink {
    type Error = io::Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        match self.get_mut() {
            ChannelSink::Channel(inner) => inner
                .tx
                .send(item)
                .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "I/O thread gone")),
            ChannelSink::Calibrate(_) => {
                CALIBRATE_SINK_COUNT.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Unpin for ChannelSink {}

/// Offload the write half of a `DynStreamSink` to an I/O thread.
/// Batches writes: drains all available items before flushing once.
pub fn offload_sink(mut sink: DynStreamSink) -> ChannelSink {
    if calibrate_config().is_some() {
        start_calibrate_reporter();
        return ChannelSink::Calibrate(CalibrateSink);
    }

    let (tx, mut rx) = mpsc::unbounded_channel::<Bytes>();
    next_handle().spawn(async move {
        while let Some(item) = rx.recv().await {
            if sink.feed(item).await.is_err() {
                break;
            }
            while let Ok(item) = rx.try_recv() {
                if sink.feed(item).await.is_err() {
                    return;
                }
            }
            if sink.flush().await.is_err() {
                break;
            }
        }
    });
    ChannelSink::Channel(ChannelSinkInner { tx })
}
