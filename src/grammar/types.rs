use std::fmt::{Display};
use std::intrinsics::{likely, unlikely};
use std::marker::PhantomData;
use std::ops::{Add, Mul};
use std::string::{ParseError, ToString};
use derive_more::Display;
use crate::parse::{MatchResult, Optional, Parser, Stateful, Suggestion};
use crate::parse::MatchResult::*;
use crate::src_in::Source;
use crate::vm::LayoutContext;


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

// impl UnsignedPrimitive for u32 {}
// impl UnsignedPrimitive for usize {}
// impl<T: UnsignedPrimitive + Display> Parser for T {
//     type Out = T;
//     type SP = ();
//     const ERR: fn() -> String = || "".to_string();
//
//     fn test(src: &mut Source) -> bool {
//         match src.peek() {
//             b'0'..=b'9' => true,
//             _ => false,
//         }
//     }
//
//     fn parse<'a>(src: &mut Source) -> Result<T, ParseError> {
//         UnsignedPrimitive::parse(src)
//     }
// }

// trait SignedPrimitive {
//     fn parse_signed<T : Parser>(parser: &T, src: &mut Source) -> Result<T, ParseError> {
//         let additive_fn = match src.peek() {
//             b'-' => |r, num| r.checked_add(num as T - b'0'),
//             _ => |r, num| r.ch(num as T - b'0'),
//         };
//         let mut result = match src.peek() {
//             num @ b'0'..=b'9' => {
//                 src.next();
//                 num as T - b'0'
//             },
//             _ => return Err(parser.get_error(src))
//         };
//         loop {
//             match src.peek() {
//                 num @ b'0'..=b'9' => {
//                     src.next();
//                     result = match result.checked_mul(10).and_then(|r| additive_fn(r, num)) {
//                         Some(r) => r,
//                         _ => break Err(parser.get_error(src)),
//                     };
//                 },
//                 _ => break Ok(result)
//             }
//         }
//     }
// }

// #[derive(Display)]
// #[display(fmt = "{}..{}", int, dec)]
// pub struct UDecimal { int: Optional<u32>, dec: Optional<u32> }
// impl Parser for UDecimal {
//     type Out = UDecimal;
//     fn get_error(&self, src: &mut Source) -> ParseError {
//         ParseError::from(src, "one of ")
//     }
//
//     fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
//         todo!()
//     }
//
//     fn parse<'a>(&self, src: &mut Source) {
//         todo!()
//     }
// }
// pub struct Decimal { int: i32, dec: i32 }
// pub struct Range { from: isize, to: isize }
// pub struct URange { from: usize, to: usize }
// pub struct DecimalRange { from: Decimal, to: Decimal }
// pub struct UDecimalRange { from: UDecimal, to: UDecimal }
// pub struct Namespace { name: String }
// pub struct Resource { is_tag: bool, namespace: Namespace, path: Vec<McIdentifier> }
//
// /// `[_.-A-z0-9]+`
// #[derive(Display)]
// pub struct McIdentifier {
//     str: String
// }
// impl Parser for McIdentifier {
//     fn get_error(&self, src: &mut Source) -> ParseError {
//         todo!()
//     }
//
//     fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
//         todo!()
//     }
//
//     fn parse(&self, src: &mut Source, context: &mut LayoutContext){
//         todo!()
//     }
// }
// /// `[A-z_][A-z_0-9]*`
// #[derive(Display)]
// pub struct Identifier {
//     str: String
// }
//
// impl Parser for Identifier {
//     fn get_error(&self, src: &mut Source) -> ParseError {
//         todo!()
//     }
//
//     fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
//         todo!()
//     }
//
//     fn parse(&self, src: &mut Source, context: &mut LayoutContext){
//         todo!()
//     }
// }