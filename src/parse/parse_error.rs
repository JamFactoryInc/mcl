use crate::src_in::Source;


pub struct ParseError {
    line: usize,
    index: usize,
    absolute_index: usize,
    message_src: fn() -> String,
}
impl ParseError {
    pub fn from(src: &Source, msg: fn() -> String) -> ParseError {
        ParseError {
            line: src.line.clone(),
            index: src.get_index(),
            absolute_index: src.absolute_index.clone(),
            message_src: msg
        }
    }
}