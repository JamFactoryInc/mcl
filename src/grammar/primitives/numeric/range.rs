use std::intrinsics::{likely, unlikely};
use crate::parse::{MatchResult, Optional, Parser, Stateful};
use crate::parse::MatchResult::{Consumed, NoMatch, Parsed};

enum RangeState {
    From,
    Middle,
    To,
}

struct RangeParserState<T: Parser> {
    parsed_first: Option<T>,
    seen_dot: bool,
    state: RangeState,
    parser: T::State,
}

impl<T: Parser> Stateful<T> for RangeParserState<T> {
    fn new() -> Self {
        RangeParserState::<T> {
            parsed_first: None,
            seen_dot: false,
            state: RangeState::From,
            parser: T::State::new(),
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<T> {
        match self.state {
            RangeState::From => {
                match self.parser.parse(byte) {
                    Consumed => {
                        self.seen_dot = byte == b'.';
                        Consumed
                    },
                    Parsed(ok) => {
                        self.seen_dot = byte == b'.';
                        self.parsed_first = Some(ok);
                        self.state = RangeState::Middle;
                        Consumed
                    }
                    _ => NoMatch
                }
            }
            RangeState::Middle => {
                if self.seen_dot {

                } else {

                }
            }
            RangeState::To => {

            }
        }
    }
}

pub struct Range<T: Parser> {
    from: Optional<T>,
    to: Optional<T>,
}



