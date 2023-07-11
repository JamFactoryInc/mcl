use derive_more::Display;
use crate::parse::{ParseError, Parser, Suggestion};
use crate::src_in::Source;
use crate::vm::LayoutContext;

/// `[_.-A-z0-9]+`
#[derive(Display)]
pub struct McIdentifier {
    str: String
}
impl Parser for McIdentifier {
    fn get_error(&self, src: &mut Source) -> ParseError {
        todo!()
    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    fn parse(&self, src: &mut Source, context: &mut LayoutContext){
        todo!()
    }
}
/// `[A-z_][A-z_0-9]*`
#[derive(Display)]
pub struct Identifier {
    str: String
}

impl Parser for Identifier {
    fn get_error(&self, src: &mut Source) -> ParseError {
        todo!()
    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    fn parse(&self, src: &mut Source, context: &mut LayoutContext){
        todo!()
    }
}