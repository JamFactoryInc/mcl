use std::fmt::{Display, Formatter};

pub struct RawString {
    bytes: Vec<u8>,
}

impl RawString {
    pub fn from(src: Vec<u8>) -> RawString {
        RawString { bytes: src }
    }
}

impl Display for RawString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for byte in &self.bytes {
            write!(f, "{}", *byte as char).expect("");
        }
        Ok(())
    }
}

pub trait Generate<T = Self> {
    fn generate() -> T;
}
