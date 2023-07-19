use crate::parse::MatchResult::{Consumed, NoMatch, Parsed};
use crate::parse::{MatchResult, Parser, Stateful};
use std::intrinsics::likely;
use std::marker::PhantomData;
use std::simd::u8x8;
use crate::util::ascii_simd::producing_mask::AsciiUtilsProducingMask;
use crate::util::ascii_simd::producing_simd::AsciiUtilsProducingSimd;

pub struct UnsignedParserState<T, const MAX: u64> {
    number: u64,
    len: usize,
    _p: PhantomData<T>,
}

impl<T, const MAX: u64> Stateful for UnsignedParserState<T, MAX> {
    type Out = T;

    fn new() -> Self {
        UnsignedParserState::<T, MAX> {
            number: 0,
            len: 0,
            _p: PhantomData,
        }
    }

    fn parse(&mut self, bytes: u8x8) -> MatchResult<T> {
        let prev = self.number.clone();
        bytes.ascii_hex_digit()

        todo!()

        // self.number = self
        //     .number
        //     .wrapping_mul(10)
        //     .wrapping_add(((byte as usize) - (b'0' as usize)) as u64);
        // self.len += 1;
        //
        // match byte {
        //     b'0'..=b'9' => Consumed,
        //     _ => {
        //         // make sure the number we parsed fits in a u64 and is at least 1 character
        //         if likely(
        //             self.len > 1
        //                 && self.len <= u64::MAX.ilog10() as usize
        //                 && prev <= self.number
        //                 && self.number <= MAX,
        //         ) {
        //             Oops(self.number.clone())
        //         } else {
        //             if self.len == 0 {
        //                 NoMatch("expected number literal")
        //             } else {
        //                 NoMatch("number literal is too long")
        //             }
        //         }
        //     }
        // }
    }
}

const fn to_u64<T: Into<u64>>(num: T) -> u64 {
    num as u64
}

pub struct U64 {
    number: u64,
}
impl Parser for U64 {
    type State = UnsignedParserState<u64, { u64::MAX }>;
}

pub struct U32 {
    number: u32,
}
impl Parser for U32 {
    type State = UnsignedParserState<u32, { u32::MAX as u64 }>;
}

pub struct U16 {
    number: u16,
}
impl Parser for U16 {
    type State = UnsignedParserState<u16, { u16::MAX as u64 }>;
}

pub struct Byte {
    number: u8,
}
impl Parser for Byte {
    type State = UnsignedParserState<u8, { u8::MAX as u64 }>;
}

pub struct Stack {
    number: usize,
}
impl Parser for Stack {
    type State = UnsignedParserState<usize, 64>;
}
