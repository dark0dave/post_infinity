use core::str;
use std::collections::BTreeMap;
use std::error::Error;

use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};
use zerovec::{ZeroVec, make_ule};

use crate::common::types::ResourceType;
use crate::{IEModels, common::strref::Strref};
use crate::{common::ZeroCharArray, from_buffer, model::Parseable};

const SIZE_OF_FILESET_ENTRY: usize = std::mem::size_of::<FilesetEntry>();
const SIZE_OF_TILESET_ENTRY: usize = std::mem::size_of::<TilesetEntry>();

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm
#[derive(Serialize, Deserialize)]
pub struct Biff<'data> {
    #[serde(flatten)]
    pub header: BiffHeader,
    #[serde(borrow)]
    pub fileset_entries: ZeroVec<'data, FilesetEntry>,
    #[serde(borrow)]
    pub tileset_entries: ZeroVec<'data, TilesetEntry>,
    pub contained_files: Vec<IEModels<'data>>,
}

impl<'data> Parseable<'data> for Biff<'data> {}

impl<'data> TryFrom<&'data [u8]> for Biff<'data> {
    type Error = Box<dyn Error>;

    fn try_from(value: &'data [u8]) -> Result<Self, Self::Error> {
        let (header, _) = <BiffHeader>::read_from_prefix(value).map_err(|err| err.to_string())?;
        let fileset_entires_start: usize = header.offset_to_file_entries as usize;
        let fileset_entries_end: usize = fileset_entires_start
            + (header.count_of_fileset_entries as usize * SIZE_OF_FILESET_ENTRY);
        let fileset_entries: ZeroVec<'data, FilesetEntry> = ZeroVec::parse_bytes(
            value
                .get(fileset_entires_start..fileset_entries_end)
                .unwrap_or_default(),
        )?;
        let tileset_entries_end: usize =
            fileset_entries_end + header.count_of_tileset_entries as usize * SIZE_OF_TILESET_ENTRY;
        let tileset_entries: ZeroVec<'data, TilesetEntry> = ZeroVec::parse_bytes(
            value
                .get(fileset_entries_end..tileset_entries_end)
                .unwrap_or_default(),
        )?;
        let contained_files = parse_contained_files(value, &fileset_entries, &tileset_entries)?;
        Ok(Self {
            header,
            fileset_entries,
            tileset_entries,
            contained_files,
        })
    }
}

impl<'data> TryInto<Vec<u8>> for Biff<'data> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = vec![];
        self.header
            .write_to(&mut buffer)
            .map_err(|err| err.to_string())?;
        buffer.extend_from_slice(self.fileset_entries.as_bytes());
        buffer.extend_from_slice(self.tileset_entries.as_bytes());
        // Because we read everything in order we can write in order
        for model in self.contained_files {
            buffer.extend(model.to_bytes()?);
        }
        Ok(buffer)
    }
}

fn parse_contained_files<'data>(
    buffer: &'data [u8],
    fileset_entries: &ZeroVec<'data, FilesetEntry>,
    tileset_entries: &ZeroVec<'data, TilesetEntry>,
) -> Result<Vec<IEModels<'data>>, Box<dyn Error>> {
    let mut order = BTreeMap::new();
    let mut out: Vec<IEModels> = Vec::with_capacity(fileset_entries.len() + tileset_entries.len());
    for fileset_entry in fileset_entries.iter() {
        let start: usize = fileset_entry.offset as usize;
        let end: usize = start + fileset_entry.size as usize;
        order.insert(end, fileset_entry.resource_type);
    }
    for tileset_entry in tileset_entries.iter() {
        let start: usize = tileset_entry.offset as usize;
        let end: usize = start + (tileset_entry.tile_count * tileset_entry.tile_size) as usize;
        order.insert(end, ResourceType::FileTypeTis as u16);
    }
    let mut start: usize = 0;
    for (k, v) in order {
        let buff = buffer.get(start..k).unwrap_or_default();
        match from_buffer(buff, v) {
            Ok(data) => {
                out.push(data);
            }
            Err(err) => {
                log::error!("Failed to parse resource: {:#?}, with error: {:#?}", v, err);
                log::debug!("Dumping buffer: {buff:#?}");
            }
        }
        start = k;
    }

    Ok(out)
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_Header
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout, Clone,
)]
#[repr(C, packed)]
pub struct BiffHeader {
    pub signature: ZeroCharArray,
    pub version: ZeroCharArray,
    pub count_of_fileset_entries: u32,
    pub count_of_tileset_entries: u32,
    pub offset_to_file_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_FileEntry
#[make_ule(TLKEntryULE)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct FilesetEntry {
    pub resource_locator: Strref,
    pub offset: u32,
    pub size: u32,
    pub resource_type: u16,
    pub unknown: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_TilesetEntry
#[make_ule(TilesetEntryULE)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct TilesetEntry {
    pub resource_locator: Strref,
    pub offset: u32,
    pub tile_count: u32,
    pub tile_size: u32,
    // Type of this resource (always 0x3eb - TIS)
    pub resource_type: u16,
    pub unknown: u16,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;

    const FIXTURES: [(&str, &str); 1] = [("fixtures/effects.bif", "fixtures/effects.bif.json")];

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let buffer = fs::read(file_path)?;
            let biff: Biff = Biff::try_from(buffer.as_slice())?;
            let result: Value = serde_json::to_value(biff)?;
            fs::write("./effects.bif.json", serde_json::to_string(&result)?)?;
            let json_buffer = fs::read(json_file_path)?;
            let expected: Value = serde_json::from_slice(json_buffer.as_slice())?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
