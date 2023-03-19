use std::{fmt::Debug, usize};

use crate::common::header::Header;
use crate::common::{fixed_char_array::FixedCharSlice, varriable_char_array::VarriableCharArray};
use crate::resources::utils::{copy_buff_to_struct, copy_transmute_buff};

use super::resources::types::ResourceType;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm
#[derive(Debug)]
pub struct Key {
    pub header: KeyHeader,
    pub bif_entries: Vec<BiffIndex>,
    pub resource_entries: Vec<ResourceIndex>,
}

impl Key {
    pub fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<KeyHeader>(buffer, 0);

        let start = usize::try_from(header.offset_to_bif_entries).unwrap_or(0);
        let count = usize::try_from(header.count_of_bif_entries).unwrap_or(0);
        let bifs = copy_transmute_buff::<BiffIndexHeader>(buffer, start, count);

        let bif_entries: Vec<BiffIndex> = bifs
            .iter()
            .flat_map(|header| BiffIndex::try_from(header, buffer))
            .collect();

        let start = usize::try_from(header.offset_to_resource_entries).unwrap_or(0);
        let count = usize::try_from(header.count_of_resource_entries).unwrap_or(0);
        let raw_resource_entries = copy_transmute_buff::<RawResourceIndex>(buffer, start, count);
        let resource_entries = raw_resource_entries
            .iter()
            .map(ResourceIndex::from)
            .collect();

        Key {
            header,
            bif_entries,
            resource_entries,
        }
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct KeyHeader {
    header: Header<4, 4>,
    count_of_bif_entries: i32,
    count_of_resource_entries: i32,
    offset_to_bif_entries: i32,
    offset_to_resource_entries: i32,
}

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_BifIndices
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BiffIndexHeader {
    file_length: i32,
    offset_to_file_name: i32,
    file_name_length: i16,
    file_location: i16,
}

#[derive(Debug)]
pub struct BiffIndex {
    pub header: BiffIndexHeader,
    pub name: VarriableCharArray,
}

impl BiffIndex {
    fn try_from(header: &BiffIndexHeader, buffer: &[u8]) -> Option<Self> {
        let start = usize::try_from(header.offset_to_file_name).unwrap_or(0);
        let end = start + usize::try_from(header.file_name_length).unwrap_or(0);
        buffer.get(start..end).map(|buff| BiffIndex {
            header: *header,
            name: VarriableCharArray(buff.to_vec()),
        })
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm#keyv1_ResIndices
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct RawResourceIndex {
    name: FixedCharSlice<8>,
    resource_type: ResourceType,
    locator: u32,
}

#[derive(Debug)]
pub struct ResourceIndex {
    pub name: FixedCharSlice<8>,
    pub resource_type: ResourceType,
    pub source_index: u16,
    pub tileset_index: u8,
    pub non_tileset_file_index: u16,
}

impl From<&RawResourceIndex> for ResourceIndex {
    fn from(raw: &RawResourceIndex) -> Self {
        ResourceIndex {
            name: raw.name,
            resource_type: raw.resource_type,
            source_index: ((raw.locator & 0xFFF00000) >> 20) as u16,
            tileset_index: ((raw.locator & 0x000FC000) >> 14) as u8,
            non_tileset_file_index: (raw.locator & 0x00003FFF) as u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_item_file_parsed() {
        let file = File::open("fixtures/chitin.key").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let key = Key::new(&buffer);
        assert_eq!(key.bif_entries.len(), { key.header.count_of_bif_entries }
            as usize);
        assert_eq!(
            key.resource_entries.len(),
            { key.header.count_of_resource_entries } as usize
        );
    }
}
