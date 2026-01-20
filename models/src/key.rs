use std::fmt::Debug;

use std::error::Error;

use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};
use zerovec::maps::MutableZeroVecLike;
use zerovec::vecs::Index32;
use zerovec::{VarZeroVec, ZeroVec, make_ule};

use crate::common::{ZeroCharArray, ZeroResref};
use crate::model::Parseable;

const SIZE_OF_KEY_HEADER: usize = std::mem::size_of::<KeyHeader>();
const SIZE_OF_BIF_ENTRY: usize = std::mem::size_of::<BiffEntry>();

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm
#[derive(Debug, Serialize, Deserialize)]
pub struct Key<'data> {
    #[serde(flatten)]
    pub header: KeyHeader,
    #[serde(borrow)]
    pub bif_entries: ZeroVec<'data, BiffEntry>,
    #[serde(borrow)]
    pub bif_file_names: VarZeroVec<'data, str, Index32>,
    #[serde(borrow)]
    pub resource_entries: ZeroVec<'data, ResourceEntry>,
}

impl<'data> Parseable<'data> for Key<'data> {}

impl<'data> TryFrom<&'data [u8]> for Key<'data> {
    type Error = Box<dyn Error>;

    fn try_from(value: &'data [u8]) -> Result<Self, Self::Error> {
        let (header, buff) = <KeyHeader>::read_from_prefix(value).map_err(|err| err.to_string())?;
        let biff_entries_end: usize =
            (header.count_of_bif_entries as usize * SIZE_OF_BIF_ENTRY) - SIZE_OF_KEY_HEADER;
        let bif_entries: ZeroVec<'data, BiffEntry> =
            ZeroVec::parse_bytes(buff.get(0..biff_entries_end).unwrap_or_default())?;
        let bif_file_names = read_key_strings(buff, &bif_entries);
        let resource_entires_start: usize = header.offset_to_resource_entries as usize;
        let resource_entires_end: usize = resource_entires_start
            + (header.count_of_resource_entries * header.count_of_resource_entries) as usize;
        let resource_entries: ZeroVec<'data, ResourceEntry> = ZeroVec::parse_bytes(
            buff.get(resource_entires_start..resource_entires_end)
                .unwrap_or_default(),
        )?;
        Ok(Self {
            header,
            bif_entries,
            bif_file_names,
            resource_entries,
        })
    }
}

impl<'data> TryInto<Vec<u8>> for Key<'data> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = vec![];
        self.header
            .write_to(&mut buffer)
            .map_err(|err| err.to_string())?;
        buffer.extend_from_slice(self.bif_entries.as_bytes());
        buffer.extend_from_slice(self.bif_file_names.as_bytes());
        buffer.extend_from_slice(self.resource_entries.as_bytes());
        Ok(buffer)
    }
}

fn read_key_strings<'data>(
    buffer: &'data [u8],
    entries: &ZeroVec<'data, BiffEntry>,
) -> VarZeroVec<'data, str, Index32> {
    let mut out = VarZeroVec::zvl_with_capacity(entries.len());
    for entry in entries.iter() {
        let start = entry.offset_to_file_name as usize - SIZE_OF_KEY_HEADER;
        let end = entry.file_name_length as usize + start;
        let slice = buffer.get(start..end).unwrap_or_default();
        out.zvl_push(std::str::from_utf8(slice).unwrap_or_default());
    }
    out
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_Header
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout, Clone,
)]
#[repr(C, packed)]
pub struct KeyHeader {
    pub signature: ZeroCharArray,
    pub version: ZeroCharArray,
    pub count_of_bif_entries: u32,
    pub count_of_resource_entries: u32,
    pub offset_to_bif_entries: u32,
    pub offset_to_resource_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_BifIndices
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[make_ule(BiffEntryULE)]
pub struct BiffEntry {
    pub file_length: u32,
    pub offset_to_file_name: u32,
    pub file_name_length: u16,
    pub file_location: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_ResIndices
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[make_ule(ResourceEntryULE)]
pub struct ResourceEntry {
    pub name: ZeroResref,
    pub resource_type: u16,
    pub locator: u32,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;

    const FIXTURES: [(&str, &str); 1] = [("fixtures/chitin.key", "fixtures/chitin.key.json")];

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let buffer = fs::read(file_path)?;
            let key: Key = Key::try_from(buffer.as_slice())?;
            let result: Value = serde_json::to_value(key)?;
            let json_buffer = fs::read(json_file_path)?;
            let expected: Value = serde_json::from_slice(json_buffer.as_slice())?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
