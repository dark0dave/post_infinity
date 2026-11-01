use core::str;
use std::{error::Error, slice};

use serde::{Deserialize, Serialize};
use zerocopy::FromBytes;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::common::{Signature, Strref, Version, zero_char_vec::ZeroCharVec};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm
#[derive(Debug, PartialEq, Serialize, IntoBytes)]
#[repr(C, packed)]
pub struct Biff<'data> {
    #[serde(flatten, borrow)]
    pub header: &'data BiffHeader,
    #[serde(borrow)]
    pub fileset_entries: &'data [FilesetEntry],
    #[serde(borrow)]
    pub tileset_entries: &'data [TilesetEntry],
    #[serde(borrow)]
    pub data: &'data [ZeroCharVec<'data>],
}

impl<'data> Biff<'data> {
    pub fn parse(bytes: &'data [u8]) -> Result<Self, Box<dyn Error>> {
        let header = <BiffHeader>::ref_from_bytes(bytes).map_err(|err| err.to_string())?;

        let offset_to_fileset_entries: usize = header.offset_to_file_entries.try_into()?;
        let number_of_fileset_entries: usize = header.count_of_fileset_entries.try_into()?;
        let (fileset_entries, tail) = <[FilesetEntry]>::ref_from_prefix_with_elems(
            bytes.get(offset_to_fileset_entries..).unwrap_or_default(),
            number_of_fileset_entries,
        )
        .map_err(|err| err.to_string())?;

        let number_of_tileset_entries: usize = header.count_of_tileset_entries.try_into()?;
        let tileset_entries =
            <[TilesetEntry]>::ref_from_bytes_with_elems(tail, number_of_fileset_entries)
                .map_err(|err| err.to_string())?;

        let number_of_contained_files = number_of_fileset_entries + number_of_tileset_entries;
        let layout = std::alloc::Layout::array::<ZeroCharVec>(number_of_contained_files)?;
        let data = unsafe {
            let ptr = std::alloc::alloc(layout) as *mut ZeroCharVec;

            let mut i = 0;
            for entry in fileset_entries.iter() {
                let start: usize = entry.offset.try_into()?;
                let size: usize = entry.size.try_into()?;
                let end: usize = size + start;
                let slice: &[u8] = tail
                    .get(start..end)
                    .ok_or("failed to get bytes for string from buffer")?;
                *ptr.add(i) = ZeroCharVec::from(slice);
                i += 1;
            }

            for entry in tileset_entries.iter() {
                let start: usize = entry.offset.try_into()?;
                let size: usize = (entry.tile_count * entry.tile_size).try_into()?;
                let end: usize = size + start;
                let slice: &[u8] = tail
                    .get(start..end)
                    .ok_or("failed to get bytes for string from buffer")?;
                *ptr.add(i) = ZeroCharVec::from(slice);
                i += 1;
            }

            slice::from_raw_parts(ptr, number_of_contained_files)
        };
        Ok(Self {
            header,
            fileset_entries,
            tileset_entries,
            data,
        })
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_Header
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
#[repr(C, packed)]
pub struct BiffHeader {
    pub signature: Signature,
    pub version: Version,
    pub count_of_fileset_entries: u32,
    pub count_of_tileset_entries: u32,
    pub offset_to_file_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_FileEntry
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
#[repr(C, packed)]
pub struct FilesetEntry {
    pub resource_locator: Strref,
    pub offset: u32,
    pub size: u32,
    pub resource_type: u16,
    pub unknown: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_TilesetEntry
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
#[repr(C, packed)]
pub struct TilesetEntry {
    pub resource_locator: u32,
    pub offset: u32,
    pub tile_count: u32,
    pub tile_size: u32,
    // Type of this resource (always 0x3eb - TIS)
    pub resource_type: u16,
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
            let bytes = read_file(file_path)?;
            let biff: Biff = Biff::parse(&bytes)?;
            let result: Value = serde_json::to_value(biff)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
