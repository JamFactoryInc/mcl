use std::intrinsics::likely;
use std::string::ToString;
use crate::grammar::primitives::numeric::unsigned::U64;
use crate::parse::{MatchResult, Parser, Stateful};
use crate::parse::MatchResult::{Consumed, NoMatch, Parsed};

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

impl Stateful<UDecimal> for FloatParserState {
    fn new() -> Self {
        FloatParserState {
            parsed_first: 0,
            parser: <U64 as Parser>::State::new(),
            state: UnsignedFloatState::Before,
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<UDecimal> {
        match self.state {
            UnsignedFloatState::Before => {
                match self.parser.parse() {
                    Consumed => Consumed,
                    Parsed(num) => {
                        if byte == b'.' {
                            self.parsed_first = num;
                            self.parser = <U64 as Parser>::State::new();
                            self.state = UnsignedFloatState::After;
                            Consumed
                        } else {
                            Parsed(UDecimal { before: num, after: 0 })
                        }
                    }
                    _ => NoMatch
                }
            }
            UnsignedFloatState::After => {
                match self.parser.parse() {
                    Consumed => Consumed,
                    Parsed(num) => Parsed(UDecimal { before: self.parsed_first.clone(), after: num }),
                    _ => NoMatch
                }
            }
        }
    }
}

struct SignedFloatParserState {
    negative: bool,
    parsed_first: u64,
    parser: <U64 as Parser>::State,
    state: SignedFloatState,
}

impl Stateful<Decimal> for SignedFloatParserState {
    fn new() -> Self {
        SignedFloatParserState {
            negative: false,
            parsed_first: 0,
            parser: <U64 as Parser>::State::new(),
            state: SignedFloatState::Before,
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<Decimal> {
        match self.state {
            SignedFloatState::First => {
                match byte {
                    b'0'..=b'9' => {
                        self.state = SignedFloatState::Before;
                        if likely(self.parser.parse(byte) == Consumed) {
                            Consumed
                        } else {
                            NoMatch
                        }
                    }
                    b'-' => {
                        self.state = SignedFloatState::Before;
                        self.negative = true;
                        Consumed
                    }
                    _ => NoMatch
                }
            }
            SignedFloatState::Before => {
                match self.parser.parse() {
                    Consumed => Consumed,
                    Parsed(num) => {
                        if byte == b'.' {
                            self.parsed_first = num;
                            self.parser = <U64 as Parser>::State::new();
                            self.state = SignedFloatState::After;
                            Consumed
                        } else {
                            Parsed(Decimal {
                                negative:
                                self.negative.clone(),
                                before: num, after: 0
                            })
                        }
                    }
                    _ => NoMatch
                }
            }
            SignedFloatState::After => {
                match self.parser.parse() {
                    Consumed => Consumed,
                    Parsed(num) => Parsed(Decimal {
                        negative: self.negative.clone(),
                        before: self.parsed_first.clone(),
                        after: num }),
                    _ => NoMatch
                }
            }
        }
    }
}

pub struct UDecimal {
    before: u64,
    after: u64,
}
impl Parser for UDecimal {
    type State = UnsignedFloatState;
    const ERR: fn() -> String = || "".to_string();
}

pub struct Decimal {
    negative: bool,
    before: u64,
    after: u64,
}
impl Parser for Decimal {
    type State = SignedFloatState;
    const ERR: fn() -> String = || "".to_string();
}