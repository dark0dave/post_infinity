use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::{
    common::parsers::{read_to_end, write_string},
    model::Model,
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/2da.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct TwoDA {
    #[bw(write_with = write_string)]
    #[br(parse_with = read_to_end)]
    pub data: String,
}

impl Model for TwoDA {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}
