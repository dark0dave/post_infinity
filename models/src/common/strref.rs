use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Strref(pub u32);
