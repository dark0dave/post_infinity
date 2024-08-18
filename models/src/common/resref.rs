use std::fmt::Write;

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Resref(#[br(count = 8)] pub Vec<u8>);

impl std::fmt::Display for Resref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.0.iter() {
            f.write_char(char::from(*element))?;
        }
        Ok(())
    }
}

impl From<String> for Resref {
    fn from(value: String) -> Self {
        Self(value.into_bytes())
    }
}
