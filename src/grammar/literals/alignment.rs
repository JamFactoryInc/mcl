use std::ops::{Deref, Shr};
use std::simd::{SimdOrd, SimdUint, ToBitMask, u8x8};
use crate::parse::{MatchResult, SIMD_LANE_SIZE, Stateful, StdSimd};
use crate::parse::MatchResult::{Consumed, NoMatch, Parsed};
use crate::util::ascii_simd::simd::SimdUtils;

enum ParserState {
    NotSeen,
    Seen,
}

pub struct AlignmentParser {
    seen: ParserState,
    accumulator: u8,
}
impl Stateful for AlignmentParser {
    type Out = Alignment;

    fn new() -> Self {
        AlignmentParser {
            seen: ParserState::NotSeen,
            accumulator: 0,
        }
    }

    fn parse(&mut self, bytes: StdSimd) -> MatchResult<Self::Out> {
        let xyz_mask = bytes.range::<b'x', b'z'>();
        let bitmask = xyz_mask.to_bitmask();
        let packed = StdSimd::splat(0b100)
            // shift right by x=0, y=1, z=2
            .shr(bytes.justify_to_zero::<b'x'>())
            // get rid of values outside the range
            .mask(xyz_mask)
            // horizontal bitor to collect the bit packed u8, then convert to Alignment
            .reduce_or();

        match self.seen {
            ParserState::NotSeen => {
                match (bitmask.count_ones(), bitmask.trailing_zeros(), &packed) {
                    // we got 3 xzy chars and they're unique
                    (3, trail, 0b111) => {
                        Parsed(SIMD_LANE_SIZE - trail, Alignment::XYZ)
                    }
                    // we got 2 xzy chars terminated and they're unique
                    (2, trail @ 1.., 0b101 | 0b110 | 0b011) => {
                        Parsed(SIMD_LANE_SIZE - trail, Alignment::XYZ)
                    }
                    (..=2, trail, _) => {
                        self.accumulator = packed;
                        self.seen = ParserState::Seen;
                        Consumed(SIMD_LANE_SIZE)
                    }

                }
            }
            ParserState::Seen => {

            }
        }
        if packed == 0u8 {
            NoMatch("expected")
        } else {
            let bitmask = xyz_mask.to_bitmask();
            Parsed(bitmask.leading_zeros() + bitmask., packed as Alignment)
        }
    }
}

pub struct Alignment(u8);
impl Alignment {
    const X: Alignment = Alignment(0b100);
    const Y: Alignment = Alignment(0b010);
    const Z: Alignment = Alignment(0b001);
    const XY: Alignment = Alignment(Self::X & Self::Y);
    const XZ: Alignment = Alignment(Self::X & Self::Z);
    const YZ: Alignment = Alignment(Self::Y & Self::Z);
    const XYZ: Alignment = Alignment(Self::X & Self::Y & Self::Z);

    fn aligned_to(&self, alignment: Alignment) -> bool {
        self & alignment == alignment
    }

    fn from(bytes: StdSimd) -> Alignment {
        StdSimd::splat(0b100)
            // shift right by x=0, y=1, z=2
            .shr(bytes.justify_to_zero::<b'x'>())
            // get rid of values outside the range
            .mask(bytes.range::<b'x', b'z'>())
            // horizontal bitor to collect the bit packed u8, then convert to Alignment
            .reduce_or() as Alignment
    }
}

impl From<u8> for Alignment {
    fn from(value: u8) -> Self {
        Alignment(value)
    }
}

impl Deref for Alignment {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        *self.0
    }
}