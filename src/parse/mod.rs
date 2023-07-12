pub mod unicode;

use std::fmt::{Display};
use derive_more::Display;
use crate::src_in::Source;
use crate::translate::bytecode::Instr;
use crate::util::RawString;
use crate::vm::LayoutContext;

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

pub struct ParseError {
    line: usize,
    index: usize,
    absolute_index: usize,
    message: &'static str,
}
impl ParseError {
    pub fn from(src: &Source, msg: &'static str) -> ParseError {
        ParseError {
            line: src.line,
            index: src.get_index(),
            absolute_index: src.absolute_index,
            message: msg
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

pub trait Parser<Out = Instr> where Self: Display {
    fn get_error(&self, src: &mut Source) -> ParseError {
        ParseError::from(src, &format!("{}", self))
    }
    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        Vec::new()
    }
    fn parse<'a>(&self, src: &mut Source) -> Result<Out, ParseError>;
}

pub struct LiteralParser {
    cursor: usize,
    options: Vec<Vec<u8>>,
}

#[derive(Display, Clone, Copy)]
pub enum Wow {
    One,
    Two,
}
impl Parser for Wow {
    fn get_error(&self, _: &mut Source) -> ParseError {
        todo!()
    }

    fn get_suggestions(&self, _: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    // todo: refactor this to use a proc macro so we can do pre-computed logic instead of array garbage
    fn parse<'a>(&self, src: &mut Source, context: &'a mut LayoutContext) {
        let options = [(Wow::One, b"a"),(Wow::Two,  b"b")];
        let mut cursor = 0;
        let mut outstanding = options.len();
        let mut next;
        let mut found = None;

        while outstanding > 0 {
            for option in &options {
                next = src.next().unwrap();

                // for each option, check if the current index-to-be-checked matches the given char
                if option.1.len() >= cursor + 1 && option.1[cursor] == next {
                    // if we've checked each character in this option
                    if cursor == option.1.len() {
                        outstanding -= 1;
                        if outstanding == 0 {
                            // if this was the last outstanding option, return it
                            //context.add(Instr::Literal())
                            return
                        } else {
                            // if there are other outstanding options, save this one but keep going
                            found = Some(option.0);
                        }
                    }
                } else {
                    outstanding -= 1;
                }
            }
            cursor += 1;
        }

        todo!()
    }
}


macro_rules! literal_options {
    ( $name:ident { $($rule:ident: $lit:literal),+ } ) => {
        enum $name {
            $($rule),+
        }

        impl LiteralOptions<$name> for $name {
            fn get_error_message() -> &'static str {
                 concat!("expected one of [" $("'",  $lit, "'"),+ "]")
            }

            fn get_suggestions(partial: &str) -> Vec<&str> {
                vec![$($lit),+]
            }

            fn parse(&mut Vec) -> $name {
                todo!()
            }
        }
    };
}



pub struct OmniParser {

}