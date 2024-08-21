use binrw::{
    io::{Cursor, Read, Seek},
    BinRead, BinReaderExt, BinResult, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::{common::char_array::CharArray, model::Model};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/2da.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct TwoDA {
    #[br(parse_with = |reader, _, _:()| read_to_end(reader))]
    pub data: CharArray,
}

fn read_to_end<R: Read + Seek>(reader: &mut R) -> BinResult<CharArray> {
    let mut buff = vec![];
    reader.read_to_end(&mut buff).unwrap_or_default();
    Ok(CharArray(buff))
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
