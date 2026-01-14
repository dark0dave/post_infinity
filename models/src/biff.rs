use core::str;
use std::{error::Error, fs::File, path::PathBuf, rc::Rc};

use binrw::{
    BinRead, BinReaderExt, BinResult, BinWrite,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
};
use serde::{Deserialize, Serialize};

use crate::common::{header::Header, strref::Strref};
use crate::tileset::Tileset;
use crate::{common::types::ResourceType, from_buffer, model::Model};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Biff {
    #[serde(flatten)]
    pub header: BiffHeader,
    #[br(seek_before=SeekFrom::Start(header.offset_to_file_entries as u64), count=header.count_of_fileset_entries)]
    pub fileset_entries: Vec<FilesetEntry>,
    #[br(count=header.count_of_tileset_entries)]
    pub tileset_entries: Vec<TilesetEntry>,
    #[serde(skip)]
    #[br(seek_before=SeekFrom::Start(0), parse_with = |reader, _, _: ()| Biff::parse_contained_files(reader, &fileset_entries, &tileset_entries))]
    #[bw(map = |x| x.iter().flat_map(|x| x.to_bytes()).collect::<Vec<u8>>())]
    pub contained_files: Vec<Rc<dyn Model>>,
}

impl Model for Biff {
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

impl TryFrom<&PathBuf> for Biff {
    type Error = Box<dyn Error>;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let file = File::open(value)?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;
        Ok(Biff::new(&buffer))
    }
}

impl Biff {
    fn parse_contained_files<R: Read + Seek>(
        reader: &mut R,
        fileset_entries: &Vec<FilesetEntry>,
        tileset_entries: &Vec<TilesetEntry>,
    ) -> BinResult<Vec<Rc<dyn Model>>> {
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;

        let mut out: Vec<Rc<dyn Model>> =
            Vec::with_capacity(fileset_entries.len() + tileset_entries.len());
        for fileset_entry in fileset_entries {
            let start: usize = fileset_entry.offset as usize;
            let end: usize = start + fileset_entry.size as usize;
            let buff = buffer.get(start..end).unwrap_or_default();
            match from_buffer(buff, fileset_entry.resource_type) {
                Ok(data) => {
                    out.push(data);
                }
                Err(err) => {
                    log::error!(
                        "Failed to parse resource: {:#?}, with error: {:#?}",
                        fileset_entry.resource_type,
                        err
                    );
                    log::debug!("Dumping buffer: {buff:#?}");
                }
            }
        }
        for tileset_entry in tileset_entries {
            let start: usize = tileset_entry.offset as usize;
            let end: usize = start + (tileset_entry.tile_count * tileset_entry.tile_size) as usize;
            let buff = buffer.get(start..end).unwrap_or_default();
            out.push(Rc::new(Tileset {
                data: buff.to_vec(),
            }));
        }
        Ok(out)
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_Header
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BiffHeader {
    #[serde(flatten)]
    pub header: Header,
    pub count_of_fileset_entries: u32,
    pub count_of_tileset_entries: u32,
    pub offset_to_file_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_FileEntry
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct FilesetEntry {
    pub resource_locator: Strref,
    pub offset: u32,
    pub size: u32,
    pub resource_type: ResourceType,
    pub unknown: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_TilesetEntry
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]

pub struct TilesetEntry {
    pub resource_locator: u32,
    pub offset: u32,
    pub tile_count: u32,
    pub tile_size: u32,
    // Type of this resource (always 0x3eb - TIS)
    pub resource_type: ResourceType,
    pub unknown: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::io::Read;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::fs::File;

    const FIXTURES: [(&str, &str); 1] = [("fixtures/effects.bif", "fixtures/effects.bif.json")];

    fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let biff: Biff = Biff::new(&read_file(file_path)?);
            let result: Value = serde_json::to_value(biff)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
