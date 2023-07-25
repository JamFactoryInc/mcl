use std::simd::u8x8;
use crate::utils::{Context, PathHasher};

pub enum MatchStatusVariant<T> {
    Parsed(T),
    Consumed,
    NoMatch(&'static str),
}

pub struct MatchStatus<T> {
    used_bytes: usize,
    status: MatchStatusVariant<T>
}
impl<T> MatchStatus<T> {
    pub fn parsed(used_bytes: usize, result: T) -> MatchStatus<T> {
        MatchStatus {
            used_bytes,
            status: MatchStatusVariant::Parsed(result),
        }
    }
    pub fn consumed(used_bytes: usize) -> MatchStatus<T> {
        MatchStatus {
            used_bytes,
            status: MatchStatusVariant::Consumed,
        }
    }
    pub fn no_match(used_bytes: usize, err_msg: &'static str) -> MatchStatus<T> {
        MatchStatus {
            used_bytes,
            status: MatchStatusVariant::NoMatch(err_msg),
        }
    }
}

pub struct Deserializer {
    current_context: u128,
}
impl Deserializer {
    pub fn reset_context(&mut self) {
        self.current_context = 0;
    }
    pub fn add_context(&mut self, path: &'static str) {
        self.current_context = self.current_context.saturating_add_signed(PathHasher::add_path(path))
    }
    pub fn remove_context(&mut self, path: &'static str) {
        self.current_context = self.current_context.saturating_add_signed(PathHasher::remove_path(path))
    }
}

pub trait DeserializerState {
    type Out;
    fn parse_chunk(&mut self, ds: Deserializer, chunk: u8x8) -> MatchStatus<Self::Out>;
}

pub struct Accumulator<T> {
    state: usize,
    accumulator: T,
}
impl<T> DeserializerState for Accumulator<T> {
    type Out = T;

    fn parse_chunk(&mut self, ds: Deserializer, chunk: u8x8) -> MatchStatus<Self::Out> {
        todo!()
    }
}

pub trait FromText {

}