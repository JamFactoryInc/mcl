use std::simd::{Mask, mask8x8, SimdPartialEq, ToBitMask, u8x8};

pub trait MaskAsciiUtils where Self: Sized {

    /// counts the matches (0-8) left-to-right for the given filter chain
    ///
    /// stops once it reaches a non-match
    fn count_matches<T : From<u8>>(&self) -> T;
}

impl MaskAsciiUtils for mask8x8 {
    #[inline]
    fn count_matches<T : From<u8>>(&self) -> T {
        self.to_bitmask().leading_ones() as T
    }
}