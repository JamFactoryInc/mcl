use std::io::{Error, Read};
use std::io::ErrorKind::Interrupted;
use std::net::TcpStream;
use std::simd::u8x8;
use crate::parse::MatchResult;
use crate::parse::MatchResult::{Consumed, Parsed};
use crate::util::ascii_simd::mask::MaskAsciiUtils;
use crate::util::ascii_simd::simd::SimdUtils;

pub struct FieldDeserializer<T: Deserialize> {

}
impl<T: Deserialize> Deserializer for FieldDeserializer<T> {
    type Out = T;

    fn parse_chunk(&self, bytes: u8x8) -> MatchResult<Self::Out> {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }
}

pub struct StringDeserializer {

}
impl Deserializer for StringDeserializer {
    type Out = String;

    fn parse_chunk(&self, bytes: u8x8) -> MatchResult<Self::Out> {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }
}

pub struct UIntDeserializer {
    state: u32,
}
impl Deserializer for UIntDeserializer {
    type Out = u32;

    fn parse_chunk(&self, bytes: u8x8) -> MatchResult<Self::Out> {
        let (nums, mask) = bytes.digit();
        match mask.count_matches() {
            0 => Parsed(0, self.state.clone()),
            len @ 1..=7 => Parsed(len, )
            8 => Consumed()
        }
    }

    fn new() -> Self {
        todo!()
    }
}

pub struct IntDeserializer {

}
impl Deserializer for IntDeserializer {
    type Out = i32;

    fn parse_chunk(&self, bytes: u8x8) -> MatchResult<Self::Out> {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }
}

pub struct FloatDeserializer {

}
impl Deserializer for FloatDeserializer {
    type Out = f32;

    fn parse_chunk(&self, bytes: u8x8) -> MatchResult<Self::Out> {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }
}

pub struct VecDeserializer<T: Deserialize> {

}
impl<T: Deserialize> Deserializer for VecDeserializer<T> {
    type Out = Vec<T>;

    fn parse_chunk(&self, bytes: u8x8) -> MatchResult<Self::Out> {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }
}

pub trait Deserialize {
    type Deserializer: Deserializer;
}

pub trait Deserializer {
    type Out;
    fn parse_chunk(&self, bytes: u8x8) -> MatchResult<Self::Out>;
    fn new() -> Self;
}

pub struct JsonRPC {
    tcp: TcpStream
}

// indicates what the last byte parsed was and what we should expect as a result
enum ParseState {
    WithinObject,
    WithinArray,
    /// ```
    ///   "wow": "wow2"
    /// _^^
    /// { "wow": "wow2"
    /// _^^
    ExpectingKey,
    /// ```
    /// "wow": "wow2"
    /// _^^^^
    InKey,
    /// ```
    /// "wow": "wow2"
    ///     _^^
    EndKey,
    /// ```
    /// "wow": "wow2"
    ///      _^^
    /// "wow": []
    ///      _^^
    /// "wow": 123
    ///      _^^
    /// "wow": -123
    ///      _^^
    /// "wow": {"wow2": "wow3"}
    ///      _^^      _^^
    ExpectingValueWithinObject,
    /// ```
    /// "wow", "wow2"
    ///      _^^
    ExpectingValueWithinArray,
    /// ```
    ///  "wow": "wow2",}
    ///         __^^^^
    InStringWithinObject,
    /// ```
    ///  "wow": "wow2",]
    ///         __^^^^
    InStringWithinArray,
    /// ```
    /// "wow": 12345 }
    ///        _^^^^^^
    /// "wow": -12345]
    ///        _^^^^^^
    /// "wow": -12345
    ///        _^^^^
    InNumber,
    /// ```
    /// "wow2",
    ///      _^
    /// "wow2" }
    ///      _^^
    /// "wow2" ]
    ///      _^^
    EndValue,
}

impl JsonRPC {

    pub fn parse_chunk(bytes: u8x8) -> MatchResult<> {

    }

    pub fn parse_message(&mut self) -> Result<RequestMessage, Error> {
        let mut byte_buffer = [0u8; 128];
        loop {
            let len = match self.tcp.read(&mut byte_buffer) {
                Ok(len) => len,
                Err(err) if err.kind() == Interrupted => continue,
                Err(err) => return Err(err),
            };


        }

        todo!()
    }
}

pub enum RequestMessage {

}


