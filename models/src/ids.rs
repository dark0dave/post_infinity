use binrw::{
    io::{Cursor, Read, Seek},
    BinRead, BinReaderExt, BinResult, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/ids.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Ids {
    #[br(parse_with = |reader, _, _:()| read_to_end(reader))]
    #[bw(map = |x| x.as_bytes())]
    pub data: String,
}

fn read_to_end<R: Read + Seek>(reader: &mut R) -> BinResult<String> {
    let mut buff = String::new();
    reader.read_to_string(&mut buff).unwrap_or_default();
    Ok(buff)
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
