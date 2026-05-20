//! Threaded I/O: moves socket read/write to dedicated threads so the tick
//! thread only touches in-memory channels (no syscalls).
//!
//! If `HYDRO_NETWORKING_CORES=N` is set, spawns N I/O threads pinned to cores
//! 1..=N (main thread pins to core 0). Sockets are partitioned across threads
//! round-robin. If unset, uses a single unpinned I/O thread.

use std::io;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::task::{Context, Poll};

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
        let networking_cores = std::env::var("HYDRO_NETWORKING_CORES")
            .ok()
            .map(|v| v.parse::<usize>().expect("HYDRO_NETWORKING_CORES must be a number"));
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
pub struct ChannelStream {
    rx: mpsc::UnboundedReceiver<Result<BytesMut, io::Error>>,
}

impl Stream for ChannelStream {
    type Item = Result<BytesMut, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

impl Unpin for ChannelStream {}

/// Offload the read half of a `DynStreamSink` to an I/O thread.
pub fn offload_source(mut source: DynStreamSink) -> ChannelStream {
    let (tx, rx) = mpsc::unbounded_channel();
    next_handle().spawn(async move {
        while let Some(item) = source.next().await {
            if tx.send(item).is_err() {
                break;
            }
        }
    });
    ChannelStream { rx }
}

// ─── Sink (write) side ───────────────────────────────────────────────────────

/// A `Sink` backed by an in-memory channel. The actual socket writes happen
/// on an I/O thread which drains items from this channel.
pub struct ChannelSink {
    tx: mpsc::UnboundedSender<Bytes>,
}

impl Sink<Bytes> for ChannelSink {
    type Error = io::Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        self.tx
            .send(item)
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "I/O thread gone"))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

/// Offload the write half of a `DynStreamSink` to an I/O thread.
/// Batches writes: drains all available items before flushing once.
pub fn offload_sink(mut sink: DynStreamSink) -> ChannelSink {
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
    ChannelSink { tx }
}
