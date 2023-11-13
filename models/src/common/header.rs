use serde::{Deserialize, Serialize};

use super::fixed_char_array::FixedCharSlice;

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Header<const N: usize, const M: usize> {
    pub signature: FixedCharSlice<N>,
    pub version: FixedCharSlice<M>,
}
