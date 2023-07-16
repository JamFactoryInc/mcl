use derive_more::Display;
use crate::parse::*;

/// Metaparser that allows the first type `Opt` to be either skipped or parsed
///
/// `Req` must always be parsed
#[derive(Display)]
#[display(fmt = "{} {}", opt, req)]
pub struct AllowSkip<Opt: Parser, Req: Parser> {
    opt: Optional<Opt>,
    req: Req,
}

enum State {
    ParsedOpt,
    ParsedReq,
    Normal,
}

struct ParserState<Opt: Parser, Req: Parser> {
    pub opt_state: Opt::State,
    pub req_state: Req::State,
    pub parsed_req: Option<Req>,
    pub parsed_opt: Option<Opt>,
    pub state: State,
}

impl<Opt: Parser, Req: Parser> Stateful<AllowSkip<Opt, Req>> for ParserState<Opt, Req> {
    fn new() -> Self {
        ParserState::<Opt, Req> {
            opt_state: Opt::State::new(),
            req_state: Req::State::new(),
            parsed_req: None,
            parsed_opt: None,
            state: State::Normal,
        }
    }

    fn parse(&mut self, byte: u8) -> MatchResult<AllowSkip<Opt, Req>> {
        match self.state {
            State::ParsedReq => {
                match self.opt_state.parse(byte) {
                    Parsed(opt) => Parsed(AllowSkip {
                        opt: Optional::Some(opt),
                        req: self.parsed_req.unwrap(),
                    }),
                    Oops(opt) => Oops(AllowSkip {
                        opt: Optional::Some(opt),
                        req: self.parsed_req.unwrap(),
                    }),
                    Consumed => Consumed,
                    NoMatch => NoMatch,
                }
            }
            State::ParsedOpt => {
                match self.req_state.parse(byte) {
                    Parsed(req) => Parsed(AllowSkip {
                        opt: self.parsed_opt.into(),
                        req,
                    }),
                    Oops(req) => Oops(AllowSkip {
                        opt: self.parsed_opt.into(),
                        req,
                    }),
                    Consumed => Consumed,
                    NoMatch => NoMatch,
                }
            }
            State::Normal => {
                match self.opt_state.parse(byte.clone()) {
                    Consumed => {
                        match self.req_state.parse(byte) {
                            Consumed => Consumed,
                            Parsed(req) | Oops(req) => {
                                self.parsed_req = Some(req);
                                self.state = State::ParsedReq;
                                Consumed
                            }
                            NoMatch => NoMatch
                        }
                    },
                    Oops(opt) => match self.req_state.parse(byte) {
                        Consumed => {
                            self.parsed_opt = Some(opt);
                            self.state = State::ParsedOpt;
                            Consumed
                        },
                        Parsed(req) => {
                            Parsed(AllowSkip {
                                opt: Optional::Some(opt),
                                req,
                            })
                        },
                        Oops(req) => {
                            Oops(AllowSkip {
                                opt: Optional::Some(opt),
                                req,
                            })
                        }
                        NoMatch => NoMatch
                    }
                    Parsed(opt) => match self.req_state.parse(byte) {
                        Consumed => {
                            self.parsed_opt = Some(opt);
                            self.state = State::ParsedOpt;
                            Consumed
                        },
                        Oops(req) | Parsed(req) => Parsed(AllowSkip {
                            opt: Optional::Some(opt),
                            req,
                        }),
                        NoMatch => NoMatch,
                    },
                    NoMatch => match self.req_state.parse(byte) {
                        Consumed => {
                            self.state = State::ParsedOpt;
                            Consumed
                        },
                        Oops(req) => Oops(AllowSkip {
                            opt: Optional::None,
                            req,
                        }),
                        Parsed(req) => Parsed(AllowSkip {
                            opt: Optional::None,
                            req,
                        }),
                        NoMatch => NoMatch
                    }
                }
            }
        }
    }
}
impl<Opt: Parser, Req: Parser> Parser for AllowSkip<Opt, Req> {
    type State = ParserState<Opt, Req>;
    const ERR: fn() -> String = || format!("either {} or empty", Opt::ERR());

}