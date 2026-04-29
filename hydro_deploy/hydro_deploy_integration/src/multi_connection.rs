use std::collections::{HashMap, HashSet};
use std::io;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{Sink, SinkExt, Stream, StreamExt};
#[cfg(unix)]
use tempfile::TempDir;
use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::codec::{Decoder, Encoder, Framed, FramedRead, FramedWrite};

use crate::{AcceptedServer, BoundServer, Connected, Connection};

pub struct ConnectedMultiConnection<I, O, C: Decoder<Item = I> + Encoder<O>> {
    pub source: MultiConnectionSource<I, O, C>,
    pub sink: MultiConnectionSink<O, C>,
    pub membership: UnboundedReceiverStream<(u64, bool)>,
}

impl<
    I: Send + 'static,
    O: Send + Sync + 'static,
    C: Decoder<Item = I> + Encoder<O> + Send + Sync + Default + 'static,
> Connected for ConnectedMultiConnection<I, O, C>
{
    fn from_defn(pipe: Connection) -> Self {
        match pipe {
            Connection::AsServer(AcceptedServer::MultiConnection(bound_server)) => {
                let (new_sink_sender, new_sink_receiver) = mpsc::unbounded_channel();
                let (membership_sender, membership_receiver) = mpsc::unbounded_channel();

                let source = match *bound_server {
                    #[cfg(unix)]
                    BoundServer::UnixSocket(listener, dir) => {
                        let (item_sender, item_receiver) = mpsc::unbounded_channel();
                        MultiConnectionSource {
                            unix_listener: Some(listener),
                            tcp_listener: None,
                            _dir_holder: Some(dir),
                            next_connection_id: 0,
                            item_sender,
                            item_receiver,
                            new_sink_sender,
                            membership_sender,
                        }
                    }
                    BoundServer::TcpPort(listener, _) => {
                        let (item_sender, item_receiver) = mpsc::unbounded_channel();
                        MultiConnectionSource {
                            #[cfg(unix)]
                            unix_listener: None,
                            tcp_listener: Some(listener.into_inner()),
                            #[cfg(unix)]
                            _dir_holder: None,
                            next_connection_id: 0,
                            item_sender,
                            item_receiver,
                            new_sink_sender,
                            membership_sender,
                        }
                    }
                    _ => panic!("MultiConnection only supports UnixSocket and TcpPort"),
                };

                let sink = MultiConnectionSink::<O, C> {
                    connection_sinks: HashMap::new(),
                    new_sink_receiver,
                    dirty_ids: HashSet::new(),
                };

                ConnectedMultiConnection {
                    source,
                    sink,
                    membership: UnboundedReceiverStream::new(membership_receiver),
                }
            }
            _ => panic!("Cannot connect to a non-multi-connection pipe as a multi-connection"),
        }
    }
}

type DynDecodedStream<I, C> =
    Pin<Box<dyn Stream<Item = Result<I, <C as Decoder>::Error>> + Send + Sync>>;
type DynEncodedSink<O, C> = Pin<Box<dyn Sink<O, Error = <C as Encoder<O>>::Error> + Send + Sync>>;

pub struct MultiConnectionSource<I, O, C: Decoder<Item = I> + Encoder<O>> {
    #[cfg(unix)]
    unix_listener: Option<UnixListener>,
    tcp_listener: Option<TcpListener>,
    #[cfg(unix)]
    _dir_holder: Option<TempDir>, // keeps the folder containing the socket alive
    next_connection_id: u64,
    /// Shared channel that all per-connection tasks feed into.
    item_sender: mpsc::UnboundedSender<(u64, I)>,
    item_receiver: mpsc::UnboundedReceiver<(u64, I)>,
    new_sink_sender: mpsc::UnboundedSender<(u64, DynEncodedSink<O, C>)>,
    membership_sender: mpsc::UnboundedSender<(u64, bool)>,
}

pub struct MultiConnectionSink<O, C: Encoder<O>> {
    connection_sinks: HashMap<u64, DynEncodedSink<O, C>>,
    new_sink_receiver: mpsc::UnboundedReceiver<(u64, DynEncodedSink<O, C>)>,
    /// Connection IDs that have been written to since the last flush.
    dirty_ids: HashSet<u64>,
}

impl<
    I: Send + 'static,
    O: Send + Sync + 'static,
    C: Decoder<Item = I> + Encoder<O> + Send + Sync + Default + 'static,
> Stream for MultiConnectionSource<I, O, C>
where
    <C as Decoder>::Error: Send,
{
    type Item = Result<(u64, I), <C as Decoder>::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let me = self.deref_mut();

        // Helper: spawn a per-connection reader task
        let spawn_reader = |connection_id: u64,
                            mut stream: DynDecodedStream<I, C>,
                            item_sender: mpsc::UnboundedSender<(u64, I)>,
                            membership_sender: mpsc::UnboundedSender<(u64, bool)>| {
            tokio::spawn(async move {
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(data) => {
                            if item_sender.send((connection_id, data)).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                let _ = membership_sender.send((connection_id, false));
            });
        };

        // Handle Unix socket accepts
        #[cfg(unix)]
        if let Some(listener) = me.unix_listener.as_mut() {
            loop {
                match listener.poll_accept(cx) {
                    Poll::Ready(Ok((stream, _))) => {
                        let connection_id = me.next_connection_id;
                        me.next_connection_id += 1;

                        let framed = Framed::new(stream, C::default());
                        let (sink, stream) = framed.split();

                        let boxed_stream: DynDecodedStream<I, C> = Box::pin(stream);
                        let boxed_sink: DynEncodedSink<O, C> = Box::pin(sink.buffer(1024));

                        spawn_reader(
                            connection_id,
                            boxed_stream,
                            me.item_sender.clone(),
                            me.membership_sender.clone(),
                        );

                        let _ = me.new_sink_sender.send((connection_id, boxed_sink));
                        let _ = me.membership_sender.send((connection_id, true));
                    }
                    Poll::Ready(Err(e)) => {
                        // If no tasks are running, propagate the error
                        if me.item_sender.is_closed() {
                            return Poll::Ready(Some(Err(e.into())));
                        }
                        break;
                    }
                    Poll::Pending => break,
                }
            }
        }

        // Handle TCP socket accepts
        if let Some(listener) = me.tcp_listener.as_mut() {
            loop {
                match listener.poll_accept(cx) {
                    Poll::Ready(Ok((stream, _))) => {
                        let connection_id = me.next_connection_id;
                        me.next_connection_id += 1;

                        let framed = Framed::new(stream, C::default());
                        let (sink, stream) = framed.split();

                        let boxed_stream: DynDecodedStream<I, C> = Box::pin(stream);
                        let boxed_sink: DynEncodedSink<O, C> = Box::pin(sink.buffer(1024));

                        spawn_reader(
                            connection_id,
                            boxed_stream,
                            me.item_sender.clone(),
                            me.membership_sender.clone(),
                        );

                        let _ = me.new_sink_sender.send((connection_id, boxed_sink));
                        let _ = me.membership_sender.send((connection_id, true));
                    }
                    Poll::Ready(Err(e)) => {
                        if me.item_sender.is_closed() {
                            return Poll::Ready(Some(Err(e.into())));
                        }
                        break;
                    }
                    Poll::Pending => break,
                }
            }
        }

        // Poll the shared channel for items from any connection
        me.item_receiver.poll_recv(cx).map(|opt| opt.map(Ok))
    }
}

impl<O, C: Encoder<O>> Sink<(u64, O)> for MultiConnectionSink<O, C> {
    type Error = <C as Encoder<O>>::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let me = self.get_mut();
        loop {
            match me.new_sink_receiver.poll_recv(cx) {
                Poll::Ready(Some((connection_id, sink))) => {
                    me.connection_sinks.insert(connection_id, sink);
                }
                Poll::Ready(None) => {
                    if me.connection_sinks.is_empty() {
                        return Poll::Ready(Err(io::Error::new(
                            io::ErrorKind::BrokenPipe,
                            "No additional sinks are available (was the stream dropped)?",
                        )
                        .into()));
                    } else {
                        break;
                    }
                }
                Poll::Pending => {
                    break;
                }
            }
        }

        // Only check readiness of dirty sinks
        let mut any_pending = false;
        me.dirty_ids.retain(|id| {
            if let Some(sink) = me.connection_sinks.get_mut(id) {
                match sink.as_mut().poll_ready(cx) {
                    Poll::Ready(Ok(())) => true,
                    Poll::Ready(Err(_)) => false,
                    Poll::Pending => {
                        any_pending = true;
                        true
                    }
                }
            } else {
                false
            }
        });

        if any_pending {
            Poll::Pending
        } else {
            Poll::Ready(Ok(())) // always ready, because we drop messages if there is no sink
        }
    }

    fn start_send(self: Pin<&mut Self>, item: (u64, O)) -> Result<(), Self::Error> {
        let me = self.get_mut();
        if let Some(sink) = me.connection_sinks.get_mut(&item.0) {
            me.dirty_ids.insert(item.0);
            let _ = sink.as_mut().start_send(item.1);
        }
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let me = self.get_mut();
        let mut any_pending = false;

        me.dirty_ids.retain(|id| {
            if let Some(sink) = me.connection_sinks.get_mut(id) {
                match sink.as_mut().poll_flush(cx) {
                    Poll::Ready(Ok(())) => false,
                    Poll::Ready(Err(_)) => false,
                    Poll::Pending => {
                        any_pending = true;
                        true
                    }
                }
            } else {
                false
            }
        });

        if any_pending {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut any_pending = false;

        self.get_mut().connection_sinks.retain(|_, sink| {
            match sink.as_mut().poll_close(cx) {
                Poll::Ready(Ok(()) | Err(_)) => false,
                Poll::Pending => {
                    any_pending = true;
                    true
                }
            }
        });

        if any_pending {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

/// TCP-only concrete type versions for use in containerized deployments
pub struct TcpMultiConnectionSource<C: Decoder> {
    /// The TCP listener accepting new connections
    pub listener: TcpListener,
    /// Counter for assigning unique connection IDs
    pub next_connection_id: u64,
    /// Shared channel that all per-connection tasks feed into.
    pub item_sender: mpsc::UnboundedSender<(u64, C::Item)>,
    pub item_receiver: mpsc::UnboundedReceiver<(u64, C::Item)>,
    /// Channel to send new sinks to the TcpMultiConnectionSink
    pub new_sink_sender: mpsc::UnboundedSender<(u64, FramedWrite<OwnedWriteHalf, C>)>,
    /// Channel to send membership events
    pub membership_sender: mpsc::UnboundedSender<(u64, bool)>,
}

impl<C: Decoder + Default + Send + Unpin + 'static> Stream for TcpMultiConnectionSource<C>
where
    C::Item: Send,
    C::Error: From<io::Error> + Send,
{
    type Item = Result<(u64, C::Item), C::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let me = self.deref_mut();

        // Accept new connections and spawn a reader task for each
        loop {
            match me.listener.poll_accept(cx) {
                Poll::Ready(Ok((stream, _peer))) => {
                    let connection_id = me.next_connection_id;
                    me.next_connection_id += 1;

                    let (rx, tx) = stream.into_split();
                    let fr = FramedRead::new(rx, C::default());
                    let fw = FramedWrite::new(tx, C::default());

                    let item_sender = me.item_sender.clone();
                    let membership_sender = me.membership_sender.clone();
                    tokio::spawn(async move {
                        let mut fr = fr;
                        while let Some(result) = fr.next().await {
                            match result {
                                Ok(data) => {
                                    if item_sender.send((connection_id, data)).is_err() {
                                        break;
                                    }
                                }
                                Err(_) => break,
                            }
                        }
                        let _ = membership_sender.send((connection_id, false));
                    });

                    let _ = me.new_sink_sender.send((connection_id, fw));
                    let _ = me.membership_sender.send((connection_id, true));
                }
                Poll::Ready(Err(e)) => {
                    if me.item_sender.is_closed() {
                        return Poll::Ready(Some(Err(e.into())));
                    }
                    break;
                }
                Poll::Pending => break,
            }
        }

        // Poll the shared channel for items from any connection
        me.item_receiver.poll_recv(cx).map(|opt| opt.map(Ok))
    }
}

/// TCP-only multi-connection sink using concrete types (no boxing).
/// Routes (connection_id, data) to the appropriate connection.
pub struct TcpMultiConnectionSink<I, C: Encoder<I>> {
    /// Map of connection IDs to their framed writers
    pub connection_sinks: HashMap<u64, FramedWrite<OwnedWriteHalf, C>>,
    /// Channel to receive new sinks from TcpMultiConnectionSource
    pub new_sink_receiver: mpsc::UnboundedReceiver<(u64, FramedWrite<OwnedWriteHalf, C>)>,
    /// Connection IDs that have been written to since the last flush.
    dirty_ids: HashSet<u64>,
    _marker: std::marker::PhantomData<fn(I) -> I>, /* fn(I) -> I instead of just I to keep the struct invariant over I, which keeps it Unpin. */
}

impl<I, C: Encoder<I> + Unpin> Sink<(u64, I)> for TcpMultiConnectionSink<I, C> {
    type Error = C::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let me = self.get_mut();
        // Receive any new sinks
        while let Poll::Ready(Some((id, sink))) = me.new_sink_receiver.poll_recv(cx) {
            me.connection_sinks.insert(id, sink);
        }
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: (u64, I)) -> Result<(), Self::Error> {
        let me = self.get_mut();
        if let Some(sink) = me.connection_sinks.get_mut(&item.0) {
            me.dirty_ids.insert(item.0);
            let _ = Pin::new(sink).start_send(item.1);
        }
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let me = self.get_mut();
        let mut any_pending = false;

        me.dirty_ids.retain(|id| {
            if let Some(sink) = me.connection_sinks.get_mut(id) {
                match Pin::new(sink).poll_flush(cx) {
                    Poll::Ready(Ok(())) => false,
                    Poll::Ready(Err(_)) => false,
                    Poll::Pending => {
                        any_pending = true;
                        true
                    }
                }
            } else {
                false
            }
        });

        if any_pending {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let me = self.get_mut();
        let mut any_pending = false;

        me.connection_sinks
            .retain(|_id, sink| match Pin::new(sink).poll_close(cx) {
                Poll::Ready(Ok(())) => false,
                Poll::Ready(Err(_)) => false,
                Poll::Pending => {
                    any_pending = true;
                    true
                }
            });

        if any_pending {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

type TcpMultiConnectionParts<I, C> = (
    TcpMultiConnectionSource<C>,
    TcpMultiConnectionSink<I, C>,
    UnboundedReceiverStream<(u64, bool)>,
);

pub fn tcp_multi_connection<I, C>(listener: TcpListener) -> TcpMultiConnectionParts<I, C>
where
    C: Decoder + Encoder<I> + Default,
{
    let (new_sink_sender, new_sink_receiver) = mpsc::unbounded_channel();
    let (membership_sender, membership_receiver) = mpsc::unbounded_channel();
    let (item_sender, item_receiver) = mpsc::unbounded_channel();

    let source = TcpMultiConnectionSource {
        listener,
        next_connection_id: 0,
        item_sender,
        item_receiver,
        new_sink_sender,
        membership_sender,
    };

    let sink = TcpMultiConnectionSink {
        connection_sinks: HashMap::new(),
        new_sink_receiver,
        dirty_ids: HashSet::new(),
        _marker: std::marker::PhantomData,
    };

    let membership = UnboundedReceiverStream::new(membership_receiver);

    (source, sink, membership)
}
