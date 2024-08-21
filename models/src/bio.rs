use binrw::{
    io::{Cursor, Read, Seek},
    BinRead, BinReaderExt, BinResult, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::{common::char_array::CharArray, model::Model};

#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Biography(#[br(parse_with = |reader, _, _:()| read_to_end(reader))] pub CharArray);

fn read_to_end<R: Read + Seek>(reader: &mut R) -> BinResult<CharArray> {
    let mut buff = vec![];
    reader.read_to_end(&mut buff).unwrap_or_default();
    Ok(CharArray(buff))
}

impl Model for Biography {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0 .0.to_vec()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use binrw::io::{BufReader, Read};
    use std::fs::File;

    #[test]
    fn read_biography() {
        let file = File::open("fixtures/test.bio").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let bio = Biography::new(&buffer);
        assert_eq!(bio.0, CharArray(buffer));
    }
}
