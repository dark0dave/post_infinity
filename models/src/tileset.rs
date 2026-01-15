use binrw::{BinReaderExt, BinWrite, binread, io::Cursor};
use serde::{Deserialize, Serialize};

use crate::model::Model;

#[binread]
#[derive(Debug, PartialEq, BinWrite, Serialize, Deserialize)]
pub struct Tileset {
    #[serde(skip)]
    #[br(temp)]
    #[bw(ignore)]
    length: u32,

    #[br(count=length)]
    pub data: Vec<u8>,
}

impl Model for Tileset {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {err:?}, dumping buffer: {buffer:?}");
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}
