use crate::grammar::literals::numeric::unsigned::UnsignedParserState;
use crate::parse::{MatchResult, Parser, Stateful};
use std::marker::PhantomData;
use crate::parse::MatchResult::NoMatch;

enum SignedState {
    First,
    Neg,
    Pos,
}

const fn sub_one(val: u64) -> u64 {
    val - 1
}

struct SignedParserState<T: From<i64>, const HALF: u64> {
    state_t_pos: UnsignedParserState<u64, { HALF - 1 }>,
    state_t_neg: UnsignedParserState<u64, HALF>,
    state: SignedState,
    _p: PhantomData<T>,
}

impl<T: From<u64>, const HALF: u64> Stateful for SignedParserState<T, HALF> {
    type Out = T;

    fn new() -> Self {
        SignedParserState::<T, HALF> {
            state_t_pos: UnsignedParserState::<u64, { HALF - 1 }>::new(),
            state_t_neg: UnsignedParserState::<u64, HALF>::new(),
            state: SignedState::First,
            _p: PhantomData,
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<T> {
        match self.state {
            SignedState::First => match byte {
                b'-' => MatchResult::consume(|| self.state = SignedState::Neg),
                b'0'..=b'9' => {
                    self.state = SignedState::Pos;
                    self.state_t_pos.parse(byte)
                        .bubble(|ok| ok as T)
                }
                _ => NoMatch("invalid leading character in signed number literal"),
            },
            SignedState::Pos => self.state_t_pos.parse(byte)
                .bubble(|ok| ok as T),
            SignedState::Neg => self.state_t_pos.parse(byte)
                .bubble(|ok| -(ok as T)),
        }
    }
}

pub struct Long {
    number: i64,
}
impl Parser for Long {
    type State = SignedParserState<i64, { u64::MAX / 2 }>;
}

pub struct Int {
    number: i32,
}
impl Parser for Int {
    type State = SignedParserState<i32, { (u32::MAX / 2) as u64 }>;
}

pub struct Short {
    number: i16,
}
impl Parser for Short {
    type State = SignedParserState<i16, { (u16::MAX / 2) as u64 }>;
}
