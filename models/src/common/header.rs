use std::{fmt::Debug, str};

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::char_array::CharArray;

// Generic header for this one
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Header {
    pub signature: CharArray<4>,
    pub version: CharArray<4>,
}
