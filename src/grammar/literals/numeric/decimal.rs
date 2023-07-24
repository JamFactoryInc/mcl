use crate::grammar::literals::numeric::unsigned::U64;
use crate::parse::MatchResult::*;
use crate::parse::{MatchResult, Parser, Stateful, StdSimd};

enum UnsignedFloatState {
    Before,
    After,
}

enum SignedFloatState {
    First,
    Before,
    After,
}

struct FloatParserState {
    parsed_first: u64,
    parser: <U64 as Parser>::State,
    state: UnsignedFloatState,
}

impl Stateful for FloatParserState {
    type Out = UDecimal;

    fn new() -> Self {
        FloatParserState {
            parsed_first: 0,
            parser: <U64 as Parser>::State::new(),
            state: UnsignedFloatState::Before,
        }
    }

    fn parse(&mut self, bytes: StdSimd) -> MatchResult<Self::Out> {
        match self.state {
             UnsignedFloatState::Before => self.parser.parse()
                .bubble_or_get(|num, result| {
                    if bytes == b'.' {
                        self.parsed_first = num;
                        self.parser = <U64 as Parser>::State::new();
                        self.state = UnsignedFloatState::After;
                        Consumed
                    } else {
                        result.bubble(|num| UDecimal {
                            before: num,
                            after: 0,
                        })
                    }
                }),
            UnsignedFloatState::After => self.parser.parse()
                .bubble(|num| UDecimal {
                    before: self.parsed_first.clone(),
                    after: num,
                }),
        }
    }
}

struct SignedFloatParserState {
    multiplicand: isize,
    parsed_first: u64,
    parser: <U64 as Parser>::State,
    state: SignedFloatState,
}

impl Stateful for SignedFloatParserState {
    type Out = Decimal;

    fn new() -> Self {
        SignedFloatParserState {
            multiplicand: 1,
            parsed_first: 0,
            parser: <U64 as Parser>::State::new(),
            state: SignedFloatState::Before,
        }
    }

    fn parse(&mut self, bytes: StdSimd) -> MatchResult<Self::Out> {
        match self.state {
            SignedFloatState::First => match bytes {
                b'0'..=b'9' => {
                    self.state = SignedFloatState::Before;
                    self.parser.parse(bytes)
                        .expect_consumed()
                }
                b'-' => {
                    self.state = SignedFloatState::Before;
                    self.multiplicand = -1;
                    Consumed
                }
                _ => NoMatch,
            },
            SignedFloatState::Before => self.parser.parse()
                .bubble_or_get(|num, result| {
                    if bytes == b'.' {
                        self.parsed_first = num;
                        self.parser = <U64 as Parser>::State::new();
                        self.state = SignedFloatState::After;
                        Consumed
                    } else {
                        result.bubble(|num| Decimal {
                            before: num as i64 * &self.multiplicand,
                            after: 0,
                        })
                    }
                }),
            SignedFloatState::After => self.parser.parse()
                .bubble(|num| Decimal {
                    before: &self.parsed_first * &self.multiplicand,
                    after: num,
                }),
        }
    }
}

pub struct UDecimal {
    before: u64,
    after: u64,
}
impl Parser for UDecimal {
    type State = UnsignedFloatState;
}

pub struct Decimal {
    before: i64,
    after: u64,
}
impl Parser for Decimal {
    type State = SignedFloatState;
}
