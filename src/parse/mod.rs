pub mod unicode;

use std::fmt::{Display, Formatter, write};
use derive_more::Display;
use crate::parse::Optional::*;
use crate::src_in::Source;
use crate::translate::bytecode::Instr;
use crate::util::RawString;
use crate::vm::LayoutContext;

pub enum Optional<T> {
    Filled(T),
    Empty
}
impl<T: Display> Display for Optional<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Some(x) => write!(f, "{x}").unwrap(),
            None => ()
        };
        Ok(())
    }
}
impl<T: Display + Parser> Parser for Optional<T> {
    type Out = Self;

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
    message: &'static str,
}
impl ParseError {
    pub fn from(src: &Source, msg: &'static str) -> ParseError {
        ParseError {
            line: src.line,
            index: src.get_index(),
            absolute_index: src.absolute_index,
            message: msg
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
    fn get_error(src: &mut Source) -> ParseError {
        ParseError::from(src, "parse error")
    }
    fn get_suggestions(partial: &[u8]) -> Vec<Suggestion> {
        Vec::new()
    }
    fn test(src: &mut Source) -> bool;
    fn parse<'a>(src: &mut Source) -> Result<Self::Out, ParseError>;
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