use std::fmt::Write;

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BinRead, BinWrite, Serialize, Deserialize)]
#[br(import{count: usize})]
pub struct CharArray(#[br(count = count)] pub Vec<u8>);

impl std::fmt::Display for CharArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.0.iter() {
            f.write_char(char::from(*element))?;
        }
        Ok(())
    }
}

impl From<&str> for CharArray {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}
