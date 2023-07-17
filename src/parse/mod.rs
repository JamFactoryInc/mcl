pub mod unicode;
mod parse_error;
mod meta_parsers;

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::string::ToString;
use crate::src_in::Source;
use crate::util::RawString;
use MatchResult::*;
use crate::parse::parse_error::ParseError;

pub trait Stateful<T> {
    fn new() -> Self;
    fn parse(&mut self, byte: u8) -> MatchResult<T>;
}
pub struct ParserState<T> {
    state: usize,
    _p: PhantomData<T>
}

pub enum Optional<T : Parser> {
    Some(T),
    None
}
impl<T : Parser> Display for Optional<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(val) => write!(f, "{}", val),
            Self::None => write!(f, "")
        }
    }
}
impl<T : Parser> From<Option<T>> for Optional<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(val) => Optional::Some(val),
            None => Optional::None
        }
    }
}

pub enum MatchResult<Ok> {
    Parsed(Ok),
    Consumed,
    /// The last `Consumed` should have been `Parsed`
    Oops(Ok),
    NoMatch
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

pub trait Parser where Self: Display + Sized {
    type State: Stateful<Self>;
    const ERR: fn() -> String;

    fn get_error(src: &mut Source) -> ParseError {
        ParseError::from(src, || "something else (unspecified parse error)".to_string())
    }

    fn get_suggestions(_: &[u8]) -> Vec<Suggestion> {
        Vec::new()
    }

    fn parse<'a>(src: &mut Source) -> Result<Self, &'a ParseError> {
        let mut parser = Self::State::new();
        loop {
            match parser.parse(src.peek()) {
                Consumed => {
                    src.next();
                },
                Parsed(ok) => {
                    src.next();
                    break Ok(ok)
                },
                Oops(ok) => break Ok(ok),
                NoMatch => return Err(&Self::get_error(src))
            }
        }
    }
}
