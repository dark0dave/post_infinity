use std::fmt::{Debug, Write};

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

// #[zerovec::make_varule(TestULE)]
// #[zerovec::derive(Serialize, Deserialize)]
// #[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
// pub struct Test<'data> {
//     f: u8,
//     #[serde(borrow)]
//     name: &'data str,
// }

// #[zerovec::make_varule(ARRULE)]
// #[zerovec::derive(Serialize, Deserialize)]
// #[derive(Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
// pub struct Arrr<'data> {
//     test: u8,
//     #[serde(borrow)]
//     goat: ZeroVec<'data, u8>,
//     #[serde(borrow)]
//     fish: VarZeroVec<'data, TestULE>
// }

// #[derive(Serialize, Deserialize)]
// pub struct ParentARR<'data> {
//     test: u8,
//     #[serde(borrow)]
//     goat: ZeroVec<'data, u8>,
//     #[serde(borrow)]
//     fish: VarZeroVec<'data, ARRULE>
// }
