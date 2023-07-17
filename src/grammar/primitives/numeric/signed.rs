use std::marker::PhantomData;
use crate::grammar::primitives::numeric::unsigned::UnsignedParserState;
use crate::parse::{MatchResult, Parser, Stateful};
use crate::parse::MatchResult::{Consumed, NoMatch, Parsed};

enum SignedState {
    First,
    Neg,
    Pos,
}

struct SignedParserState<T: From<u64>, const HALF: u64> {
    state_t_pos: UnsignedParserState<u64, { HALF - 1 }>,
    state_t_neg: UnsignedParserState<u64, HALF>,
    state: SignedState,
    _p: PhantomData<T>
}

impl<T: From<u64>, const HALF: u64> Stateful<(bool, T)> for SignedParserState<T, HALF> {
    fn new() -> Self {
        SignedParserState::<T, HALF> {
            state_t_pos: UnsignedParserState::<u64, { HALF - 1 }>::new(),
            state_t_neg: UnsignedParserState::<u64, HALF>::new(),
            state: SignedState::First,
            _p: PhantomData
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<T> {
        match self.state {
            SignedState::First => {
                match byte {
                    b'-' => {
                        self.state = SignedState::Neg;
                        Consumed
                    }
                    b'0'..=b'9' => {
                        self.state = SignedState::Pos;
                        self.state_t_pos.parse(byte)
                    }
                    _ => NoMatch
                }
            }
            SignedState::Pos => {
                match self.state_t_pos.parse(byte) {
                    Parsed(ok) => Parsed(ok as T),
                    Consumed => Consumed,
                    _ => NoMatch
                }
            }
            SignedState::Neg => {
                match self.state_t_neg.parse(byte) {
                    Parsed(ok) => Parsed(-(ok as T)),
                    Consumed => Consumed,
                    _ => NoMatch
                }
            }
        }
    }
}

pub struct I64 { number: i64 }
impl Parser for I64 {
    type State = SignedParserState<i64, { u64::MAX/ 2 }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}

pub struct I32 { number: i32 }
impl Parser for I32 {
    type State = SignedParserState<i32, { (u32::MAX/ 2) as u64 }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}

pub struct I16 { number: i16 }
impl Parser for I16 {
    type State = SignedParserState<i16, { (u16::MAX/ 2) as u64 }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}