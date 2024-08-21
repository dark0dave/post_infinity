use std::fmt::Debug;

use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::common::{char_array::CharArray, resref::Resref};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Key {
    #[serde(flatten)]
    pub header: KeyHeader,
    #[br(count=header.count_of_bif_entries)]
    pub bif_entries: Vec<BiffEntry>,
    #[br(count=header.offset_to_resource_entries - (header.offset_to_bif_entries + 12 * bif_entries.len() as u32), map = |s: Vec<u8>| read_key_strings(&s, &bif_entries))]
    #[bw(map = |x : &Vec<String>| x.iter().flat_map(|x: &String| x.as_bytes().to_vec()).collect::<Vec<u8>>())]
    pub bif_file_names: Vec<String>,
    #[br(count=header.count_of_resource_entries)]
    pub resource_entries: Vec<ResourceEntry>,
}

fn read_key_strings(s: &[u8], entries: &Vec<BiffEntry>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(entries.len());
    let mut start = 0;
    for entry in entries {
        let end = entry.file_name_length as usize + start;
        out.push(String::from_utf8_lossy(s.get(start..end).unwrap_or_default()).to_string());
        start = end;
    }
    out
}

impl Key {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Self {
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}", err);
            }
        }
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct KeyHeader {
    #[br(count = 4)]
    signature: CharArray,
    #[br(count = 4)]
    version: CharArray,
    count_of_bif_entries: u32,
    count_of_resource_entries: u32,
    offset_to_bif_entries: u32,
    offset_to_resource_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_BifIndices
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BiffEntry {
    file_length: u32,
    offset_to_file_name: u32,
    file_name_length: u16,
    file_location: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_ResIndices
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct ResourceEntry {
    name: Resref,
    resource_type: u16,
    locator: u32,
}

#[cfg(test)]
mod tests {

    use std::fs::File;

    use super::*;
    use binrw::io::BufReader;
    use pretty_assertions::assert_eq;

    #[test]
    fn valid_key_file_parsed() {
        let file = File::open("fixtures/chitin.key").unwrap();
        let mut reader = BufReader::new(file);
        let key = Key::new(&mut reader);
        assert_eq!(
            key.bif_entries.len(),
            key.header.count_of_bif_entries as usize
        );
        assert_eq!(
            key.bif_file_names.first(),
            Some(&"data/Default.bif\0".to_string())
        );
        assert_eq!(
            key.bif_file_names.last(),
            Some(&"data/BDTP_DLC.BIF\0".to_string())
        );
        assert_eq!(
            key.resource_entries.len(),
            key.header.count_of_resource_entries as usize
        );
    }
}
