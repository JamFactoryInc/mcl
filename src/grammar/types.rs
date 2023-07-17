use std::fmt::{Display};
use std::intrinsics::{likely, unlikely};
use std::marker::PhantomData;
use std::ops::{Add, Mul};
use std::string::{ParseError, ToString};
use derive_more::Display;
use crate::parse::{MatchResult, Optional, Parser, Stateful, Suggestion};
use crate::src_in::Source;
use crate::vm::LayoutContext;

struct Base10UnsignedParserState<T, const MAX: u128> {
    state: u128,
    len: usize,
    _p: PhantomData<T>
}

impl<T, const MAX: u128> Stateful<T> for Base10UnsignedParserState<T, MAX> {
    fn new() -> Self {
        Base10UnsignedParserState::<T, MAX> {
            state: 0,
            len: 0,
            _p: PhantomData
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<T> {
        self.state = self.state.clone() * 10 + byte as usize - b'0' as usize;
        self.len += 1;
        match byte {
            b'0'..=b'9' => {
                if likely(self.state <= MAX) {
                    MatchResult::Consumed
                } else {
                    MatchResult::NoMatch
                }
            }
            _ => {
                if likely(self.len > 1) {
                    MatchResult::Parsed(self.state.clone())
                } else {
                    MatchResult::NoMatch
                }
            }
        }
    }
}

enum SignedState {
    First,
    Neg,
    Pos,
}

struct SignedParserState<T, const MAX: u128> {
    number: u128,
    len: usize,
    state: SignedState,
}

impl<T: Parser> Stateful<(bool, T)> for SignedParserState<(bool, T)> {
    fn new() -> Self {
        T::State::new()
    }

    fn parse(&mut self, byte: u8) -> MatchResult<T> {
        match self.state {
            SignedState::First => {

            }
            SignedState::Pos => {

            }
            SignedState::Neg => {

            }
        }
    }
}

const fn to_u128<T: Into<u128>>(num: T) -> u128 {
    num as u128
}

struct U32 { number: u32 }
impl Parser for U32 {
    type State = Base10UnsignedParserState<u32, { to_u128(u32::MAX) }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}

struct U64 { number: u64 }
impl Parser for U64 {
    type State = Base10UnsignedParserState<u64, { to_u128(u64::MAX) }>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}

struct Stack64 { number: usize }
impl Parser for Stack64 {
    type State = Base10UnsignedParserState<usize, 64>;
    const ERR: fn() -> String = || "expected number from 0 to 9".to_string();
}


trait UnsignedPrimitive {
    fn parse_unsigned<T : Parser>(parser: &T, src: &mut Source) -> Result<T, ParseError> {
        let mut result = match src.peek() {
            num @ b'0'..=b'9' => {
                src.next();
                num as T - b'0'
            },
            _ => return Err(parser.get_error(src))
        };
        loop {
            match src.peek() {
                num @ b'0'..=b'9' => {
                    src.next();
                    result = match result.checked_mul(10).and_then(|r| r.checked_add(num as T - b'0')) {
                        Some(r) => r,
                        _ => break Err(parser.get_error(src)),
                    };
                },
                _ => break Ok(result)
            }
        }
    }
}

impl UnsignedPrimitive for u32 {}
impl UnsignedPrimitive for usize {}
impl<T: UnsignedPrimitive + Display> Parser for T {
    type Out = T;
    type SP = ();
    const ERR: fn() -> String = || "".to_string();

    fn test(src: &mut Source) -> bool {
        match src.peek() {
            b'0'..=b'9' => true,
            _ => false,
        }
    }

    fn parse<'a>(src: &mut Source) -> Result<T, ParseError> {
        UnsignedPrimitive::parse(src)
    }
}

trait SignedPrimitive {
    fn parse_signed<T : Parser>(parser: &T, src: &mut Source) -> Result<T, ParseError> {
        let additive_fn = match src.peek() {
            b'-' => |r, num| r.checked_add(num as T - b'0'),
            _ => |r, num| r.ch(num as T - b'0'),
        };
        let mut result = match src.peek() {
            num @ b'0'..=b'9' => {
                src.next();
                num as T - b'0'
            },
            _ => return Err(parser.get_error(src))
        };
        loop {
            match src.peek() {
                num @ b'0'..=b'9' => {
                    src.next();
                    result = match result.checked_mul(10).and_then(|r| additive_fn(r, num)) {
                        Some(r) => r,
                        _ => break Err(parser.get_error(src)),
                    };
                },
                _ => break Ok(result)
            }
        }
    }
}

#[derive(Display)]
#[display(fmt = "{}..{}", int, dec)]
pub struct UDecimal { int: Optional<u32>, dec: Optional<u32> }
impl Parser for UDecimal {
    type Out = UDecimal;
    fn get_error(&self, src: &mut Source) -> ParseError {
        ParseError::from(src, "one of ")
    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    fn parse<'a>(&self, src: &mut Source) {
        todo!()
    }
}
pub struct Decimal { int: i32, dec: i32 }
pub struct Range { from: isize, to: isize }
pub struct URange { from: usize, to: usize }
pub struct DecimalRange { from: Decimal, to: Decimal }
pub struct UDecimalRange { from: UDecimal, to: UDecimal }
pub struct Namespace { name: String }
pub struct Resource { is_tag: bool, namespace: Namespace, path: Vec<McIdentifier> }

/// `[_.-A-z0-9]+`
#[derive(Display)]
pub struct McIdentifier {
    str: String
}
impl Parser for McIdentifier {
    fn get_error(&self, src: &mut Source) -> ParseError {
        todo!()
    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    fn parse(&self, src: &mut Source, context: &mut LayoutContext){
        todo!()
    }
}
/// `[A-z_][A-z_0-9]*`
#[derive(Display)]
pub struct Identifier {
    str: String
}

impl Parser for Identifier {
    fn get_error(&self, src: &mut Source) -> ParseError {
        todo!()
    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    fn parse(&self, src: &mut Source, context: &mut LayoutContext){
        todo!()
    }
}