use std::simd::{ToBitMask};
use crate::parse::StdMask;

pub trait MaskAsciiUtils where Self: Sized {

    /// counts the matches (0-8) left-to-right for the given filter chain
    ///
    /// stops once it reaches a non-match
    fn count_matches<T : From<u8>>(&self) -> T;

    /// counts the misses (0-8) left-to-right for the given filter chain
    ///
    /// stops once it reaches a match
    fn count_misses<T : From<u8>>(&self) -> T;
}

impl MaskAsciiUtils for StdMask {
    #[inline]
    fn count_matches<T : From<u8>>(&self) -> T {
        self.to_bitmask().leading_ones() as T
    }

    #[inline]
    fn count_misses<T : From<u8>>(&self) -> T {
        self.to_bitmask().leading_zeros() as T
    }
}