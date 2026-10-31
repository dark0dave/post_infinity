use char_array::CharArray;

use crate::common::zero_char_array::ZeroCharArray;

pub mod char_array;
pub mod feature_block;
pub mod header;
pub mod parsers;
pub mod strref;
pub mod types;
pub mod zero_char_array;
pub mod zero_char_vec;

pub type Resref = CharArray<8>;
pub type Signature = ZeroCharArray<4>;
pub type Version = ZeroCharArray<4>;
