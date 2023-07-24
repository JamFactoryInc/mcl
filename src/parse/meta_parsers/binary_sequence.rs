use crate::parse::*;
use derive_more::Display;

#[derive(Display)]
#[display(fmt = "{}{}", first, second)]
pub struct BinarySequence<_1, _2>
where
    _1: Parser,
    _2: Parser,
{
    first: _1,
    second: _2,
}

enum State {
    First,
    Second,
}

struct ParserState<_1: Parser, _2: Parser> {
    pub state_1: _1::State,
    pub state_2: _2::State,
    pub parsed_1: Option<_1>,
    pub state: State,
}

impl<_1: Parser, _2: Parser> Stateful<BinarySequence<_1, _2>> for ParserState<_1, _2> {
    fn new() -> Self {
        ParserState::<_1, _2> {
            state_1: _1::State::new(),
            state_2: _2::State::new(),
            parsed_1: None,
            state: State::First,
        }
    }

    fn parse(&mut self, bytes: StdSimd) -> MatchResult<Self::Out> {
        match self.state {
            State::First => match self.state_1.parse(bytes) {
                Consumed => Consumed,
                Oops(first) | Parsed(first) => {
                    self.parsed_1 = Some(first);
                    self.state = State::Second;
                    Consumed
                }
                NoMatch => NoMatch,
            },
            State::Second => match self.state_2.parse(bytes) {
                Consumed => Consumed,
                Oops(second) => {
                    self.state = State::Second;
                    Oops(BinarySequence {
                        first: self.parsed_1.unwrap(),
                        second,
                    })
                }
                Parsed(second) => {
                    self.state = State::Second;
                    Parsed(BinarySequence {
                        first: self.parsed_1.unwrap(),
                        second,
                    })
                }
                NoMatch => NoMatch,
            },
        }
    }
}

impl<_1: Parser, _2: Parser> Parser for BinarySequence<_1, _2> {
    type State = ParserState<_1, _2>;
    const ERR: fn() -> String = || format!("either {} or {}", _1::ERR(), _2::ERR());
}
