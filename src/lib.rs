//! `am-word` is a fast, embedded word list you can index into.
//!
//! The whole list is baked into the binary at compile time, with each word's
//! byte range computed ahead of time in `build.rs`.

mod data {
    // Genrated by build.rs
    include!(concat!(env!("OUT_DIR"), "/worddata.rs"));
}

/// How many words there are.
pub const LEN: usize = data::WORD_COUNT;

#[inline]
pub fn len() -> usize {
    LEN
}

#[inline]
pub fn is_empty() -> bool {
    LEN == 0
}

#[inline]
pub fn get(index: usize) -> Option<&'static str> {
    let (start, end) = *data::SPANS.get(index)?;
    Some(&data::BLOB[start as usize..end as usize])
}

#[inline]
pub fn iter() -> impl Iterator<Item = &'static str> {
    data::SPANS
        .iter()
        .map(|&(start, end)| &data::BLOB[start as usize..end as usize])
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
}
