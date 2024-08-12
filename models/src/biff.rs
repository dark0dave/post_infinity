use core::str;
use std::io::{Read, Seek};
use std::{fmt::Debug, rc::Rc};

use binrw::{io::Cursor, io::SeekFrom, BinRead, BinReaderExt, BinResult, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::strref::Strref;
use crate::tileset::Tileset;
use crate::{common::types::ResourceType, from_buffer, model::Model};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize)]
pub struct Biff {
    #[serde(flatten)]
    pub header: BiffHeader,
    #[br(seek_before=SeekFrom::Start(header.offset_to_file_entries as u64), count=header.count_of_fileset_entries)]
    pub fileset_entries: Vec<FilesetEntry>,
    #[br(count=header.count_of_tileset_entries)]
    pub tileset_entries: Vec<TilesetEntry>,
    #[serde(skip)]
    #[br(seek_before=SeekFrom::Start(0), parse_with = |reader, _endian, _args: Vec<u8>| parse_contained_files(reader, &fileset_entries, &tileset_entries))]
    #[bw(map = |x| x.iter().flat_map(|x| x.to_bytes()).collect::<Vec<u8>>())]
    pub contained_files: Vec<Rc<dyn Model>>,
}

fn parse_contained_files<R: Read + Seek>(
    reader: &mut R,
    fileset_entries: &Vec<FilesetEntry>,
    tileset_entries: &Vec<TilesetEntry>,
) -> BinResult<Vec<Rc<dyn Model>>> {
    let mut buffer = vec![];
    reader.read_to_end(&mut buffer).unwrap();

    let mut out: Vec<Rc<dyn Model>> = Vec::with_capacity(fileset_entries.len());
    for fileset_entry in fileset_entries {
        let start: usize = fileset_entry.offset as usize;
        let end: usize = start + fileset_entry.size as usize;
        let buff = buffer.get(start..end).unwrap_or_default();
        if let Some(data) = from_buffer(buff, fileset_entry.resource_type) {
            out.push(data);
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

impl Biff {
    pub fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}, dumping buffer: {:?}", err, buffer);
            }
        }
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_Header
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BiffHeader {
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub signature: String,
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub version: String,
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
    use pretty_assertions::assert_eq;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_biff_file_parsed() {
        let file = File::open("fixtures/Effects.bif").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let biff = Biff::new(&buffer);
        assert_eq!(
            biff.header,
            BiffHeader {
                signature: "BIFF".to_string(),
                version: "V1  ".to_string(),
                count_of_fileset_entries: 534,
                count_of_tileset_entries: 0,
                offset_to_file_entries: 181288
            }
        );
        assert_eq!(
            *biff.fileset_entries.first().unwrap(),
            FilesetEntry {
                resource_locator: Strref(0),
                offset: 24,
                size: 492,
                resource_type: ResourceType::FileTypeVvc,
                unknown: 0
            }
        );
        assert_eq!(
            *biff.fileset_entries.last().unwrap(),
            FilesetEntry {
                resource_locator: Strref(533),
                offset: 181012,
                size: 272,
                resource_type: ResourceType::FileTypeEff,
                unknown: 0
            }
        )
    }
}
