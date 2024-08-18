use std::fmt::Debug;

use erased_serde::Serialize;

pub trait Model: Debug + Serialize {
    fn new(reader: &[u8]) -> Self
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
}
