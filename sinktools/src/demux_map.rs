//! [`DemuxMap`] and related items.
use core::fmt::Debug;
use core::hash::Hash;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::HashMap;

use crate::{Sink, ready_both};

static SINK_TICKS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static SINK_ACTIVE_SUM: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Sink which receives keys paired with items `(Key, Item)`, and pushes to the corresponding output sink in a [`HashMap`] of sinks.
pub struct DemuxMap<Key, Si> {
    sinks: HashMap<Key, Si>,
    /// Maps each key to a stable bit index for tracking
        key_indices: HashMap<Key, u8>,
    /// Bitmask of sinks written to this tick
        active_mask: u64,
}

impl<Key, Si> DemuxMap<Key, Si> {
    /// Create with the given next `sinks` map.
    pub fn new<Item>(sinks: impl Into<HashMap<Key, Si>>) -> Self
    where
        Key: Eq + Hash + Clone,
        Self: Sink<(Key, Item)>,
    {
        let sinks = sinks.into();
                let key_indices: HashMap<Key, u8> = sinks
            .keys()
            .enumerate()
            .map(|(i, k)| (k.clone(), i as u8))
            .collect();
        Self {
            sinks,
                        key_indices,
                        active_mask: 0,
        }
    }
}

impl<Key, Si, Item> Sink<(Key, Item)> for DemuxMap<Key, Si>
where
    Key: Eq + Hash + Clone + Debug + Unpin,
    Si: Sink<Item> + Unpin,
{
    type Error = Si::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        #[expect(
            clippy::disallowed_methods,
            reason = "nondeterministic iteration order, the `try_fold` is not order-dependent"
        )]
        self.get_mut()
            .sinks
            .values_mut()
            .try_fold(Poll::Ready(()), |poll, sink| {
                ready_both!(poll, Pin::new(sink).poll_ready(cx)?);
                Poll::Ready(Ok(()))
            })
    }

    fn start_send(self: Pin<&mut Self>, item: (Key, Item)) -> Result<(), Self::Error> {
        let me = self.get_mut();
        let sink = me
            .sinks
            .get_mut(&item.0)
            .unwrap_or_else(|| panic!("`DemuxMap` missing key {:?}", item.0));
                if let Some(&idx) = me.key_indices.get(&item.0) {
            me.active_mask |= 1u64 << idx;
        }
        Pin::new(sink).start_send(item.1)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let me = self.get_mut();
                if me.active_mask != 0 {
            let active = me.active_mask.count_ones() as u64;
            SINK_ACTIVE_SUM.fetch_add(active, std::sync::atomic::Ordering::Relaxed);
            let ticks = SINK_TICKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            me.active_mask = 0;

            if ticks % 200000 == 0 {
                let total_active = SINK_ACTIVE_SUM.load(std::sync::atomic::Ordering::Relaxed);
                let avg = total_active as f64 / ticks as f64;
                println!("HYDRO_SINK_STATS: avg_active_per_tick={:.2} ticks={}", avg, ticks);
            }
        }
        #[expect(
            clippy::disallowed_methods,
            reason = "nondeterministic iteration order, the `try_fold` is not order-dependent"
        )]
        me.sinks
            .values_mut()
            .try_fold(Poll::Ready(()), |poll, sink| {
                ready_both!(poll, Pin::new(sink).poll_flush(cx)?);
                Poll::Ready(Ok(()))
            })
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        #[expect(
            clippy::disallowed_methods,
            reason = "nondeterministic iteration order, the `try_fold` is not order-dependent"
        )]
        self.get_mut()
            .sinks
            .values_mut()
            .try_fold(Poll::Ready(()), |poll, sink| {
                ready_both!(poll, Pin::new(sink).poll_close(cx)?);
                Poll::Ready(Ok(()))
            })
    }
}

/// Creates a `DemuxMap` sink that sends each item to one of many outputs, depending on the key.
///
/// This requires sinks `Si` to be `Unpin`. If your sinks are not `Unpin`, first wrap them in `Box::pin` to make them `Unpin`.
pub fn demux_map<Key, Si, Item>(sinks: impl Into<HashMap<Key, Si>>) -> DemuxMap<Key, Si>
where
    Key: Eq + Hash + Clone + Debug + Unpin,
    Si: Sink<Item> + Unpin,
{
    DemuxMap::new(sinks)
}
