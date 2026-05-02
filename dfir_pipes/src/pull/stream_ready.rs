//! [`StreamReady`] - a non-blocking `Pull` that wraps a `Stream`.

use core::pin::Pin;
use core::task::{Poll, Waker};

use futures_core::stream::Stream;
use pin_project_lite::pin_project;

use crate::pull::{Pull, PullStep};
use crate::{No, Yes};

/// Maximum number of messages to pull from a stream per tick.
/// Adjust this to control how many network messages are consumed at once.
pub const MAX_MESSAGES_PER_PULL_BATCH: usize = 1024;

pin_project! {
    /// A `Pull` implementation that wraps a `Stream` and a `Waker`.
    ///
    /// Converts a `Stream` into a non-blocking `Pull` by polling with the provided waker.
    /// If the stream returns `Pending`, this pull treats it as ended (non-blocking).
    #[must_use = "`Pull`s do nothing unless polled"]
    #[derive(Clone, Debug)]
    pub struct StreamReady<S> {
        #[pin]
        stream: S,
        waker: Waker,
        pulled_count: usize,
    }
}

impl<S> StreamReady<S>
where
    Self: Pull,
{
    /// Create a new `StreamReady` from the given stream and waker function.
    pub(crate) const fn new(stream: S, waker: Waker) -> Self {
        Self { stream, waker, pulled_count: 0 }
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
        if *this.pulled_count >= MAX_MESSAGES_PER_PULL_BATCH {
            *this.pulled_count = 0;
            // Wake so the runtime schedules another tick for remaining data.
            this.waker.wake_by_ref();
            return PullStep::Ended(Yes);
        }
        let mut cx = core::task::Context::from_waker(this.waker);
        match this.stream.poll_next(&mut cx) {
            Poll::Ready(Some(item)) => {
                *this.pulled_count += 1;
                PullStep::Ready(item, ())
            }
            Poll::Ready(None) => {
                *this.pulled_count = 0;
                PullStep::Ended(Yes)
            }
            Poll::Pending => {
                *this.pulled_count = 0;
                PullStep::Ended(Yes)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
