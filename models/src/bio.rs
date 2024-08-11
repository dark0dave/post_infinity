use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::{model::Model, tlk::Lookup};

#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Biography(
    #[bw(map = |x| x.as_bytes())]
    #[br(parse_with = binrw::helpers::until_eof, map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    pub String,
);

impl Model for Biography {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn read_biography() {
        let file = File::open("fixtures/test.bio").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let bio = Biography::new(&buffer);
        assert_eq!(bio.0, String::from_utf8(buffer).unwrap());
    }
}
