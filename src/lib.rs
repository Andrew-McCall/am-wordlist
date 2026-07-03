#![no_std]
//! A fast, embedded word list you can index, with zero runtime init and zero
//! allocation.
//!
//! The whole list is baked into the binary at compile time, with each word's
//! byte range computed ahead of time in `build.rs`.

use core::ops::{Bound, RangeBounds};

mod data {
    // Genrated by build.rs
    include!(concat!(env!("OUT_DIR"), "/worddata.rs"));
}

/// How many words there are.
pub const LEN: usize = data::WORD_COUNT;

/// Resolve a `(start, end)` byte span into the word it borrows from the blob.
#[inline]
fn word(&(start, end): &(u32, u32)) -> &'static str {
    &data::BLOB[start as usize..end as usize]
}

#[inline]
pub fn len() -> usize {
    LEN
}

#[inline]
pub fn is_empty() -> bool {
    LEN == 0
}

/// Look up a single word by index. O(1), no allocation. Out of range is `None`.
#[inline]
pub fn get(index: usize) -> Option<&'static str> {
    data::SPANS.get(index).map(word)
}

/// Look up a contiguous range of words as an allocation-free iterator.
///
/// Accepts any range (`a..b`, `a..=b`, `a..`, `..b`, `..`). Returns `None` if
/// the range is out of bounds or its start is past its end — matching [`get`].
///
/// ```
/// let mut it = am_word::get_range(0..3).unwrap();
/// assert_eq!(it.clone().count(), 3);
/// assert_eq!(it.next(), am_word::get(0));
/// assert!(am_word::get_range(0..=am_word::LEN).is_none());
/// ```
#[inline]
pub fn get_range<R: RangeBounds<usize>>(
    range: R,
) -> Option<impl Iterator<Item = &'static str> + Clone> {
    let start = match range.start_bound() {
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n.checked_add(1)?,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(&n) => n.checked_add(1)?,
        Bound::Excluded(&n) => n,
        Bound::Unbounded => LEN,
    };
    Some(data::SPANS.get(start..end)?.iter().map(word))
}

/// Iterate over every word in order. O(1) to set up, no allocation.
#[inline]
pub fn iter() -> impl Iterator<Item = &'static str> + Clone {
    data::SPANS.iter().map(word)
}

#[cfg(test)]
mod tests {
    #[test]
    fn non_empty() {
        assert!(super::len() > 0);
        assert_eq!(super::len(), super::LEN);
        assert!(!super::is_empty());
    }

    #[test]
    fn bounds() {
        assert!(super::get(0).is_some());
        assert!(super::get(super::len() - 1).is_some());
        assert!(super::get(super::len()).is_none());
        assert!(super::get(usize::MAX).is_none());
    }

    #[test]
    fn iter_matches_get() {
        assert_eq!(super::iter().count(), super::len());
        for (i, w) in super::iter().enumerate() {
            assert_eq!(super::get(i), Some(w));
        }
    }

    #[test]
    fn get_range_matches_get() {
        let mut range = super::get_range(2..5).unwrap();
        assert_eq!(range.clone().count(), 3);
        assert_eq!(range.next(), super::get(2));
        assert_eq!(range.next(), super::get(3));
        assert_eq!(range.next(), super::get(4));
        assert_eq!(range.next(), None);
    }

    #[test]
    fn get_range_bounds_variants() {
        let full = super::len();
        assert_eq!(super::get_range(..).unwrap().count(), full);
        assert_eq!(super::get_range(..3).unwrap().count(), 3);
        assert_eq!(super::get_range(0..=2).unwrap().count(), 3);
        assert_eq!(super::get_range(full - 1..).unwrap().count(), 1);
        // Empty ranges are valid and yield nothing.
        assert_eq!(super::get_range(2..2).unwrap().count(), 0);
    }

    #[test]
    fn get_range_out_of_bounds() {
        let full = super::len();
        assert!(super::get_range(0..=full).is_none());
        assert!(super::get_range(full + 1..full + 2).is_none());
        // start past end is rejected, not clamped.
        assert!(super::get_range(5..2).is_none());
        assert!(super::get_range(..=usize::MAX).is_none());
    }
}
