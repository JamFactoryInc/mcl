use std::intrinsics::{likely, unlikely};
use std::ops::{BitAnd, Index, Rem, Sub};
use crate::grammar::literals::uuid::UUIDState::*;
use crate::parse::{MatchResult, Stateful};
use std::simd::{Mask, mask8x8, SimdOrd, SimdPartialEq, SimdUint, ToBitMask, u8x8};

enum UUIDState {
    Hex1,
    Dash1,
    Hex2,
    Dash2,
    Hex3,
    Dash3,
    Hex4,
    Dash4,
    Hex5,
}

const ASCII_DIGIT_MASK: usize = 0x40;
const SIMD_CHAR_0: u8x8 = u8x8::from([b'0'; 8]);
const SIMD_CHAR_9: u8x8 = u8x8::from([b'9'; 8]);
const SIMD_ALPHA_BLOCK_START: u8x8 = u8x8::from([b'@'; 8]);
const SIMD_UPPER_A: u8x8 = u8x8::from([b'A'; 8]);
const SIMD_1: u8x8 = u8x8::from([1; 8]);
const SIMD_26: u8x8 = u8x8::from([26; 8]);
const SIMD_32: u8x8 = u8x8::from([32; 8]);

struct UUIDParserState {
    hex_size: usize,
    data: u128,
    state: UUIDState,
}
impl UUIDParserState {
    fn parse_hex(bytes: u8x8) -> u64 {
        let masked: u8x8 = bytes & ASCII_DIGIT_MASK;
        let decimal = masked.simd_clamp(SIMD_CHAR_0, SIMD_CHAR_9)
            .sub(SIMD_CHAR_0);
        let hex = masked.rem(SIMD_ALPHA_BLOCK_START)
            .clamp(SIMD_1, SIMD_26)
            .sub(SIMD_26);

    }
}

impl Stateful for UUIDParserState {
    type Out = ();

    fn new() -> Self {
        UUIDParserState {
            hex_size: 0,
            data: 0,
            state: Hex1,
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<Self::Out> {
        match self.state {
            Hex1 => {

            }
            Dash1 => {

            }
        }
    }
}

pub struct UUID {
    data: u128
}

