mod meta_parsers;
pub mod parse_error;
pub mod unicode;

use crate::parse::parse_error::ParseError;
use crate::src_in::Source;
use crate::util::RawString;
use std::fmt::{Display, Formatter};
use std::intrinsics::likely;
use std::marker::PhantomData;
use MatchResult::*;

pub trait Stateful {
    type Out;
    fn new() -> Self;
    fn parse(&mut self, byte: u8) -> MatchResult<Self::Out>;
}
pub struct ParserState<T> {
    state: usize,
    _p: PhantomData<T>,
}

pub enum Optional<T: Parser> {
    Some(T),
    None,
}
impl<T: Parser> Display for Optional<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(val) => write!(f, "{}", val),
            Self::None => write!(f, ""),
        }
    }
}
impl<T: Parser> From<Option<T>> for Optional<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(val) => Optional::Some(val),
            None => Optional::None,
        }
    }
}

pub enum MatchResult<Ok> {
    Parsed(Ok),
    Consumed,
    /// The last `Consumed` should have been `Parsed`
    Oops(Ok),
    /// Indicates that the current byte results in a parse error
    NoMatch(&'static str),
}
impl<T> MatchResult<T> {

    pub fn expect_consumed<Target>(self) -> MatchResult<Target> {
        if likely(self == Consumed) {
            MatchResult::<Target>::Consumed
        } else {
            MatchResult::<Target>::NoMatch
        }
    }

    pub fn bubble_or_get<Mapped, Getter: Fn(T, &Self) -> MatchResult<Mapped>>(
        &self,
        mapper: Getter,
    ) -> MatchResult<Mapped> {
        match self {
            Consumed => Consumed,
            Oops(ok) => mapper(ok, &self),
            Parsed(ok) => mapper(ok, &self),
            NoMatch(msg) => NoMatch(msg),
        }
    }

    pub fn bubble<Mapped, Mapper: Fn(T) -> Mapped>(
        self,
        mapper: Mapper,
    ) -> MatchResult<Mapped> {
        match self {
            Consumed => Consumed,
            Oops(ok) => Oops(mapper(ok)),
            Parsed(ok) => Parsed(mapper(ok)),
            NoMatch(msg) => NoMatch(msg),
        }
    }

    pub fn bubble_msg<PreMap, Mapper: Fn(PreMap) -> T>(
        self,
        transformer: Mapper,
        msg: &'static str,
    ) -> MatchResult<T> {
        match self {
            Consumed => Consumed,
            Oops(ok) => Oops(transformer(ok)),
            Parsed(ok) => Parsed(transformer(ok)),
            NoMatch(_) => NoMatch(msg),
        }
    }

    pub fn consume<Action: Fn()>(action: Action) -> Self {
        action();
        Consumed
    }
}

pub struct ParsedExpression {
    tooltip: &'static str,
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
            bold_from,
        }
    }
}

pub struct Matchers {}
impl Matchers {
    pub fn repeat<F>(predicate: F, src: &mut Source) -> RawString
    where
        F: Fn(u8) -> bool,
    {
        let mut vec = Vec::new();

        while predicate(src.peek()) {
            vec.push(src.next().unwrap());
        }

        RawString::from(vec)
    }
}

pub trait Parser
where
    Self: Display + Sized,
{
    type State: Stateful<Out = Self>;

    fn get_suggestions(_: &[u8]) -> Vec<Suggestion> {
        Vec::new()
    }

    fn parse<'a>(src: &mut Source) -> Result<Self, &'a ParseError> {
        let mut parser = Self::State::new();
        loop {
            match parser.parse(src.peek()) {
                Consumed => {
                    src.next();
                }
                Parsed(ok) => {
                    src.next();
                    break Ok(ok);
                }
                Oops(ok) => break Ok(ok),
                NoMatch(err) => break Err(&src.err(err)),
            }
        }
    }
}
