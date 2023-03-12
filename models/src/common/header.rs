use serde::Serialize;

use super::fixed_char_array::FixedCharSlice;

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct Header {
    pub signature: FixedCharSlice<4>,
    pub version: FixedCharSlice<4>,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct CustomHeader<const N: usize, const M: usize> {
    pub signature: FixedCharSlice<N>,
    pub version: FixedCharSlice<M>,
}
