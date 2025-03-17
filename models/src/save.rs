use std::path::Path;
use std::rc::Rc;

use binrw::{io::Cursor, io::Read, BinRead, BinReaderExt, BinWrite};
use flate2::bufread::ZlibDecoder;
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        header::Header,
        parsers::{read_string, write_string},
        types::ResourceType,
    },
    from_buffer,
    model::Model,
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sav_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Save {
    #[serde(flatten)]
    pub header: Header,
    #[br(parse_with=binrw::helpers::until_eof)]
    pub files: Vec<SavedFile>,
}

impl Model for Save {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}, dumping buffer: {:?}", err, buffer);
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sav_v1.htm#savv1_File
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SavedFile {
    pub length_of_filename: u32,
    #[bw(write_with = write_string)]
    #[br(parse_with = |reader, _, _:()| read_string(reader, length_of_filename.into()))]
    pub filename: String,
    pub uncompressed_data_length: u32,
    pub compressed_data_length: u32,
    #[br(count=compressed_data_length, restore_position)]
    pub compressed_data: Vec<u8>,
    #[br(count=compressed_data_length, map = |s: Vec<u8>| parse_compressed_data(&s, &filename))]
    #[bw(ignore)]
    #[serde(skip)]
    pub uncompressed_data: Option<Rc<dyn Model>>,
}

fn parse_compressed_data(buff: &[u8], file_name: &String) -> Option<Rc<dyn Model>> {
    let mut d = ZlibDecoder::new(buff);
    let mut buffer = vec![];
    match d.read_to_end(&mut buffer) {
        Ok(_) => {
            let extension = Path::new(&file_name.to_string())
                .extension()
                .unwrap_or_default()
                .to_ascii_lowercase()
                .into_string()
                .unwrap_or_default()
                .replace('\0', "");
            let resource_type = ResourceType::from(extension.as_str());
            from_buffer(&buffer, resource_type)
        }
        Err(err) => {
            log::error!("{}", err);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::io::Read;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::{error::Error, fs::File};

    const FIXTURES: [(&str, &str); 1] = [("fixtures/baldur.sav", "fixtures/baldur.sav.json")];

    fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let save: Save = Save::new(&read_file(file_path)?);
            let result: Value = serde_json::to_value(save)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
