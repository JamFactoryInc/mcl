use std::simd::{mask8x8, ToBitMask, u8x8};

pub mod mask;
pub mod producing_mask;
pub mod producing_simd;
pub mod simd;

pub const fn s(val: u8) -> u8x8 {
    u8x8::from_array([val; 8])
}
pub const fn m(val: u8) -> mask8x8 {
    mask8x8::from_bitmask(val)
}