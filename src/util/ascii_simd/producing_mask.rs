use std::simd::{mask8x8, SimdPartialEq, SimdPartialOrd, u8x8};
use crate::util::ascii_simd::s;

pub trait AsciiUtilsProducingMask where Self: Sized {

    fn in_range<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> mask8x8;

    fn ascii_alpha(&self) -> mask8x8;
    fn ascii_alpha_lower(&self) -> mask8x8;
    fn ascii_alpha_upper(&self) -> mask8x8;
    fn ascii_hex_digit(&self) -> mask8x8;
    fn ascii_digit(&self) -> mask8x8;

    fn ascii_sequence<
        const seq_0: u8,
        const seq_1: u8,
        const seq_2: u8,
        const seq_3: u8,
        const seq_4: u8,
        const seq_5: u8,
        const seq_6: u8,
        const seq_7: u8>(&self) -> mask8x8;
}

impl AsciiUtilsProducingMask for u8x8 {
    #[inline]
    fn in_range<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> mask8x8 {
        self.simd_ge(s(MIN))
            .simd_eq(self.simd_le(s(MAX_INCLUSIVE)))
    }

    fn ascii_alpha(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_alpha_lower(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_alpha_upper(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_hex_digit(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_digit(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_sequence<const seq_0: u8, const seq_1: u8, const seq_2: u8, const seq_3: u8, const seq_4: u8, const seq_5: u8, const seq_6: u8, const seq_7: u8>(&self) -> mask8x8 {
        todo!()
    }
}

impl AsciiUtilsProducingMask for mask8x8 {
    fn in_range<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_alpha(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_alpha_lower(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_alpha_upper(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_hex_digit(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_digit(&self) -> mask8x8 {
        todo!()
    }

    fn ascii_sequence<const seq_0: u8, const seq_1: u8, const seq_2: u8, const seq_3: u8, const seq_4: u8, const seq_5: u8, const seq_6: u8, const seq_7: u8>(&self) -> mask8x8 {
        todo!()
    }
}
