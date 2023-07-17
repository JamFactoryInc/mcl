use std::intrinsics::likely;
use std::marker::PhantomData;
use crate::parse::{MatchResult, Parser, Stateful};
use crate::parse::MatchResult::{Consumed, NoMatch, Parsed};

pub struct UnsignedParserState<T, const MAX: u64> {
    number: u64,
    len: usize,
    _p: PhantomData<T>
}

impl<T, const MAX: u64> Stateful<T> for UnsignedParserState<T, MAX> {
    fn new() -> Self {
        UnsignedParserState::<T, MAX> {
            number: 0,
            len: 0,
            _p: PhantomData
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<T> {
        let prev = self.number.clone();
        self.number = self.number.wrapping_mul(10).wrapping_add(((byte as usize) - (b'0' as usize)) as u64);
        self.len += 1;
        match byte {
            b'0'..=b'9' => {
                Consumed
            }
            _ => {
                // make sure the number we parsed fits in a u64 and is at least 1 character
                if likely(self.len > 1
                    && self.len <= u64::MAX.ilog10() as usize
                    && prev <= self.number
                    && self.number <= MAX) {
                    Parsed(self.number.clone())
                } else {
                    NoMatch
                }
            }
        }
    }
}

const fn to_u64<T: Into<u64>>(num: T) -> u64 {
    num as u64
}

pub struct U64 { number: u64 }
impl Parser for U64 {
    type State = UnsignedParserState<u64, { u64::MAX }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}

pub struct U32 { number: u32 }
impl Parser for U32 {
    type State = UnsignedParserState<u32, { u32::MAX as u64 }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}

pub struct U16 { number: u16 }
impl Parser for U16 {
    type State = UnsignedParserState<u16, { u16::MAX as u64 }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}

pub struct U8 { number: u8 }
impl Parser for U8 {
    type State = UnsignedParserState<u8, { u8::MAX as u64 }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}

pub struct Stack { number: usize }
impl Parser for Stack {
    type State = UnsignedParserState<usize, 64>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}