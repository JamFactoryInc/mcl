use crate::grammar::types::Decimal;

pub enum CoordinateFragment {
    Literal(Decimal),
    Relative(Decimal),
    Rotational(Decimal),
}

pub struct Coordinate {
    x: CoordinateFragment,
    y: CoordinateFragment,
    z: CoordinateFragment,
}

pub struct Rotation {
    x: CoordinateFragment,
    y: CoordinateFragment,
}
