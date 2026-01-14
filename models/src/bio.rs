use binrw::{BinRead, BinReaderExt, BinWrite, io::Cursor};
use serde::{Deserialize, Serialize};

use crate::{
    common::parsers::{read_to_end, write_string},
    model::Model,
};

#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Biography {
    #[bw(write_with = write_string)]
    #[br(parse_with = read_to_end)]
    pub contents: String,
}

impl Model for Biography {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.contents.clone().into_bytes()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use binrw::io::{BufReader, Read};
    use std::fs::File;

    #[test]
    fn read_biography() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("fixtures/test.bio")?;
        let mut buffer = vec![];
        BufReader::new(file).read_to_end(&mut buffer)?;
        let bio = Biography::new(&buffer);
        let expected = String::from_utf8(buffer.to_vec())?;
        assert_eq!(bio.contents, expected);
        Ok(())
    }
}
