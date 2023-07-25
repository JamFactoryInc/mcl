use std::hash::Hash;

pub(crate) struct PathHasher {
    hash: u64,
    accumulator: i64,
    m: i64,
    p_pow: i64,
}
impl PathHasher {
    pub const fn new() -> PathHasher {
        PathHasher {
            hash: 0,
            accumulator: 0,
            m: 1_000_000_007i64,
            p_pow: 1,
        }
    }
    const fn path_op<const SIGN: i64>(string: &'static str) -> i128 {
        let mut hasher = PathHasher::new();
        let bytes = string.as_bytes();
        let mut i = bytes.len();
        let mut accumulator = 0i128;
        while i != 0 {
            let byte = &bytes[bytes.len()];
            if *byte == b'.' {
                hasher = PathHasher::new();
            } else {
                hasher = PathHasher::hash(hasher, byte);
                accumulator += (hasher.accumulator * SIGN) as i128
            }
        }
        SIGN as i128 * (accumulator as i128)
    }
    pub(crate) const fn add_path(string: &'static str) -> i128 {
        PathHasher::path_op::<1>(string)
    }
    pub(crate) const fn remove_path(string: &'static str) -> i128 {
        PathHasher::path_op::<-1>(string)
    }
    pub(crate) const fn hash(mut hasher: PathHasher, byte: &u8) -> PathHasher {
        if *byte != 0 {
            hasher.accumulator = (hasher.accumulator + ((byte - b'a') as i64 + 1) * hasher.p_pow) % hasher.m;
            hasher.p_pow = (hasher.p_pow * 32) % hasher.m;
            PathHasher {
                hash: hasher.hash,
                accumulator: hasher.accumulator,
                m: hasher.m,
                p_pow: hasher.p_pow,
            }
        } else {
            hasher
        }
    }
}

pub struct Context {
    context: u128
}
impl Context {
    const fn from(path: &'static str) -> u128 {
        PathHasher::add_path(path) as u128
    }
}