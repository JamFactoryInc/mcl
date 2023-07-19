use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};
use std::simd::{Mask, mask8x8, SimdPartialEq, SimdPartialOrd, SimdUint, ToBitMask, u8x8};
use crate::util::ascii_simd::producing_mask::AsciiUtilsProducingMask;
use crate::util::ascii_simd::s;

pub trait AsciiUtilsProducingSimd where Self: Sized {

    fn from<const VAL: u8>() -> u8x8 {
        u8x8::from_array([VAL; 8])
    }

    fn ascii_justify<const JUSTIFY: u8, const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> u8x8;

    fn ascii_mask_justify<const JUSTIFY: u8, const MIN: u8>(&self, mask: mask8x8) -> u8x8;

    fn ascii_justify_zero<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> u8x8;

    fn ascii_mask_justify_zero<const MIN: u8>(&self, mask: mask8x8) -> u8x8;

    fn ascii_hex_digit_parse(&self) -> (u8x8, mask8x8);
}

impl AsciiUtilsProducingSimd for u8x8 {
    /// justifies a range of ascii characters to the given index
    ///
    /// e.g. `ascii_range<0, b'a', b'z'>` behaves as:
    /// ```
    /// b'a' => 0
    /// b'j' => 9
    /// b'1' => 0
    /// ```
    /// sets outlying characters to 0
    #[inline]
    fn ascii_justify<const JUSTIFY: u8, const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> u8x8 {
        self.ascii_mask_justify::<JUSTIFY, MIN>(self.in_range::<MIN, MAX_INCLUSIVE>())
    }

    #[inline]
    fn ascii_mask_justify<const JUSTIFY: u8, const MIN: u8>(&self, mask: mask8x8) -> u8x8 {
        mask.select(self + s(JUSTIFY - MIN), s(0))
    }

    fn ascii_justify_zero<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> u8x8 {
        self.ascii_mask_justify_zero::<MIN>(self.in_range::<MIN, MAX_INCLUSIVE>())
    }

    fn ascii_mask_justify_zero<const MIN: u8>(&self, mask: mask8x8) -> u8x8 {
        mask.select(self - s(MIN), s(0))
    }

    fn ascii_hex_digit_parse(&self) -> (u8x8, mask8x8) {
        let digit_mask = self.in_range::<b'0', b'1'>();

        let alpha_mask = self.in_range::<b'a', b'f'>()
            .bitor(self.in_range::<b'A', b'F'>());

        (
            digit_mask.select(
            // if it's a number, justify to int value
            self - s(b'0'),
            alpha_mask.select(
                // if it's an alpha (a-f or A-F), mod 32 to standardize upper and lower case,
                // then add 8 to bring b'a' % 32 to 10
                self % s(32) + s(8),
                // otherwise make it zero
                s(0)
            )),
            digit_mask.bitor(alpha_mask)
        )
    }
}