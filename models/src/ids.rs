use binrw::{BinRead, BinReaderExt, BinWrite, io::Cursor};
use serde::{Deserialize, Serialize};

use crate::{
    common::parsers::{read_to_end, write_string},
    model::Model,
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/ids.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Ids {
    #[bw(write_with = write_string)]
    #[br(parse_with = read_to_end)]
    pub data: String,
}

impl Model for Ids {
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
