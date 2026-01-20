use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use zerovec::make_ule;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, BinRead, BinWrite, Serialize, Deserialize,
)]
#[make_ule(StrrefLE)]
pub struct Strref(pub u32);
