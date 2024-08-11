use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::{model::Model, tlk::Lookup};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/ids.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Ids {
    #[br(parse_with = binrw::helpers::until_eof, map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub data: String,
}

impl Model for Ids {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}
