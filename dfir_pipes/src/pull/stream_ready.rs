//! [`StreamReady`] - a non-blocking `Pull` that wraps a `Stream`.

use core::pin::Pin;
use core::task::{Poll, Waker};

use futures_core::stream::Stream;
use pin_project_lite::pin_project;

use crate::pull::{Pull, PullStep};
use crate::{No, Yes};

pin_project! {
    /// A `Pull` implementation that wraps a `Stream` and a `Waker`.
    ///
    /// Converts a `Stream` into a non-blocking `Pull` by polling with the provided waker.
    /// If the stream returns `Pending`, this pull treats it as ended (non-blocking).
    ///
    /// `batch_limit` caps how many items are yielded per pull batch. Use `usize::MAX` for
    /// unlimited. When the limit is hit, the waker is triggered so remaining data is
    /// processed on the next tick.
    #[must_use = "`Pull`s do nothing unless polled"]
    #[derive(Clone, Debug)]
    pub struct StreamReady<S> {
        #[pin]
        stream: S,
        waker: Waker,
        batch_limit: usize,
        pulled_count: usize,
    }
}

impl<S> StreamReady<S>
where
    Self: Pull,
{
    /// Create a new `StreamReady` from the given stream, waker, and batch limit.
    /// Use `usize::MAX` for unlimited.
    pub(crate) const fn new(stream: S, waker: Waker, batch_limit: usize) -> Self {
        Self { stream, waker, batch_limit, pulled_count: 0 }
    }
}

/// StreamReady uses its own waker, so it ignores the context parameter.
/// It implements `Pull` with `Ctx = ()`.
impl<S> Pull for StreamReady<S>
where
    S: Stream,
{
    type Ctx<'ctx> = ();

    type Item = S::Item;
    type Meta = ();
    type CanPend = No;
    type CanEnd = Yes;

    fn pull(
        self: Pin<&mut Self>,
        _ctx: &mut Self::Ctx<'_>,
    ) -> PullStep<Self::Item, Self::Meta, Self::CanPend, Self::CanEnd> {
        let this = self.project();
        if *this.pulled_count >= *this.batch_limit {
            *this.pulled_count = 0;
            this.waker.wake_by_ref();
            return PullStep::Ended(Yes);
        }
        let mut cx = core::task::Context::from_waker(this.waker);
        match this.stream.poll_next(&mut cx) {
            Poll::Ready(Some(item)) => {
                *this.pulled_count += 1;
                PullStep::Ready(item, ())
            }
            Poll::Ready(None) | Poll::Pending => PullStep::Ended(Yes),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
