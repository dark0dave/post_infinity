use char_array::CharArray;

pub mod char_array;
pub mod feature_block;
pub mod header;
pub mod parsers;
pub mod strref;
pub mod types;

pub type Resref = CharArray<8>;
