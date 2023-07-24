use crate::parse::StdSimd;

pub mod mask;
pub mod simd;

pub const DIGIT_BLOCK_FILTER: StdSimd = StdSimd::splat(63u8);
pub const LOWER_A: StdSimd = StdSimd::splat(b'a');
pub const LOWER_Z: StdSimd = StdSimd::splat(b'z');
pub const UPPER_A: StdSimd = StdSimd::splat(b'A');
pub const UPPER_Z: StdSimd = StdSimd::splat(b'Z');
pub const ZERO: StdSimd = StdSimd::splat(b'0');
pub const NINE: StdSimd = StdSimd::splat(b'9');
pub const ALPHA_BLOCK: StdSimd = StdSimd::splat(b'@');
pub const ALPHA_BLOCK_SIZE: StdSimd = StdSimd::splat(32);
pub const ALPHA_LENGTH: u8 = 26;
pub const JUSTIFIED_A: StdSimd = StdSimd::splat(0);
pub const JUSTIFIED_Z: StdSimd = StdSimd::splat(ALPHA_LENGTH - 1);

/// pre-computed base-16 orders of magnitude so we can easily sum the simd lanes into an integer
pub const HEX_ORDERS: u32x8 = u32x8::from_array([0x1000_0000, 0x100_0000, 0x10_0000, 0x1_0000, 0x1000, 0x100, 0x10, 0x1]);

/// pre-computed base-10 orders of magnitude so we can easily sum the simd lanes into an integer
pub  const DEC_ORDERS: u32x8 = u32x8::from_array([1000_0000, 100_0000, 10_0000, 1_0000, 1000, 100, 10, 1]);