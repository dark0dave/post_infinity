use std::fmt::Debug;

use crate::tlk::Lookup;
use erased_serde::Serialize;

pub trait Model: Debug + Serialize {
    fn name(&self, lookup: &Lookup) -> String;
    fn new(buffer: &[u8]) -> Self
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
}
