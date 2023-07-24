use crate::parse::meta_parsers::either::Either::{A, B};
use crate::parse::meta_parsers::either::State::{Normal, ParsingA, ParsingB};
use crate::parse::*;
use derive_more::Display;

/// Returns the successful parse result of the two options
///
/// If they both succeed, returns the longer option. If they are the same length, returns A
#[derive(Display)]
pub enum Either<A, B>
where
    A: Parser,
    B: Parser,
{
    A(A),
    B(B),
}

enum State {
    ParsingA,
    ParsingB,
    Normal,
}

struct ParserState<A: Parser, B: Parser> {
    pub a_state: A::State,
    pub b_state: B::State,
    pub parsed_a: Option<A>,
    pub parsed_b: Option<B>,
    pub state: State,
}

impl<A: Parser, B: Parser> Stateful<Either<A, B>> for ParserState<A, B> {
    fn new() -> Self {
        ParserState::<A, B> {
            a_state: A::State::new(),
            b_state: B::State::new(),
            parsed_a: None,
            parsed_b: None,
            state: State::Normal,
        }
    }

    fn parse(&mut self, bytes: StdSimd) -> MatchResult<Self::Out> {
        match self.state {
            ParsingA => match self.a_state.parse(bytes) {
                Consumed => Consumed,
                Parsed(a) => Parsed(A(a)),
                Oops(a) => Oops(A(a)),
                NoMatch => match self.parsed_a.take() {
                    Some(a) => Parsed(A(a)),
                    None => NoMatch,
                },
            },
            ParsingB => match self.b_state.parse(bytes) {
                Consumed => Consumed,
                Parsed(b) => Parsed(B(b)),
                Oops(b) => Oops(B(b)),
                NoMatch => match self.parsed_b.take() {
                    Some(b) => Parsed(B(b)),
                    None => NoMatch,
                },
            },
            Normal => match self.a_state.parse(bytes.clone()) {
                Consumed => match self.b_state.parse(bytes) {
                    Consumed => Consumed,
                    Parsed(b) | Oops(b) => {
                        self.parsed_b = Some(b);
                        self.state = ParsingA;
                        Consumed
                    }
                    NoMatch => {
                        self.state = ParsingA;
                        Consumed
                    }
                },
                Parsed(a) => match self.b_state.parse(bytes) {
                    Consumed => {
                        self.parsed_a = Some(a);
                        Consumed
                    }
                    Oops(b) => Oops(B(b)),
                    _ => Parsed(A(a)),
                },
                Oops(a) => match self.b_state.parse(bytes) {
                    Consumed => {
                        self.parsed_a = Some(a);
                        Consumed
                    }
                    _ => Parsed(A(a)),
                },
                NoMatch => match self.b_state.parse(bytes) {
                    Consumed => {
                        self.state = ParsingB;
                        Consumed
                    }
                    Parsed(b) => Parsed(B(b)),
                    Oops(b) => Oops(B(b)),
                    NoMatch => NoMatch,
                },
            },
        }
    }
}

impl<A: Parser, B: Parser> Parser for Either<A, B> {
    type State = ParserState<A, B>;
    const ERR: fn() -> String = || format!("either {} or {}", A::ERR(), B::ERR());
}
