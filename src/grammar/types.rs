pub struct UDecimal { int: u32, dec: u32 }
pub struct Decimal { int: i32, dec: i32 }
pub struct Range { from: isize, to: isize }
pub struct URange { from: usize, to: usize }
pub struct DecimalRange { from: Decimal, to: Decimal }
pub struct UDecimalRange { from: UDecimal, to: UDecimal }