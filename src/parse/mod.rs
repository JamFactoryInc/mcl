pub mod unicode;

use std::fmt::{Display, Formatter, write};
use std::string::ToString;
use derive_more::Display;
use crate::parse::Optional::*;
use crate::src_in::Source;
use crate::util::RawString;
use MatchResult::*;
use crate::parse::Either::{A, B};


pub enum MatchResult<'a, Ok> {
    Parsed(Ok),
    Consumed,
    Error(&'a ParseError),
    /// The last `Consumed` should have been `Parsed`
    Oops(Ok),
}

/// Returns the successful parse result of the two options
///
/// If they both succeed, returns the longer option. If they are the same length, returns A
#[derive(Display)]
pub enum Either<A, B> where A: Display + Parser<Out = A>, B: Display + Parser<Out = B> {
    A(A),
    B(B),
}
impl<A: Display + Parser<Out = A>, B: Display + Parser<Out = B>> Either<A, B> {
    fn finish<'a, T : Parser, G>(index: usize, src: &mut Source, get_t: G, err_result: Result<Self, &'a ParseError>) -> Result<Self, &'a ParseError>
            where G: Fn(<T as Parser>::Out) -> Self {
        let mut index = index;
        loop {
            index += match T::parse_arbitrary(src.peek(), &index) {
                Consumed => { src.next(); 1 },
                Oops(a) => break Ok(get_t(a)),
                Parsed(a) => { src.next(); break Ok(get_t(a)) },
                Error(_) => { src.next(); break err_result }
            }
        }
    }

    fn finish_b<'a, E>(index: usize, src: &mut Source, err_result: Result<Self, &'a ParseError>) -> Result<Self, &'a ParseError>
        where E: Fn() -> Result<Self, &'a ParseError> {
        let mut index = index;
        loop {
            index += match B::parse_arbitrary(src.peek(), &index) {
                Consumed => { src.next(); 1 },
                Oops(b) => break Ok(B(b)),
                Parsed(b) => { src.next(); break Ok(B(b)) },
                Error(_) => { src.next(); break err_result }
            }
        }
    }
}
impl<A: Display + Parser<Out = A>, B: Display + Parser<Out = B>> Parser for Either<A, B> {
    type Out = Self;
    const ERR: fn() -> String = || format!("either {} or {}", A::ERR(), B::ERR());

    fn parse_arbitrary<'a>(byte: u8, index: &usize) -> MatchResult<'a, Self::Out> {
        match A::parse_arbitrary(byte, index) {
            Consumed => ,
            Parsed(a) => match B::parse_arbitrary(byte, index) {
                Consumed => Consumed,
                Parsed(b) => Parsed(a),

            },
            Error(a_err) => B::parse_arbitrary(src, index)
        }
    }

    fn parse<'a>(src: &mut Source) -> Result<Self::Out, &'a ParseError> {
        let mut index= 0;

        loop {
            // check A
            index += match A::parse_arbitrary(src.peek(), &index) {
                // A consumed. Check B
                Consumed => match B::parse_arbitrary(src.peek(), &index) {
                    // A and B consumed
                    Consumed => {
                        src.next();
                        1
                    },
                    // B parsed. Try parse A & return longer result
                    Parsed(b) | Oops(b) => break Self::finish(index, src, |a| A(a), Ok(B(b))),
                    // B failed. Try parse A
                    Error(_) => Self::finish(index, src, |a| A(a), Err(&Self::get_error(src)))
                },
                // A should have parsed. Check B (B either consumed or just started)
                Oops(a) => match B::parse_arbitrary(src.peek(), &index) {
                    Consumed => break loop {
                        index += match A::parse_arbitrary(src.peek(), &index) {
                            Consumed => {
                                src.next();
                                1
                            },
                            Oops(a) => break Ok(A(a)),
                            Parsed(a) => {
                                src.next();
                                break Ok(A(a))
                            },
                            Error(_) => {
                                src.next();
                                break Ok(B(b))
                            }
                        }
                    },
                    // B also should have parsed, but A wins.
                    Oops(_) => break Ok(A(a)),
                    // B Parsed, but A technically parsed last index
                    Parsed(_) => {
                        src.next();
                        break Ok(A(a));
                    },
                    // B failed
                    Error(_) => {
                        src.next();
                        break Ok(A(a))
                    }
                },
                // A parsed. Try parse B & return longer result
                Parsed(a) => break loop {
                    index += match B::parse_arbitrary(src.peek(), &index) {
                        Consumed => {
                            src.next();
                            1
                        },
                        Oops(b) => break Ok(B(b)),
                        Parsed(b) => {
                            src.next();
                            break Ok(B(b))
                        },
                        Error(_) => {
                            src.next();
                            break Ok(A(a))
                        }
                    }
                },
                // A failed. Try parse B
                Error(_) => {
                    break loop {
                        index += match B::parse_arbitrary(src.peek(), &index) {
                            Consumed => {
                                src.next();
                                1
                            },
                            Oops(b) => break Ok(B(b)),
                            // Parsed A
                            Parsed(b) => {
                                src.next();
                                break Ok(B(b));
                            },
                            // A failed
                            Error(_) => {
                                src.next();
                                break Err(&Self::get_error(src))
                            }
                        }
                    }
                },
            }
        }
    }
}

#[derive(Display)]
pub enum Optional<T> where T: Display {
    Filled(T),
    #[display(fmt = "")]
    Empty
}
impl<T: Display + Parser<Out = T>> Parser for Optional<T> {
    type Out = Self;
    const ERR: fn() -> String = || format!("either {} or empty", T::ERR());

    fn test(src: &mut Source) -> bool {
        T::test(src)
    }

    fn parse<'a>(src: &mut Source) -> Result<Self::Out, ParseError> {
        match T::parse(src) {
            Ok(x) => Ok(Filled(x)),
            Err(_) => Ok(Empty)
        }
    }
}

pub struct ParsedExpression {
    tooltip : &'static str,

}

pub struct Suggestion {
    display: String,
    contents: String,
    bold_from: usize,
}
impl Suggestion {
    pub fn of(display: String, contents: String, bold_from: usize) -> Suggestion {
        Suggestion {
            display,
            contents,
            bold_from
        }
    }
}

pub struct ParseError {
    line: usize,
    index: usize,
    absolute_index: usize,
    message_src: fn() -> String,
}
impl ParseError {
    pub fn from(src: &Source, msg: fn() -> String) -> ParseError {
        ParseError {
            line: src.line.clone(),
            index: src.get_index(),
            absolute_index: src.absolute_index.clone(),
            message_src: msg
        }
    }
}

pub struct Matchers {}
impl Matchers {
    pub fn repeat<F>(predicate: F, src: &mut Source) -> RawString
        where F: Fn(u8) -> bool {
        let mut vec = Vec::new();

        while predicate(src.peek()) {
            vec.push(src.next().unwrap());
        }

        RawString::from(vec)
    }
}

pub trait Parser where Self: Display {
    type Out;
    const ERR: fn() -> String;

    fn get_error(src: &mut Source) -> ParseError {
        ParseError::from(src, || "parse error".to_string())
    }
    fn get_suggestions(_: &[u8]) -> Vec<Suggestion> {
        Vec::new()
    }
    fn parse_arbitrary<'a>(byte: u8, index: &usize) -> MatchResult<'a, Self::Out>;
    fn parse<'a>(src: &mut Source) -> Result<Self::Out, &'a ParseError>;
}

pub struct LiteralParser {
    cursor: usize,
    options: Vec<Vec<u8>>,
}

macro_rules! literal_options {
    ( $name:ident { $($rule:ident: $lit:literal),+ } ) => {
        enum $name {
            $($rule),+
        }

        impl LiteralOptions<$name> for $name {
            fn get_error_message() -> &'static str {
                 concat!("expected one of [" $("'",  $lit, "'"),+ "]")
            }

            fn get_suggestions(partial: &str) -> Vec<&str> {
                vec![$($lit),+]
            }

            fn parse(&mut Vec) -> $name {
                todo!()
            }
        }
    };
}



pub struct OmniParser {

}