use core::mem::size_of;
use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::common::header::Header;
use crate::common::signed_fixed_char_array::SignedFixedCharSlice;
use crate::resources::utils::{copy_buff_to_struct, copy_transmute_buff};
use crate::{from_buffer, model::Model, resources::types::ResourceType};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm
#[derive(Debug)]
pub struct Biff {
    pub header: BiffHeader,
    pub fileset_entries: HashMap<ResourceType, Vec<FilesetEntry>>,
    pub tileset_entries: Vec<TilesetEntry>,
}

impl Biff {
    // TODO: Pass option to process only resources needed, make this part of init and make new from Resource
    pub fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<BiffHeader>(buffer, 0);

        let start = usize::try_from(header.offset_to_file_entries).unwrap_or(0);
        let count = usize::try_from(header.count_of_fileset_entries).unwrap_or(0);
        let file_set = copy_transmute_buff::<FilesetEntryHeader>(buffer, start, count);

        let mut fileset_entries: HashMap<ResourceType, Vec<FilesetEntry>> = HashMap::new();
        for header in file_set {
            let start = usize::try_from(header.offset).unwrap_or(0);
            let end = start + usize::try_from(header.size).unwrap_or(0);
            let buffer = buffer.get(start..end).unwrap();
            if let Some(data) = from_buffer(buffer, header.resource_type) {
                fileset_entries
                    .entry(header.resource_type)
                    .or_default()
                    .push(FilesetEntry { header, data });
            }
        }

        Biff {
            header,
            fileset_entries,
            tileset_entries: vec![],
        }
    }

    pub fn populate_tiles(&mut self, buffer: &[u8]) {
        let start_of_file_entries =
            usize::try_from(self.header.offset_to_file_entries).unwrap_or(0);
        let count_of_file_entries =
            usize::try_from(self.header.count_of_fileset_entries).unwrap_or(0);

        let start = start_of_file_entries + count_of_file_entries * size_of::<FilesetEntryHeader>();
        let count = usize::try_from(self.header.count_of_tileset_entries).unwrap_or(0);
        self.tileset_entries = copy_transmute_buff::<TilesetEntry>(buffer, start, count);
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BiffHeader {
    pub header: Header<4, 4>,
    pub count_of_fileset_entries: u32,
    pub count_of_tileset_entries: u32,
    pub offset_to_file_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_FileEntry
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FilesetEntryHeader {
    pub resource_locator: SignedFixedCharSlice<4>,
    pub offset: u32,
    pub size: u32,
    pub resource_type: ResourceType,
    pub unknown: u16,
}

#[derive(Debug)]
pub struct FilesetEntry {
    pub header: FilesetEntryHeader,
    pub data: Rc<dyn Model>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_TilesetEntry
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TilesetEntry {
    pub resource_locator: u32,
    pub offset: u32,
    pub tile_count: u32,
    pub tile_size: u32,
    pub resource_type: ResourceType,
    pub unknown: u16,
}
