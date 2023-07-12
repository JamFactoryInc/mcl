use std::fmt::Display;
use derive_more::Display;
use crate::parse::{ParseError, Parser, Suggestion};
use crate::src_in::Source;
use crate::vm::LayoutContext;

trait UnsignedPrimitive {}

impl UnsignedPrimitive for u32 {}
impl UnsignedPrimitive for usize {}
impl<T> Parser<T> for T where T: UnsignedPrimitive + Display {
    fn parse<'a>(&self, src: &mut Source) -> Result<T, ParseError> {
        let mut result = match src.peek() {
            num @ b'0'..=b'9' => {
                src.next();
                num as T - b'0'
            },
            _ => return Err(self.get_error(src))
        };
        loop {
            match src.peek() {
                num @ b'0'..=b'9' => {
                    src.next();
                    result = match result.checked_mul(10).and_then(|r| r.checked_add(num as T - b'0')) {
                        Some(r) => r,
                        _ => break Err(self.get_error(src)),
                    };
                },
                _ => break Ok(result)
            }
        }
    }
}

trait SignedPrimitive {}
impl<Signed> Parser<Signed> for Signed where Signed: SignedPrimitive + Display {
    fn parse<'a>(&self, src: &mut Source) -> Result<Signed, ParseError> {
        let mut result = match src.peek() {
            num @ b'0'..=b'9' => {
                src.next();
                num as Signed - b'0'
            },
            _ => return Err(self.get_error(src))
        };
        loop {
            match src.peek() {
                num @ b'0'..=b'9' => {
                    src.next();
                    result = match result.checked_mul(10).and_then(|r| r.checked_add(num as Signed - b'0')) {
                        Some(r) => r,
                        _ => break Err(self.get_error(src)),
                    };
                },
                _ => break Ok(result)
            }
        }
    }
}

impl<T> Parser for Option<T> where T: Parser + Display {
    fn get_error(&self, src: &mut Source) -> ParseError {
        todo!()
    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    fn parse<'a>(&self, src: &mut Source, context: &'a mut LayoutContext) {
        todo!()
    }
}

pub struct UDecimal { int: Option<u32>, dec: Option<u32> }
impl Parser for UDecimal {
    fn get_error(&self, src: &mut Source) -> ParseError {
        ParseError::from(src, "one of ")
    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    fn parse<'a>(&self, src: &mut Source, context: &'a mut LayoutContext) {
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