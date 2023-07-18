use std::intrinsics::{likely, unlikely};
use crate::grammar::literals::uuid::UUIDState::*;
use crate::parse::{MatchResult, Stateful};
use std::simd::{Mask, mask8x8, u8x8};

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

const ASCII_DIGIT_MASK: mask8x8 = Mask::fr;

struct UUIDParserState {
    hex_size: usize,
    data: u128,
    state: UUIDState,
}
impl UUIDParserState {
    fn parse_hex<const LEN: usize>(bytes: u8x8) {
        let wow = &bytes[0];
        let decimal_value =
            ((byte as usize) & ASCII_DIGIT_MASK) / (b'0' as usize - 1)
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

