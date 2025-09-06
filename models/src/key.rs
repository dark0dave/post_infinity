use std::{fmt::Debug, path::Path};

use std::error::Error;

use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::{
    biff::Biff,
    common::{header::Header, resref::Resref},
    model::Model,
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Key {
    #[serde(flatten)]
    pub header: KeyHeader,
    #[br(count=header.count_of_bif_entries)]
    pub bif_entries: Vec<BiffEntry>,
    #[br(count=header.offset_to_resource_entries - (header.offset_to_bif_entries + 12 * bif_entries.len() as u32), map = |s: Vec<u8>| read_key_strings(&s, &bif_entries))]
    #[bw(map = |x : &Vec<String>| x.iter().flat_map(|x: &String| x.clone().into_bytes()).collect::<Vec<u8>>())]
    pub bif_file_names: Vec<String>,
    #[br(count=header.count_of_resource_entries)]
    pub resource_entries: Vec<ResourceEntry>,
    #[bw(ignore)]
    #[br(ignore)]
    pub biffs: Vec<Biff>,
}

fn read_key_strings(s: &[u8], entries: &Vec<BiffEntry>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(entries.len());
    let mut start = 0;
    for entry in entries {
        let end = entry.file_name_length as usize + start;
        let slice = s.get(start..end).unwrap_or_default();
        out.push(String::from_utf8(slice.to_vec()).unwrap_or_default());
        start = end;
    }
    out
}

impl Model for Key {
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
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

impl Key {
    pub fn recurse(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let parent = path
            .parent()
            .ok_or_else(|| format!("No parent found for {path:?}"))?;
        log::trace!("Parent path is: {parent:?}");
        let mut out = vec![];
        for bif_file_name in self.bif_file_names.iter() {
            let file_path = &parent
                .join(bif_file_name.to_string().replace('\0', ""))
                .canonicalize()?;
            log::debug!("Path to biff: {file_path:?}");
            out.push(Biff::try_from(file_path)?);
        }
        self.biffs = out;
        Ok(())
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct KeyHeader {
    #[serde(flatten)]
    pub header: Header,
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
    use super::*;
    use binrw::io::Read;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::fs::File;

    const FIXTURES: [(&str, &str); 1] = [("fixtures/chitin.key", "fixtures/chitin.key.json")];

    fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let key: Key = Key::new(&read_file(file_path)?);
            let result: Value = serde_json::to_value(key)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
