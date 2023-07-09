pub mod unicode;

use std::fmt::{Display, format};
use crate::src_in::Source;

pub struct ParsedExpression {
    tooltip : &'static str,

}

pub struct Suggestion {
    display: &'static str,
    contents: &'static str,
    bold_from: usize,
}
impl Suggestion {
    pub fn of(display: &'static str, contents: &'static str, bold_from: usize) -> Suggestion {
        Suggestion {
            display,
            contents,
            bold_from
        }
    }
}

pub struct ParseError {
    line: usize,
    index: isize,
    absolute_index: usize,
    message: &'static str,
}
impl ParseError {
    pub fn from(src: &Source, msg: &'static str) -> ParseError {
        ParseError {
            line: src.line,
            index: src.index,
            absolute_index: src.absolute_index,
            message: msg
        }
    }
}

pub trait Parser<T : Display> {
    fn get_error(&self, src: &mut Source) -> ParseError;
    fn get_suggestions(&self, partial: &str) -> Vec<Suggestion>;
    fn parse(&self, src: &mut Source) -> Option<T>;

    fn repeat<F>(predicate: F, src: &mut Source) -> Vec<u8>
        where F: Fn(u8) -> bool {
        let mut vec = Vec::new();

        while predicate(src.peek()) {
            vec.push(src.next().unwrap());
        }

        return vec;
    }
}

pub struct LiteralParser {
    cursor: usize,
    options: Vec<Vec<u8>>,
}

pub enum Wow {
    One,
    Two,
}
impl Parser<Wow> for LiteralParser {
    fn get_error(&self, src: &Source) -> ParseError {
        todo!()
    }

    fn get_suggestions(&self, partial: &str) -> Vec<Wow> {
        todo!()
    }

    // todo: refactor this to use a proc macro so we can do pre-computed logic instead of array garbage
    fn parse(&self, src: &mut Source) -> Option<Wow> {
        let options = [(Wow::One, "a"),(Wow::Two,  b"b")];
        let mut cursor = 0;
        let mut outstanding = options.len();
        let mut next;
        let mut found = None;

        while outstanding > 0 {
            for option in options {
                next = src.next().unwrap();

                // for each option, check if the current index-to-be-checked matches the given char
                if option[1].len() >= cursor + 1 && option[1][cursor] == next {
                    // if we've checked each character in this option
                    if cursor == option[1].len() {
                        outstanding -= 1;
                        if outstanding == 0 {
                            // if this was the last outstanding option, return it
                            return Some(option[0]);
                        } else {
                            // if there are other outstanding options, save this one but keep going
                            found = Some(option[0]);
                        }
                    }
                } else {
                    outstanding -= 1;
                }
            }
            cursor += 1;
        }

        return found
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