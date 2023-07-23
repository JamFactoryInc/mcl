use std::simd::{u32x8, u8x8};

pub mod mask;
pub mod simd;

pub const DIGIT_BLOCK_FILTER: u8x8 = u8x8::splat(63u8);
pub const LOWER_A: u8x8 = u8x8::splat(b'a');
pub const LOWER_Z: u8x8 = u8x8::splat(b'z');
pub const UPPER_A: u8x8 = u8x8::splat(b'A');
pub const UPPER_Z: u8x8 = u8x8::splat(b'Z');
pub const ZERO: u8x8 = u8x8::splat(b'0');
pub const NINE: u8x8 = u8x8::splat(b'9');
pub const ALPHA_BLOCK: u8x8 = u8x8::splat(b'@');
pub const ALPHA_BLOCK_SIZE: u8x8 = u8x8::splat(32);
pub const ALPHA_LENGTH: u8 = 26;

/// pre-computed base-16 orders of magnitude so we can easily sum the simd lanes into an integer
pub const HEX_ORDERS: u32x8 = u32x8::from_array([0x1000_0000, 0x100_0000, 0x10_0000, 0x1_0000, 0x1000, 0x100, 0x10, 0x1]);

/// pre-computed base-10 orders of magnitude so we can easily sum the simd lanes into an integer
pub  const DEC_ORDERS: u32x8 = u32x8::from_array([1000_0000, 100_0000, 10_0000, 1_0000, 1000, 100, 10, 1]);