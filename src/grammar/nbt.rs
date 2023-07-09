use crate::grammar::identifier::McIdentifier;

pub struct Nbt {
    key: McIdentifier, val: Box<NbtElement>
}

pub enum NbtElement {
    Array { array_type: ArrayTypes, elements: Vec<NbtElement> },
    Object (Nbt),
    String (str),
    Long (usize),
    Int (usize),
    Short (usize),
    Byte (usize),
    Double (f64),
    Float (f32),
}

pub enum ArrayTypes {
    Any,
    Int,
    Long,
    Byte
}