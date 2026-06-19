//! Threaded I/O: moves socket read/write to dedicated threads so the tick
//! thread only touches in-memory channels (no syscalls).
//!
//! If `HYDRO_NETWORKING_CORES=N` is set, spawns N I/O threads pinned to cores
//! 1..=N (main thread pins to core 0). Sockets are partitioned across threads
//! round-robin. If unset, uses a single unpinned I/O thread.
//!
//! # Calibration mode
//!
//! If `HYDRO_NET_CALIBRATE=<size>` is set, the socket is replaced but the real
//! channel + cross-core hand-off is kept: each read-side channel is prefilled
//! with synthetic `<size>`-byte messages, and each write-side channel is drained
//! by a pinned recycler thread that sends the echoed bytes back into a read-side
//! channel. This keeps the tick thread on the normal `mpsc` receive/send path
//! without doing socket I/O. Throughput (the recycle count) is printed every
//! second.

use std::io;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
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

/// Pin the main Hydro logic thread to core 0 in calibration mode.
///
/// Normally this pinning happens in [`io_pool`], but calibration short-circuits
/// the source/sink before the I/O pool is ever initialized, so the calibration
/// compute loop would otherwise float across cores and read as 0% on the
/// `sar -P 0` (core-0-only) sampler. Called from the `Calibrate` branches, which
/// run on the main thread during dataflow setup. Idempotent.
fn pin_calibration_main_thread() {
    #[cfg(target_os = "linux")]
    {
        static PINNED: OnceLock<()> = OnceLock::new();
        PINNED.get_or_init(|| pin_thread_to_core(0));
    }
}

/// Number of networking cores from `HYDRO_NETWORKING_CORES`. Cores `1..=N` are
/// reserved for I/O (and calibration) side threads; core 0 is the compute thread.
fn networking_cores() -> Option<usize> {
    static CFG: OnceLock<Option<usize>> = OnceLock::new();
    *CFG.get_or_init(|| {
        std::env::var("HYDRO_NETWORKING_CORES").ok().map(|v| {
            v.parse::<usize>()
                .expect("HYDRO_NETWORKING_CORES must be a number")
        })
    })
}

fn io_pool() -> &'static IoThreadPool {
    IO_POOL.get_or_init(|| {
        let networking_cores = networking_cores();
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
    calibration_depth: Option<Arc<AtomicU64>>,
}

/// Source stream backed by an in-memory channel. In calibration mode the channel
/// is prefilled and replenished by a recycler thread instead of a socket I/O
/// thread; either way the compute thread sees the same real `poll_recv`.
pub enum ChannelStream {
    Channel(ChannelStreamInner),
}

impl Stream for ChannelStream {
    type Item = Result<BytesMut, io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.get_mut() {
            ChannelStream::Channel(inner) => {
                let poll = inner.rx.poll_recv(cx);
                if let Some(depth) = &inner.calibration_depth {
                    match &poll {
                        Poll::Ready(Some(_)) => {
                            let _ =
                                depth.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |depth| {
                                    Some(depth.saturating_sub(1))
                                });
                        }
                        Poll::Pending => {
                            CALIBRATE_SOURCE_STARVED.fetch_add(1, Ordering::Relaxed);
                        }
                        Poll::Ready(None) => {}
                    }
                }
                poll
            }
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

// ─── Calibration ─────────────────────────────────────────────────────────────

/// Messages recycled by the write side (= server output rate = throughput).
static CALIBRATE_SINK_COUNT: AtomicU64 = AtomicU64::new(0);
/// Number of times a calibration source was polled empty.
static CALIBRATE_SOURCE_STARVED: AtomicU64 = AtomicU64::new(0);
static CALIBRATE_REPORTER_STARTED: OnceLock<()> = OnceLock::new();
/// Next core to pin a calibration side thread to (core 0 is the compute thread).
static CALIBRATE_NEXT_CORE: AtomicUsize = AtomicUsize::new(1);

/// Default number of messages to keep available on each fake read-side channel.
/// Override with `HYDRO_NET_CALIBRATE_RESERVOIR`.
const CALIBRATE_DEFAULT_RESERVOIR: usize = 2500000;

/// Pick (round-robin) the next core for a calibration side thread among the
/// networking cores `1..=N` (`HYDRO_NETWORKING_CORES`), leaving core 0 for the
/// compute thread. Falls back to core 1 if `HYDRO_NETWORKING_CORES` is unset.
fn next_calibration_core() -> usize {
    let n = networking_cores().unwrap_or(1).max(1);
    1 + (CALIBRATE_NEXT_CORE.fetch_add(1, Ordering::Relaxed) - 1) % n
}

fn calibrate_reservoir() -> usize {
    static CFG: OnceLock<usize> = OnceLock::new();
    *CFG.get_or_init(|| {
        std::env::var("HYDRO_NET_CALIBRATE_RESERVOIR")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&n| n >= 1)
            .unwrap_or(CALIBRATE_DEFAULT_RESERVOIR)
    })
}

#[derive(Clone)]
struct CalibrateSourceHandle {
    tx: mpsc::UnboundedSender<Result<BytesMut, io::Error>>,
    depth: Arc<AtomicU64>,
}

struct CalibrateSources {
    sources: Mutex<Vec<CalibrateSourceHandle>>,
    available: Condvar,
}

static CALIBRATE_SOURCES: OnceLock<CalibrateSources> = OnceLock::new();
static CALIBRATE_NEXT_SOURCE: AtomicUsize = AtomicUsize::new(0);

fn calibrate_sources() -> &'static CalibrateSources {
    CALIBRATE_SOURCES.get_or_init(|| CalibrateSources {
        sources: Mutex::new(Vec::new()),
        available: Condvar::new(),
    })
}

fn register_calibrate_source(source: CalibrateSourceHandle) {
    let registry = calibrate_sources();
    let mut sources = registry.sources.lock().unwrap();
    sources.push(source);
    registry.available.notify_all();
}

fn wait_for_calibrate_source() -> CalibrateSourceHandle {
    let registry = calibrate_sources();
    let mut sources = registry.sources.lock().unwrap();
    while sources.is_empty() {
        sources = registry.available.wait(sources).unwrap();
    }
    let idx = CALIBRATE_NEXT_SOURCE.fetch_add(1, Ordering::Relaxed) % sources.len();
    sources[idx].clone()
}

/// Build bytes that decode as a `Vec<u8>` and serialize back to exactly `size`
/// bytes with bincode's default fixed-width length prefix.
fn calibrate_payload_template(size: usize) -> BytesMut {
    const BINCODE_VEC_LEN_PREFIX: usize = size_of::<u64>();
    assert!(
        size >= BINCODE_VEC_LEN_PREFIX,
        "HYDRO_NET_CALIBRATE must be at least {BINCODE_VEC_LEN_PREFIX} bytes for Vec<u8>"
    );

    let payload_len = size - BINCODE_VEC_LEN_PREFIX;
    let mut template = BytesMut::with_capacity(size);
    template.extend_from_slice(&(payload_len as u64).to_le_bytes());
    template.resize(size, 0);
    template
}

/// Prefill a read-side channel with valid synthetic messages and register it as
/// a recycle target. Returns the real channel-backed source the compute thread
/// pulls from.
fn spawn_calibrate_source(size: usize) -> ChannelStream {
    let (tx, rx) = mpsc::unbounded_channel::<Result<BytesMut, io::Error>>();
    let depth = Arc::new(AtomicU64::new(0));
    let template = calibrate_payload_template(size);

    for _ in 0..calibrate_reservoir() {
        depth.fetch_add(1, Ordering::Relaxed);
        if tx.send(Ok(template.clone())).is_err() {
            let _ = depth.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |depth| {
                Some(depth.saturating_sub(1))
            });
            break;
        }
    }

    register_calibrate_source(CalibrateSourceHandle {
        tx,
        depth: depth.clone(),
    });

    ChannelStream::Channel(ChannelStreamInner {
        rx,
        calibration_depth: Some(depth),
    })
}

/// Spawn the write-side recycler: drains `rx`, counting each message, on a
/// dedicated pinned, spinning thread, then places those bytes back onto a
/// registered read-side channel. Returns the real channel-backed sink the
/// compute thread pushes to.
fn spawn_calibrate_recycler() -> ChannelSink {
    let (tx, mut rx) = mpsc::unbounded_channel::<Bytes>();
    let core = next_calibration_core();
    std::thread::Builder::new()
        .name("hydro-io-calibrate-recycle".into())
        .spawn(move || {
            #[cfg(target_os = "linux")]
            if core != 0 {
                pin_thread_to_core(core);
            }
            let source = wait_for_calibrate_source();
            let reservoir = calibrate_reservoir() as u64;
            loop {
                match rx.try_recv() {
                    Ok(item) => {
                        CALIBRATE_SINK_COUNT.fetch_add(1, Ordering::Relaxed);
                        while source.depth.load(Ordering::Relaxed) >= reservoir {
                            std::hint::spin_loop();
                        }

                        // Reclaim the echoed buffer in place when it's uniquely
                        // owned (the common case) instead of allocating + copying
                        // a fresh `BytesMut` per recycled message. The copy was a
                        // per-message bottleneck on this single recycler thread,
                        // capping the rate at which the read channel can be
                        // refilled (and starving the compute thread at batch > 1).
                        let item = item
                            .try_into_mut()
                            .unwrap_or_else(|bytes| BytesMut::from(&bytes[..]));
                        source.depth.fetch_add(1, Ordering::Relaxed);
                        if source.tx.send(Ok(item)).is_err() {
                            let _ = source.depth.fetch_update(
                                Ordering::Relaxed,
                                Ordering::Relaxed,
                                |depth| Some(depth.saturating_sub(1)),
                            );
                            break;
                        }
                    }
                    Err(mpsc::error::TryRecvError::Empty) => std::hint::spin_loop(),
                    Err(mpsc::error::TryRecvError::Disconnected) => break,
                }
            }
        })
        .expect("failed to spawn calibration recycler");
    ChannelSink::Channel(ChannelSinkInner { tx })
}

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
                    if delta > 1 {
                        println!(
                            "HYDRO_OPTIMIZE_THR: {:.2} requests/s",
                            delta as f64 / elapsed,
                        );
                    }
                    let starved = CALIBRATE_SOURCE_STARVED.swap(0, Ordering::Relaxed);
                    if starved > 0 {
                        eprintln!("HYDRO_OPTIMIZE_NET_STARVED: {starved}");
                    }
                    last = now;
                    last_count = count;
                }
            })
            .expect("failed to spawn calibrate reporter");
    });
}

/// Offload the read half of a socket to an I/O thread.
///
/// `make_source` builds (and, for a real socket, registers) the source. It runs
/// *on the I/O thread*, so the socket's readiness is registered with this I/O
/// thread's reactor rather than the runtime that accepted/connected it. That
/// keeps the caller's (main) runtime entirely off the socket's readiness path —
/// otherwise every readiness edge would be discovered by the main thread's epoll
/// and turned into a cross-thread wakeup.
pub fn offload_source(
    make_source: impl FnOnce() -> DynStreamSink + Send + 'static,
) -> ChannelStream {
    if let Some(size) = calibrate_config() {
        pin_calibration_main_thread();
        return spawn_calibrate_source(size);
    }

    let (tx, rx) = mpsc::unbounded_channel();
    next_handle().spawn(async move {
        let mut source = make_source();
        while let Some(item) = source.next().await {
            if tx.send(item).is_err() {
                break;
            }
        }
    });
    ChannelStream::Channel(ChannelStreamInner {
        rx,
        calibration_depth: None,
    })
}

// ─── Sink (write) side ───────────────────────────────────────────────────────

/// A `Sink` backed by an in-memory channel. The actual socket writes happen
/// on an I/O thread which drains items from this channel.
pub struct ChannelSinkInner {
    tx: mpsc::UnboundedSender<Bytes>,
}

/// Sink backed by an in-memory channel. In calibration mode the channel is
/// drained by a recycler thread instead of a socket I/O thread; either way the
/// compute thread performs the same real `tx.send`.
pub enum ChannelSink {
    Channel(ChannelSinkInner),
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

/// Number of cooperative `try_recv` attempts the I/O thread makes before parking
/// (awaiting) when the channel is momentarily empty.
///
/// Spinning avoids the cross-thread eventfd wakeup the producer would otherwise
/// trigger on every `send` (and the kernel `_raw_spin_unlock_irq`/IPI cost that
/// wakeup incurs across pinned cores). A receiver that never `await`s the
/// channel registers no waker, so `send` skips the wake syscall entirely. We
/// still fall back to a parking `recv()` after the budget is exhausted so an
/// idle system doesn't burn the core indefinitely.
const IO_RECV_SPIN_BUDGET: usize = 256;

/// Receive the next item, spinning cooperatively before parking. Yields between
/// attempts so sibling I/O tasks sharing this single-threaded runtime still run
/// (and the runtime still polls socket readiness for the read-side tasks).
/// Returns `None` once the channel is closed.
async fn recv_spin(rx: &mut mpsc::UnboundedReceiver<Bytes>) -> Option<Bytes> {
    for _ in 0..IO_RECV_SPIN_BUDGET {
        match rx.try_recv() {
            Ok(item) => return Some(item),
            Err(mpsc::error::TryRecvError::Disconnected) => return None,
            Err(mpsc::error::TryRecvError::Empty) => tokio::task::yield_now().await,
        }
    }
    // Budget exhausted: park until an item arrives or the channel closes. This
    // is the only path that registers a waker, so it's the only path on which
    // the producer may incur a wake syscall — rare under sustained load.
    rx.recv().await
}

/// Offload the write half of a socket to an I/O thread.
/// Drains all available items before flushing once.
///
/// `make_sink` builds (and, for a real socket, registers) the sink on the I/O
/// thread — see [`offload_source`] for why registration must happen here.
pub fn offload_sink(make_sink: impl FnOnce() -> DynStreamSink + Send + 'static) -> ChannelSink {
    if calibrate_config().is_some() {
        pin_calibration_main_thread();
        start_calibrate_reporter();
        return spawn_calibrate_recycler();
    }

    let (tx, mut rx) = mpsc::unbounded_channel::<Bytes>();
    next_handle().spawn(async move {
        let mut sink = make_sink();
        // Spin-then-park on the first item of each write burst (see `recv_spin`),
        // then drain everything else already queued and flush once.
        while let Some(item) = recv_spin(&mut rx).await {
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
