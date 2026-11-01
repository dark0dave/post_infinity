use core::{slice, str};
use std::error::Error;
use std::{fmt::Debug, path::Path};

use serde::{Deserialize, Serialize};
use zerocopy::FromBytes;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::biff::Biff;
use crate::common::ZeroResref;

use crate::common::{Signature, Version, zero_char_vec::ZeroCharVec};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm
#[derive(Debug, PartialEq, Serialize, IntoBytes)]
#[repr(C, packed)]
pub struct Key<'data> {
    #[serde(flatten, borrow)]
    pub header: &'data KeyHeader,
    #[serde(borrow)]
    pub bif_entries: &'data [BiffEntry],
    #[serde(borrow)]
    pub bif_file_names: &'data [ZeroCharVec<'data>],
    #[serde(borrow)]
    pub resource_entries: &'data [ResourceEntry],
}

impl<'data> Key<'data> {
    pub fn parse(bytes: &'data [u8]) -> Result<Self, Box<dyn Error>> {
        let (header, tail) = <KeyHeader>::ref_from_prefix(bytes).map_err(|err| err.to_string())?;

        let number_of_entries: usize = header.count_of_bif_entries.try_into()?;
        let (bif_entries, tail) =
            <[BiffEntry]>::ref_from_prefix_with_elems(tail, number_of_entries)
                .map_err(|err| err.to_string())?;

        let number_of_bif_file_names = header.offset_to_resource_entries
            - (header.offset_to_bif_entries + 12 * bif_entries.len() as u32);
        let layout = std::alloc::Layout::array::<ZeroCharVec>(number_of_bif_file_names as usize)?;
        let bif_file_names = unsafe {
            let ptr = std::alloc::alloc(layout) as *mut ZeroCharVec;

            let mut start = 0;
            for (i, entry) in bif_entries.iter().enumerate() {
                let end = start + entry.file_name_length as usize;
                let slice: &[u8] = tail
                    .get(start..end)
                    .ok_or("failed to get bytes for string from buffer")?;
                *ptr.add(i) = ZeroCharVec::from(slice);
                start = end;
            }

            slice::from_raw_parts(ptr, number_of_bif_file_names as usize)
        };
        let resources = bytes
            .get((header.offset_to_resource_entries as usize)..)
            .ok_or("Empty buffer")?;
        let resource_entries = <[ResourceEntry]>::ref_from_bytes_with_elems(
            resources,
            header.count_of_resource_entries as usize,
        )
        .map_err(|err| err.to_string())?;

        Ok(Self {
            header,
            bif_entries,
            bif_file_names,
            resource_entries,
        })
    }
    pub fn recurse(&mut self, path: &Path) -> Result<Vec<Biff>, Box<dyn Error>> {
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
        Ok(out)
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_Header
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
#[repr(C, packed)]
pub struct KeyHeader {
    pub signature: Signature,
    pub version: Version,
    pub count_of_bif_entries: u32,
    pub count_of_resource_entries: u32,
    pub offset_to_bif_entries: u32,
    pub offset_to_resource_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_BifIndices
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
#[repr(C, packed)]
pub struct BiffEntry {
    pub file_length: u32,
    pub offset_to_file_name: u32,
    pub file_name_length: u16,
    pub file_location: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_ResIndices
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
#[repr(C, packed)]
pub struct ResourceEntry {
    pub name: ZeroResref,
    pub resource_type: u16,
    pub locator: u32,
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
            let bytes = read_file(file_path)?;
            let key: Key = Key::parse(&bytes)?;
            let result: Value = serde_json::to_value(key)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
