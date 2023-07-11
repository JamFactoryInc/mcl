use std::fmt::{Display, Formatter};
use derive_more::Display;
use parse::{ParseError, Parser, Suggestion};
use src_in::Source;
use vm::LayoutContext;
use crate::grammar::identifier::McIdentifier;

#[derive(Display)]
pub enum NbtPathFragment {
    #[display(fmt = ".")]
    Accessor,
    #[display(fmt = "[{}]", "_0")]
    Index(isize),
    #[display(fmt = "[]")]
    Expand,
    #[display(fmt = "[{}]", "_0")]
    Lookup(Nbt),
    Field(McIdentifier)
}
impl Parser for NbtPathFragment {
    fn get_error(&self, src: &mut Source) -> ParseError {
        todo!()
    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        todo!()
    }

    fn parse(&self, src: &mut Source, context: &mut LayoutContext) {
        todo!()
    }
}

pub struct NbtPath {
    path: Vec<NbtPathFragment>
}
impl Display for NbtPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for element in &self.path {
            write!(f, "{}", element).unwrap();
        }
        Ok(())
    }
}

#[derive(Display)]
#[display(fmt = "{{{}, {}}}", key, val)]
pub struct Nbt {
    key: McIdentifier, val: Box<NbtElement>
}

pub struct NbtArrayElements {
    elements: Vec<NbtElement>
}
impl Display for NbtArrayElements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for element in &self.elements {
            write!(f, "{},", element).unwrap();
        }
        Ok(())
    }
}

#[derive(Display)]
pub enum NbtElement {
    #[display(fmt = "[{} {}]", array_type, elements)]
    Array { array_type: ArrayTypes, elements: NbtArrayElements},
    Object (Nbt),
    String (&'static str),
    #[display(fmt = "{}l", "_0")]
    Long (usize),
    Int (usize),
    #[display(fmt = "{}s", "_0")]
    Short (usize),
    #[display(fmt = "{}b", "_0")]
    Byte (usize),
    #[display(fmt = "{}d", "_0")]
    Double (f64),
    #[display(fmt = "{}f", "_0")]
    Float (f32),
}

#[derive(Display)]
pub enum ArrayTypes {
    #[display(fmt = "")]
    Any,
    #[display(fmt = "I; ")]
    Int,
    #[display(fmt = "L; ")]
    Long,
    #[display(fmt = "B; ")]
    Byte
}