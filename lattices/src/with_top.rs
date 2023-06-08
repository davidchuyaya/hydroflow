use std::cmp::Ordering;
use std::cmp::Ordering::*;

use super::{ConvertFrom, Merge};
use crate::LatticeOrd;

/// Wraps a lattice in [`Option`], treating [`None`] as a new top element which compares as greater
/// than to all other values.
///
/// This can be used for giving a sensible top element to lattices that don't
/// necessarily have one. Can be used to implement 'tombstones'
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithTop<Inner>(pub Option<Inner>);
impl<Inner> WithTop<Inner> {
    /// Create a new `WithTop` lattice instance from a value.
    pub fn new(val: Option<Inner>) -> Self {
        Self(val)
    }

    /// Create a new `WithTop` lattice instance from a value using `Into`.
    pub fn new_from(val: impl Into<Option<Inner>>) -> Self {
        Self::new(val.into())
    }
}

// Cannot auto derive because the generated implementation has the wrong trait bounds.
// https://github.com/rust-lang/rust/issues/26925
impl<Inner> Default for WithTop<Inner> {
    fn default() -> Self {
        Self(None)
    }
}

impl<Inner, Other> Merge<WithTop<Other>> for WithTop<Inner>
where
    Inner: Merge<Other> + ConvertFrom<Other>,
{
    fn merge(&mut self, other: WithTop<Other>) -> bool {
        match (&mut self.0, other.0) {
            (None, None) => false,
            (this @ Some(_), None) => {
                *this = None;
                true
            }
            (None, Some(_)) => false,
            (Some(self_inner), Some(other_inner)) => self_inner.merge(other_inner),
        }
    }
}

impl<Inner, Other> ConvertFrom<WithTop<Other>> for WithTop<Inner>
where
    Inner: ConvertFrom<Other>,
{
    fn from(other: WithTop<Other>) -> Self {
        Self(other.0.map(Inner::from))
    }
}

impl<Inner, Other> PartialOrd<WithTop<Other>> for WithTop<Inner>
where
    Inner: PartialOrd<Other>,
{
    fn partial_cmp(&self, other: &WithTop<Other>) -> Option<Ordering> {
        match (&self.0, &other.0) {
            (None, None) => Some(Equal),
            (None, Some(_)) => Some(Greater),
            (Some(_), None) => Some(Less),
            (Some(this_inner), Some(other_inner)) => this_inner.partial_cmp(other_inner),
        }
    }
}
impl<Inner, Other> LatticeOrd<WithTop<Other>> for WithTop<Inner> where
    Self: PartialOrd<WithTop<Other>>
{
}

impl<Inner, Other> PartialEq<WithTop<Other>> for WithTop<Inner>
where
    Inner: PartialEq<Other>,
{
    fn eq(&self, other: &WithTop<Other>) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(this_inner), Some(other_inner)) => this_inner == other_inner,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set_union::{SetUnionHashSet, SetUnionSingletonSet};
    use crate::test::check_all;

    #[test]
    fn test_singly_nested_singleton_example() {
        let mut my_hash_set = WithTop::new_from(SetUnionHashSet::<&str>::default());
        let my_delta_set = WithTop::new_from(SetUnionSingletonSet::new_from("hello world"));

        assert!(my_hash_set.merge(my_delta_set)); // Changes
        assert!(!my_hash_set.merge(my_delta_set)); // No changes
    }

    #[test]
    fn test_doubly_nested_singleton_example() {
        let mut my_hash_set =
            WithTop::new_from(WithTop::new_from(SetUnionHashSet::<&str>::default()));
        let my_delta_set = WithTop::new_from(WithTop::new_from(SetUnionSingletonSet::new_from(
            "hello world",
        )));

        assert!(my_hash_set.merge(my_delta_set)); // Changes
        assert!(!my_hash_set.merge(my_delta_set)); // No changes
    }

    #[test]
    #[rustfmt::skip]
    fn auto_derives() {
        type B = WithTop<SetUnionHashSet<usize>>;

        assert_eq!(B::default().partial_cmp(&B::default()), Some(Equal));
        assert_eq!(B::new_from(SetUnionHashSet::new_from([])).partial_cmp(&B::default()), Some(Less));
        assert_eq!(B::default().partial_cmp(&B::new_from(SetUnionHashSet::new_from([]))), Some(Greater));
        assert_eq!(B::new_from(SetUnionHashSet::new_from([])).partial_cmp(&B::new_from(SetUnionHashSet::new_from([]))), Some(Equal));
        assert_eq!(B::new_from(SetUnionHashSet::new_from([0])).partial_cmp(&B::new_from(SetUnionHashSet::new_from([]))), Some(Greater));
        assert_eq!(B::new_from(SetUnionHashSet::new_from([])).partial_cmp(&B::new_from(SetUnionHashSet::new_from([0]))), Some(Less));
        assert_eq!(B::new_from(SetUnionHashSet::new_from([0])).partial_cmp(&B::new_from(SetUnionHashSet::new_from([1]))), None);

        assert!(B::default().eq(&B::default()));
        assert!(!B::new_from(SetUnionHashSet::new_from([])).eq(&B::default()));
        assert!(!B::default().eq(&B::new_from(SetUnionHashSet::new_from([]))));
        assert!(B::new_from(SetUnionHashSet::new_from([])).eq(&B::new_from(SetUnionHashSet::new_from([]))));
        assert!(!B::new_from(SetUnionHashSet::new_from([0])).eq(&B::new_from(SetUnionHashSet::new_from([]))));
        assert!(!B::new_from(SetUnionHashSet::new_from([])).eq(&B::new_from(SetUnionHashSet::new_from([0]))));
        assert!(!B::new_from(SetUnionHashSet::new_from([0])).eq(&B::new_from(SetUnionHashSet::new_from([1]))));
    }

    #[test]
    fn consistency() {
        check_all(&[
            WithTop::default(),
            WithTop::new_from(SetUnionHashSet::new_from([])),
            WithTop::new_from(SetUnionHashSet::new_from([0])),
            WithTop::new_from(SetUnionHashSet::new_from([1])),
            WithTop::new_from(SetUnionHashSet::new_from([0, 1])),
        ])
    }
}
