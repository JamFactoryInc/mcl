use derive_more::Display;
use crate::parse::{Parser, Stateful};
use crate::parse::meta_parsers::allow_skip::AllowSkip;
use crate::parse::meta_parsers::either::Either;

#[derive(Display)]
pub enum LeftXorRight<Left, M, Right> where Left: Parser, M: Parser, Right: Parser {
    #[display(fmt = "{}{}", _0, _1)]
    Left(Left, M),
    #[display(fmt = "{}{}", _0, _1)]
    Right(M, Right),
}

enum State {
    /// parsing the left component
    Left,
    /// parsing the left and middle components after parsing the left
    MiddleLeft,
    MiddleNoLeft,
    Right,
}

struct ParserState<Left: Parser, M: Parser, Right: Parser> {
    pub skip_l_state: AllowSkip<Left, M>,
    
    pub state: State,
}

impl<Left: Parser, M: Parser, Right: Parser> Stateful<LeftXorRight<Left, M, Right>> for ParserState<Left, M, Right> {
    fn new() -> Self {
        ParserState::<A, B> {
            l_state: Left::State::new(),
            m_state: M::State::new(),
            r_state: Right::State::new(),
            parsed_l: None,
            parsed_m: None,
            parsed_r: None,
            state: State::ParsingLeft,
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<Either<A, B>> {
        match self.state {
            ParsingA => {
                match self.a_state.parse(byte) {
                    Consumed => Consumed,
                    Parsed(a) => Parsed(A(a)),
                    Oops(a) => Oops(A(a)),
                    NoMatch => match self.parsed_a.take() {
                        Some(a) => Parsed(A(a)),
                        None => NoMatch
                    }
                }
            },
            ParsingB => {
                match self.b_state.parse(byte) {
                    Consumed => Consumed,
                    Parsed(b) => Parsed(B(b)),
                    Oops(b) => Oops(B(b)),
                    NoMatch => match self.parsed_b.take() {
                        Some(b) => Parsed(B(b)),
                        None => NoMatch
                    }
                }
            },
            Normal => {
                match self.a_state.parse(byte.clone()) {
                    Consumed => match self.b_state.parse(byte) {
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
                    Parsed(a) => match self.b_state.parse(byte) {
                        Consumed => {
                            self.parsed_a = Some(a);
                            Consumed
                        }
                        Oops(b) => Oops(B(b)),
                        _ => Parsed(A(a)),
                    }
                    Oops(a) => match self.b_state.parse(byte) {
                        Consumed => {
                            self.parsed_a = Some(a);
                            Consumed
                        }
                        _ => Parsed(A(a)),
                    }
                    NoMatch => match self.b_state.parse(byte) {
                        Consumed => {
                            self.state = ParsingB;
                            Consumed
                        },
                        Parsed(b) => Parsed(B(b)),
                        Oops(b) => Oops(B(b)),
                        NoMatch => NoMatch
                    }
                }
            }
        }
    }
}

impl<A: Parser, B: Parser> Parser for Either<A, B> {
    type State = ParserState<A, B>;
    const ERR: fn() -> String = || format!("either {} or {}", A::ERR(), B::ERR());
}