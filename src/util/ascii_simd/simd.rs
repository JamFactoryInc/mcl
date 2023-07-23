use std::iter::Iterator;
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Rem, Sub};
use std::simd::{Simd, SimdInt, SimdPartialEq, SimdUint, ToBitMask};
use crate::parse::{SIMD_LANE_SIZE, StdMask, StdSimd};
use crate::util::ascii_simd::{ALPHA_BLOCK, ALPHA_BLOCK_SIZE, ALPHA_LENGTH, DEC_ORDERS, ZERO};
use crate::util::ascii_simd::mask::MaskAsciiUtils;


pub trait SimdUtils {
    /// justifies a range of ascii characters to the given index
    ///
    /// e.g. `justify_range<0, b'a', b'z'>` behaves as:
    /// ```
    /// b'a' => 0
    /// b'j' => 9
    /// b'1' => 0
    /// ```
    /// sets outlying characters to 0
    fn justify_range<const NEW_LEFT_OFFSET: u8, const MIN: u8, const MAX_INCLUSIVE: u8> (&self) -> StdSimd;

    fn justify<const NEW_LEFT_OFFSET: u8, const MIN: u8> (&self) -> StdSimd;

    fn justify_range_to_zero<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> StdSimd;

    fn justify_to_zero<const MIN: u8>(&self) -> StdSimd;

    fn parse_base_16(&self) -> (StdSimd, StdMask);

    fn parse_base_10(&self) -> (StdSimd, StdMask);

    fn concat_base_n<const BASE: u32>(&self, mask: StdMask) -> (u32, u32);

    /// Concatenates the contained values into an integer.
    ///
    /// Only works if each lane is 0-9
    ///
    /// e.g.
    /// ```
    /// <1, 3, 4, 6> -> 1346
    fn concat_base_10(&self, mask: StdMask) -> (u32, u32);

    /// Concatenates the contained values into an integer
    ///
    /// Only works if each lane is 0-15
    ///
    /// e.g.
    /// ```
    /// <10, 0, 10, 2> -> 41122
    /// or equivalently
    /// <0xA, 0x0, 0xA, 0x2> -> 0xA0A2
    fn concat_base_16(&self, mask: StdMask) -> (u32, u32);

    // this might be a chunky boi when compiled, so best to not inline it
    fn use_amount(&self, amount: usize) -> StdSimd;

    /// sets the right-most `n` elements of this vector to zero
    fn do_zero(&self, num: usize) -> StdSimd;

    /// sets the right-most `LANES - n` elements of this vector to zero
    fn do_keep(&self, num: usize) -> StdSimd;

    /// rotates the right-most `n` elements of this vector to be the first `n` elements,
    /// then adds the `LANES - n` left-most elements of `new_values` to the `n` right-most
    /// lanes of this vector
    ///
    /// e.g.
    /// ```
    /// <1, 2, 3, 4>.use_and_add(2, &[10, 11]) -> <3, 4, 10, 11>
    fn use_and_add(&self, used: usize, new_values: &[u8]) -> StdSimd;

    fn range<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> StdMask;

    fn alpha(&self) -> StdMask;

    fn lower(&self) -> StdMask;

    fn upper(&self) -> StdMask;

    fn hex_alpha(&self) -> StdMask;
    
    fn hex(&self) -> StdMask;
    
    fn digit(&self) -> StdMask;

    fn sequence<const chars: [u8; 8]>(&self) -> StdMask;

    fn find<const CHAR: u8>(&self) -> StdMask;

    fn index_of<const CHAR: u8>(&self) -> usize;
}
impl SimdUtils for StdSimd {
    #[inline]
    fn justify_range<const NEW_LEFT_OFFSET: u8, const MIN: u8, const MAX_INCLUSIVE: u8> (&self) -> StdSimd {
        self.add(StdSimd::splat(NEW_LEFT_OFFSET - MIN))
            .bitand(self.range::<MIN, MAX_INCLUSIVE>())
    }

    #[inline]
    fn justify<const NEW_LEFT_OFFSET: u8, const MIN: u8> (&self) -> StdSimd {
        self.add(StdSimd::splat(NEW_LEFT_OFFSET - MIN))
    }

    #[inline]
    fn justify_range_to_zero<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> StdSimd {
        self.sub(StdSimd::splat(MIN))
            .bitand(self.range::<MIN, MAX_INCLUSIVE>())
    }

    #[inline]
    fn justify_to_zero<const MIN: u8>(&self) -> StdSimd {
        self.sub(StdSimd::splat(MIN))
    }

    #[inline]
    fn parse_base_16(&self) -> (StdSimd, StdMask) {
        let digit_mask = self.digit();
        let alpha_mask = self.hex_alpha();

        (
            digit_mask.select(
                // if it's a number, justify to int value
                self.justify_to_zero::<b'0'>(),
                // if it's an alpha (a-f or A-F), mod 32 to standardize upper and lower case,
                // then add 8 to bring b'a' % 32 to 10
                // otherwise make it zero
                self.rem(ALPHA_BLOCK_SIZE)
                    .justify::<{ 0xA - 1 }, { b'a' % 32 }>()
                    .bitand(alpha_mask.to_int().cast())
            ),
            digit_mask.bitor(alpha_mask)
        )
    }

    #[inline]
    fn parse_base_10(&self) -> (StdSimd, StdMask) {
        let digit_mask = self.digit();
        (
            self.sub(ZERO).bitand(digit_mask.to_int().cast()),
            digit_mask
        )
    }

    #[inline]
    fn concat_base_n<const BASE: u32>(&self, mask: StdMask) -> (u32, u32) {
        // get the number of orders of magnitude to reduce `ORDERS` by
        let ord = BASE.pow(8 - mask.count_matches() as u32);
        (
            // upcast to i32, allowing space for adjusted values
            self.cast::<u32>()
                // reduce the orders by the given amount
                // this will also zero unwanted values
                .mul(DEC_ORDERS.div(Simd::<u32, SIMD_LANE_SIZE>::splat(ord)))
                .reduce_sum(),
            ord.clone()
        )
    }

    #[inline]
    fn concat_base_10(&self, mask: StdMask) -> (u32, u32) {
        self.concat_base_n::<10>(mask)
    }

    #[inline]
    fn concat_base_16(&self, mask: StdMask) -> (u32, u32)  {
        self.concat_base_n::<0x10>(mask)
    }

    #[inline]
    fn use_amount(&self, amount: usize) -> StdSimd {
        let arr = self.to_array();
        arr.rotate_left(amount);
        StdSimd::from_array(arr)
    }

    #[inline]
    fn do_zero(&self, num: usize) -> StdSimd {
        const ZERO_MASKS: [StdSimd; SIMD_LANE_SIZE] = [0; SIMD_LANE_SIZE].iter().enumerate().map(|(i, _)| get_multiplicand(i)).collect();
        const fn get_multiplicand(len: usize) -> StdSimd {
            let mut bitmask = 0;
            let mut i = 0;
            while i < len {
                bitmask <<= 1;
                bitmask += 1;
                i += 1;
            }
            StdMask::from_bitmask(bitmask).to_int().cast()
        }

        self.bitand(ZERO_MASKS[num])
    }

    #[inline]
    fn do_keep(&self, num: usize) -> StdSimd {
        const KEEP_MASKS: [StdSimd; SIMD_LANE_SIZE] = [0; SIMD_LANE_SIZE].iter().enumerate().map(|(i, _)| get_multiplicand(i)).collect();
        const fn get_multiplicand(len: usize) -> StdSimd {
            let mut bitmask = 0;
            let mut i = 0;
            while i < len {
                bitmask <<= 1;
                bitmask += 1;
                i += 1;
            }
            StdMask::from_bitmask(bitmask ^ u8::MAX).to_int().cast()
        }

        self.bitand(KEEP_MASKS[num])
    }

    #[inline]
    fn use_and_add(&self, used: usize, new_values: &[u8]) -> StdSimd {
        // set the last n values to the contents of the given slice
        let mut vals = [0u8; SIMD_LANE_SIZE];
        vals[(SIMD_LANE_SIZE - &used)..].copy_from_slice(new_values);
        let (self_rotated, inverse_mask) = self.use_amount(used).do_zero();

        self_rotated.add(
            StdSimd::from_array(vals)
                .bitand(inverse_mask.to_int().cast())
        )
    }

    #[inline]
    fn range<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> StdMask {
        self.self_ge(StdSimd::splat(MIN))
            .bitand(self.self_le(StdSimd::splat(MAX_INCLUSIVE)))
    }

    #[inline]
    fn alpha(self: StdSimd) -> StdMask {
        // make sure our bytes are in the alpha blocks
        self.self_ge(ALPHA_BLOCK)
            // AND
            .bitand(
                // flatten the ascii blocks to 0-31
                self.rem(ALPHA_BLOCK_SIZE)
                    // make sure the flattened values are from 1-26 (a-z)
                    .range::<1, ALPHA_LENGTH>()
            )
    }

    #[inline]
    fn lower(&self) -> StdMask {
        self.range::<b'a', b'z'>()
    }

    #[inline]
    fn upper(&self) -> StdMask {
        self.range::<b'A', b'Z'>()
    }

    #[inline]
    fn hex_alpha(&self) -> StdMask {
        // filter for the alpha blocks,
        self.self_ge(ALPHA_BLOCK)
            // make sure the flattened values are from 1-6 (a-f),
            .bitand(self.range::<1, 6>(
                // flatten the ascii blocks to 0-31
                self.rem(ALPHA_BLOCK_SIZE)
            ))
    }

    #[inline]
    fn hex(&self) -> StdMask {
        // either get the digit mask
        self.digit(self)
            // OR
            .bitor(self.hex_alpha(self))
    }

    #[inline]
    fn digit(&self) -> StdMask {
        self.range::<b'0', b'1'>(self)
    }

    #[inline]
    fn sequence<const CHARS: [u8; 8]>(&self) -> StdMask {
        self.self_eq(StdSimd::from_array(CHARS))
    }

    fn find<const CHAR: u8>(&self) -> StdMask {
        self.simd_eq(StdSimd::splat(CHAR))
    }

    /// returns the number of chars before (aka the index of) the desired char
    ///
    /// returns 8 if it is not found
    fn index_of<const CHAR: u8>(&self) -> usize {
        // set not-equal values to 0
        self.simd_eq(StdSimd::splat(CHAR))
            // the first n zeros will be the index
            .to_bitmask()
            .leading_zeros() as usize
    }
}

