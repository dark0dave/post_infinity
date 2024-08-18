use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/2da.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct TwoDA {
    #[br(parse_with = binrw::helpers::until_eof, map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
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
