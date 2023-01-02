use core::mem::size_of;
use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    common::fixed_char_array::FixedCharSlice,
    model::Model,
    resources::types::ResourceType,
    utils::{copy_buff_to_struct, copy_transmute_buff, from_buffer},
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm
#[derive(Debug)]
pub struct Biff {
    pub header: Header,
    pub fileset_entries: HashMap<ResourceType, Vec<FilesetEntry>>,
    pub tileset_entries: Vec<TilesetEntry>,
}

impl Biff {
    // TODO: Pass option to process only resources needed, make this part of init and make new from Resource
    pub fn new(buffer: &[u8], process_tiles: bool) -> Self {
        let header = copy_buff_to_struct::<Header>(buffer, 0);

        let start = header.offset_to_file_entries as usize;
        let count = header.count_of_fileset_entries as usize;
        let file_set = copy_transmute_buff::<FilesetEntryHeader>(buffer, start, count);

        let mut fileset_entries: HashMap<ResourceType, Vec<FilesetEntry>> = HashMap::new();
        for header in file_set {
            let start = header.offset as usize;
            let end = start + header.size as usize;
            let buffer = buffer.get(start..end).unwrap();
            if let Some(data) = from_buffer(buffer, header.resource_type) {
                fileset_entries
                    .entry(header.resource_type)
                    .or_default()
                    .push(FilesetEntry { header, data });
            }
        }

        let tileset_entries = if process_tiles {
            let start = start + count * size_of::<FilesetEntryHeader>();
            let count = header.count_of_tileset_entries as usize;
            copy_transmute_buff::<TilesetEntry>(buffer, start, count)
        } else {
            vec![]
        };

        Biff {
            header,
            fileset_entries,
            tileset_entries,
        }
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub signature: FixedCharSlice<4>,
    pub version: FixedCharSlice<4>,
    pub count_of_fileset_entries: u32,
    pub count_of_tileset_entries: u32,
    pub offset_to_file_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm#bif_v1_FileEntry
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FilesetEntryHeader {
    pub resource_locator: u32,
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
