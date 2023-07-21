use std::io::{Error, Read};
use std::io::ErrorKind::Interrupted;
use std::net::TcpStream;
use std::simd::u8x8;
use crate::parse::MatchResult;


pub struct JsonRPC {
    tcp: TcpStream
}

// indicates what the last byte parsed was and what we should expect as a result
enum ParseState {
    /// ```
    ///  "wow": "wow2"
    /// ^
    ExpectingKey,
    /// ```
    ///  "wow": "wow2"
    ///  ^^^^
    InKey,
    /// ```
    ///  "wow": "wow2"
    ///      ^^
    EndKey,
    /// ```
    /// "wow": "wow2"
    ///       ^
    /// "wow": []
    ///       ^
    /// "wow": 123
    ///       ^
    /// "wow": {"wow2": "wow3"}
    ///       ^        ^
    ExpectingValue,
    /// ```
    ///  "wow": "wow2"
    ///         ^^^^^
    InString,
    /// ```
    ///  "wow": "wow2"
    ///              ^
    EndString,
    /// ```
    ///  {"wow": "wow2"}
    ///                ^
    EndObject
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


