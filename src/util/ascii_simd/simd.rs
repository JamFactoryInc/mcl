use std::iter::Iterator;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Sub};
use std::simd::{Simd, SimdInt, SimdPartialEq, SimdPartialOrd, SimdUint, ToBitMask};
use crate::parse::{SIMD_LANE_SIZE, StdMask, StdSimd};
use crate::util::ascii_simd::{ALPHA_BLOCK, ALPHA_BLOCK_SIZE, ALPHA_LENGTH, DEC_ORDERS, JUSTIFIED_Z, LOWER_A, UPPER_A, ZERO};
use crate::util::ascii_simd::mask::MaskAsciiUtils;


pub trait SimdUtils {
    /// brings the simd vector's lower bound to have the ascii value of `NEW_MIN` given the `MIN`
    /// and sets outlying characters to 0
    ///
    /// e.g. `justify_range<12, b'a', b'z'>` behaves as:
    /// ```
    /// b'a' => (12 + b'a' - b'a') = 12
    /// b'j' => (12 + b'j' - b'a') = 21
    /// b'1' => 0
    fn justify_range<const NEW_MIN: u8, const MIN: u8, const MAX_INCLUSIVE: u8> (&self) -> StdSimd;

    /// brings the simd vector's lower bound to have the ascii value of `NEW_MIN` given the `MIN`
    ///
    /// e.g. `justify<12, b'a', b'z'>` behaves as:
    /// ```
    /// b'a' => (12 + b'a' - b'a') = 12
    /// b'j' => (12 + b'j' - b'a') = 21
    /// b'1' => (12 + b'1' - b'a') = -36
    fn justify<const NEW_MIN: u8, const MIN: u8> (&self) -> StdSimd;

    /// brings the given simd vector's lower bound to equal zero, and zeroes any lanes that are outside the range
    fn justify_range_to_zero<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> StdSimd;

    /// a fancy way of writing
    /// ```
    /// self.sub(StdSimd::splat(MIN))
    /// ```
    /// brings the given simd vector's lower bound to equal zero
    fn justify_to_zero<const MIN: u8>(&self) -> StdSimd;

    /// returns a tuple containing:
    ///
    /// - a simd vector representing the found digits
    /// - a mask representing which lanes were valid hexadecimal digits
    fn parse_base_16(&self) -> (StdSimd, StdMask);

    /// returns a tuple containing:
    /// - a simd vector representing the found digits
    /// - a mask representing which lanes were valid decimal digits
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
    fn use_and_add(&self, new_values: &[u8]) -> StdSimd;

    /// returns a mask representing which lanes are within the contiguous ascii range MIN-MAX (inclusive)
    fn range<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> StdMask;

    /// returns a mask representing which lanes are valid A-z alpha characters
    fn alpha(&self) -> StdMask;

    /// returns a mask representing which lanes are valid A-z or 0-9 character
    fn alpha_numeric(self: StdSimd) -> StdMask;

    /// returns a mask representing which lanes are valid a-z alpha characters
    fn lower(&self) -> StdMask;

    /// returns a mask representing which lanes are valid A-Z alpha characters
    fn upper(&self) -> StdMask;

    /// returns a mask representing which lanes are valid A-F hexadecimal digits
    fn hex_alpha(&self) -> StdMask;

    /// returns a mask representing which lanes are valid 0-F hexadecimal digits
    fn hex(&self) -> StdMask;

    /// returns a mask representing which lanes are valid 0-9 digits
    fn digit(&self) -> StdMask;

    /// returns a mask representing the lanes that matched the given character sequence
    fn sequence<const chars: [u8; SIMD_LANE_SIZE]>(&self) -> StdMask;

    /// returns a mask representing the lanes that matched the desired character
    fn find<const CHAR: u8>(&self) -> StdMask;

    /// returns the number of chars before (aka the index of) the desired char
    ///
    /// returns 8 if it is not found
    fn index_of<const CHAR: u8>(&self) -> usize;

    /// sets the lanes to zero where the given mask is false
    fn mask(&self, mask: StdMask) -> StdSimd;

    /// merges the ranges A-Z and a-z to both have a=1
    fn flatten_alpha(&self) -> StdSimd;

    /// merges the ranges A-Z and a-z to both have a=0
    fn flatten_alpha_zero(&self) -> StdSimd;

    /// matches whitespace (actually just control characters or space)
    fn whitespace(&self) -> StdMask;

    /// only matches whitespace
    fn slow_whitespace(&self) -> StdMask;

    /// gets a mask representing the position of word chars (chars in identifiers)
    fn word(&self) -> StdMask;

    /// gets a mask representing the position of non-word chars (chars that are not connected to identifiers)
    fn non_word(&self) -> StdMask;
}

impl SimdUtils for StdSimd {
    #[inline]
    fn justify_range<const NEW_MIN: u8, const MIN: u8, const MAX_INCLUSIVE: u8> (&self) -> StdSimd {
        self.add(StdSimd::splat(NEW_MIN - MIN))
            .bitand(self.range::<MIN, MAX_INCLUSIVE>())
    }

    #[inline]
    fn justify<const NEW_MIN: u8, const MIN: u8> (&self) -> StdSimd {
        self.add(StdSimd::splat(NEW_MIN - MIN))
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
        // flatten A-Z and a-z to b'A' == b'a' == 10 via modulus
        let hex_values = self.rem(StdSimd::splat(b'a' - 10))
            .rem(StdSimd::splat(b'A' - 10));
        // get a mask representing values < 0xA i.e. values that were below A-F and should be removed
        let bad_hex_mask = hex_values.simd_lt(StdSimd::splat(0xA));
        // also flatten 0-9 into the vector
        let values = hex_values.rem(StdSimd::splat(b'0'));

        let mask = bad_hex_mask.bitor(
            // get a mask representing values > 0xF i.e. values that were above 0-F and should be removed
            values.simd_gt(StdSimd::splat(0xF))
        )
            // the mask is now true if the value is bad
            // now we get a mask representing whether the value was originally in a valid range
            .bitor(self.simd_le(StdSimd::splat(0xF)))
            // and finally invert the mask so true == valid
            .simd_ne(StdMask::from_array([true; SIMD_LANE_SIZE]));

        (values, mask)
    }

    #[inline]
    fn parse_base_10(&self) -> (StdSimd, StdMask) {
        let digit_mask = self.digit();
        (
            self.sub(ZERO).mask(digit_mask),
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
    fn use_and_add(&self, new_values: &[u8]) -> StdSimd {
        // set the last n values to the contents of the given slice
        let mut vals = [0u8; SIMD_LANE_SIZE];
        let used_amount = new_values.len();
        vals[(SIMD_LANE_SIZE - &used_amount)..].copy_from_slice(new_values);
        let (self_rotated, inverse_mask) = self.use_amount(used_amount).do_zero();

        self_rotated.add(
            StdSimd::from_array(vals)
                .bitand(inverse_mask.to_int().cast())
        )
    }

    #[inline]
    fn range<const MIN: u8, const MAX_INCLUSIVE: u8>(&self) -> StdMask {
        if MIN == 0 {
            self.self_le(StdSimd::splat(MAX_INCLUSIVE))
        } else {
            self.self_ge(StdSimd::splat(MIN))
                .bitand(self.self_le(StdSimd::splat(MAX_INCLUSIVE)))
        }

    }

    #[inline]
    fn alpha(self: StdSimd) -> StdMask {
        // make sure our bytes are in the alpha blocks
        self.self_ge(ALPHA_BLOCK)
            // AND
            .bitand(
                // flatten the ascii blocks to 0-31
                self.flatten_alpha_zero()
                    // make sure the flattened values are from 1-26 (a-z)
                    .range::<0, ALPHA_LENGTH>()
            )
    }

    #[inline]
    fn alpha_numeric(self: StdSimd) -> StdMask {
        // make sure the original values are at least b'0'
        self.simd_ge(ZERO)
            .bitand(
                // flatten a-z and A-Z so b'A' == b'a' == 0 (b'z' == 25)
                // then flatten 0-9 so b'9' == 25 (same as b'z')
                self.rem(LOWER_A)
                    .rem(UPPER_A)
                    .rem(StdSimd::splat(ALPHA_LENGTH - 10))
                    // finally, make sure all values are at most 25 (b'z' b'Z' or b'9'
                    .simd_le(JUSTIFIED_Z)
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
    fn sequence<const CHARS: [u8; SIMD_LANE_SIZE]>(&self) -> StdMask {
        self.self_eq(StdSimd::from_array(CHARS))
    }

    #[inline]
    fn find<const CHAR: u8>(&self) -> StdMask {
        self.simd_eq(StdSimd::splat(CHAR))
    }

    #[inline]
    fn index_of<const CHAR: u8>(&self) -> usize {
        // set not-equal values to 0
        self.simd_eq(StdSimd::splat(CHAR))
            // the first n zeros will be the index
            .count_misses()
    }

    #[inline]
    fn mask(&self, mask: StdMask) -> StdSimd {
        self.bitand(mask.to_int().cast())
    }

    #[inline]
    fn flatten_alpha(&self) -> StdSimd {
        self.rem(ALPHA_BLOCK_SIZE)
    }

    #[inline]
    fn flatten_alpha_zero(&self) -> StdSimd {
        self.flatten_alpha()
            .justify_to_zero::<{ b'a' % ALPHA_BLOCK_SIZE }>()
    }

    #[inline]
    fn whitespace(&self) -> StdMask {
        self.simd_le(StdSimd::splat(b' '))
    }

    #[inline]
    fn slow_whitespace(&self) -> StdMask {
        // 9 | 10 | 13 | 32
        self.simd_ge(StdSimd::splat(b'\t'))
            .bitand(self.simd_le(StdSimd::splat(b'\r')))
            .bitor(self.simd_eq(StdSimd::splat(b' ')))
    }

    #[inline]
    fn word(&self) -> StdMask {
        self.alpha_numeric().bitor(StdSimd::splat(b'_'))
    }

    #[inline]
    fn non_word(&self) -> StdMask {
        self.alpha().bitor(StdSimd::splat(b'_'))
    }
}

