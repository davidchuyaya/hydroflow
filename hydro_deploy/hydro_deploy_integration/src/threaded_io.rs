//! Threaded I/O: moves socket read/write to a dedicated thread so the tick
//! thread only touches in-memory channels (no syscalls).

use std::io;
use std::pin::Pin;
use std::sync::OnceLock;
use std::task::{Context, Poll};

use bytes::{Bytes, BytesMut};
use futures::{Sink, SinkExt, Stream, StreamExt};
use tokio::sync::mpsc;

use crate::DynStreamSink;

/// A shared tokio runtime running on a dedicated OS thread for network I/O.
static IO_RUNTIME: OnceLock<tokio::runtime::Handle> = OnceLock::new();

fn io_runtime() -> &'static tokio::runtime::Handle {
    IO_RUNTIME.get_or_init(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create I/O runtime");
        let handle = rt.handle().clone();
        std::thread::Builder::new()
            .name("hydro-io".into())
            .spawn(move || {
                // Pin I/O thread to core 1 so it doesn't compete with the tick
                // thread (typically pinned to core 0).
                #[cfg(target_os = "linux")]
                unsafe {
                    let mut cpuset: libc::cpu_set_t = std::mem::zeroed();
                    libc::CPU_SET(1, &mut cpuset);
                    libc::sched_setaffinity(0, size_of_val(&cpuset), &cpuset);
                }
                rt.block_on(futures::future::pending::<()>())
            })
            .expect("failed to spawn I/O thread");
        handle
    })
}

// ─── Source (read) side ──────────────────────────────────────────────────────

/// A `Stream` backed by an in-memory channel. The actual socket reads happen
/// on the I/O thread which feeds items into this channel.
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

/// Offload the read half of a `DynStreamSink` to the I/O thread.
pub fn offload_source(mut source: DynStreamSink) -> ChannelStream {
    let (tx, rx) = mpsc::unbounded_channel();
    io_runtime().spawn(async move {
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
/// on the I/O thread which drains items from this channel.
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

/// Offload the write half of a `DynStreamSink` to the I/O thread.
/// Batches writes: drains all available items before flushing once.
pub fn offload_sink(mut sink: DynStreamSink) -> ChannelSink {
    let (tx, mut rx) = mpsc::unbounded_channel::<Bytes>();
    io_runtime().spawn(async move {
        while let Some(item) = rx.recv().await {
            if sink.feed(item).await.is_err() {
                break;
            }
            // Drain all remaining buffered items without waiting.
            while let Ok(item) = rx.try_recv() {
                if sink.feed(item).await.is_err() {
                    return;
                }
            }
            // Single flush for the entire batch.
            if sink.flush().await.is_err() {
                break;
            }
        }
    });
    ChannelSink { tx }
}
