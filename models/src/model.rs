use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait Model: Debug + Serialize {
    fn new(reader: &[u8]) -> Self
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Parseable<'a>:
    Serialize + Deserialize<'a> + TryFrom<&'a [u8]> + TryInto<Vec<u8>>
{
}
