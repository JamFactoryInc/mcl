use crate::src_in::Source;

pub struct ParseError {
    pub line: usize,
    pub index: usize,
    pub absolute_index: usize,
    pub message: &'static str,
}
impl ParseError {
    pub fn from(src: &Source, msg: &'static str) -> ParseError {
        ParseError {
            line: src.line.clone(),
            index: src.get_index(),
            absolute_index: src.absolute_index.clone(),
            message: msg,
        }
    }
}
