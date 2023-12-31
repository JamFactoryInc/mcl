use crate::parse::MatchResult::*;
use crate::parse::{MatchResult, Optional, Parser, Stateful, StdSimd};
use std::intrinsics::likely;
use crate::grammar::literals::numeric::decimal::{Decimal, UDecimal};

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

impl<T: Parser> Stateful for RangeParserState<T> {
    type Out = Range<T>;

    fn new() -> Self {
        RangeParserState::<T> {
            parsed_first: None,
            seen_dot: false,
            state: RangeState::From,
            parser: T::State::new(),
        }
    }

    fn parse(&mut self, bytes: StdSimd) -> MatchResult<Self::Out> {
        match self.state {
            RangeState::From => match self.parser.parse(bytes) {
                Consumed => {
                    self.seen_dot = bytes == b'.';
                    Consumed
                },
                Parsed(ok) | Oops(ok) => {
                    self.seen_dot = bytes == b'.';
                    self.parsed_first = Some(ok);
                    self.state = RangeState::Middle;
                    self.parser = T::State::new();
                    Consumed
                },
                NoMatch(msg) => NoMatch(msg),
            },
            RangeState::Middle => {
                if likely(bytes == b'.') {
                    if self.seen_dot {
                        self.state = RangeState::To;
                    }
                    self.seen_dot = true;
                    Consumed
                } else {
                    NoMatch("range `..` expected")
                }
            }
            RangeState::To => self.parser.parse(bytes)
                .bubble(|ok: T::State::Out| Range::<T> {
                from: self.parsed_first.take() as Optional<T>,
                to: ok,
            })
        }
    }
}

pub struct Range<T: Parser> {
    from: Optional<T>,
    to: Optional<T>,
}
impl<T: Parser> Parser for Range<T> {
    type State = RangeParserState<T>;
}

pub type UDecimalRange = Range<UDecimal>;
pub type DecimalRange = Range<Decimal>;
