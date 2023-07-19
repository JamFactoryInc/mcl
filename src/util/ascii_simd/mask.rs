use std::simd::{Mask, mask8x8, SimdPartialEq, ToBitMask, u8x8};
use crate::util::ascii_simd::{m, s};

pub trait MaskAsciiUtils where Self: Sized {

    /// counts the matches (0-8) left-to-right for the given filter chain
    ///
    /// stops once it reaches a non-match
    fn count_matches(&self) -> usize;
}

impl MaskAsciiUtils for mask8x8 {
    fn count_matches(&self) -> usize {
        self.to_bitmask().leading_ones() as usize
    }
}