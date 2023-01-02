use std::{mem::size_of, rc::Rc};

use crate::{
    common::fixed_char_array::FixedCharSlice,
    model::Model,
    utils::{copy_buff_to_struct, copy_transmute_buff},
};

#[derive(Debug)]
pub struct WorldMap {
    pub world_map_header: WorldMapHeader,
    pub world_map_entries: Vec<WorldMapEntry>,
}

impl Model for WorldMap {
    fn new(buffer: &[u8]) -> Self {
        let world_map_header = copy_buff_to_struct::<WorldMapHeader>(buffer, 0);

        let count = usize::try_from(world_map_header.count_of_worldmap_entries).unwrap_or(0);
        let world_map_entries: Vec<WorldMapEntry> = (0..count)
            .into_iter()
            .map(|counter| {
                let start = size_of::<WorldMapEntryUnlinked>() * counter
                    + usize::try_from(world_map_header.offset_to_worldmap_entries).unwrap_or(0);
                let world_map_entry = copy_buff_to_struct::<WorldMapEntryUnlinked>(buffer, start);

                let start = usize::try_from(world_map_entry.offset_to_area_entries).unwrap_or(0);
                let count = usize::try_from(world_map_entry.count_of_area_entries).unwrap_or(0);
                let area_entries = copy_transmute_buff::<AreaEntry>(buffer, start, count);

                let start =
                    usize::try_from(world_map_entry.offset_to_area_link_entries).unwrap_or(0);
                let count =
                    usize::try_from(world_map_entry.count_of_area_link_entries).unwrap_or(0);
                let area_link_entries = copy_transmute_buff::<AreaLink>(buffer, start, count);

                WorldMapEntry {
                    world_map_entry,
                    area_entries,
                    area_link_entries,
                }
            })
            .collect();

        Self {
            world_map_header,
            world_map_entries,
        }
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Header
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WorldMapHeader {
    pub signature: FixedCharSlice<4>,
    pub version: FixedCharSlice<4>,
    pub count_of_worldmap_entries: i32,
    pub offset_to_worldmap_entries: i32,
}

// One to Many
// One WorldMapEntry -> Many Area Entries
//                   -> Many AreaEntries
#[derive(Debug)]
pub struct WorldMapEntry {
    pub world_map_entry: WorldMapEntryUnlinked,
    pub area_entries: Vec<AreaEntry>,
    pub area_link_entries: Vec<AreaLink>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Entry
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WorldMapEntryUnlinked {
    pub background_image_mos_file: FixedCharSlice<8>,
    pub width: u32,
    pub height: u32,
    pub map_number: u32,
    pub area_name: FixedCharSlice<8>,
    pub start_centered_on_x: u32,
    pub start_centered_on_y: u32,
    pub count_of_area_entries: i32,
    pub offset_to_area_entries: i32,
    pub offset_to_area_link_entries: i32,
    pub count_of_area_link_entries: i32,
    pub map_icons_bam_file: [u8; 8],
    // BGEE feild only
    pub flags: u32,
    _unused: [u8; 128],
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Area
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct AreaEntry {
    pub area_resref: [u8; 8],
    pub area_name_short: [u8; 8],
    pub area_name_long: FixedCharSlice<8>,
    pub bitmask_indicating_status_of_area: [u8; 4],
    pub bam_file_sequence_icons: u32,
    pub x_coordinate: u32,
    pub y_coordinate: u32,
    pub name_caption: FixedCharSlice<4>,
    pub name_tooltips: FixedCharSlice<4>,
    pub loading_screen_mos_file: [u8; 8],
    pub link_index_north: u32,
    pub link_count_north: u32,
    pub link_index_west: u32,
    pub link_count_west: u32,
    pub link_index_south: u32,
    pub link_count_south: u32,
    pub link_index_east: u32,
    pub link_count_east: u32,
    _unused: [u8; 128],
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_AreaLink
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct AreaLink {
    pub index_of_destination_area: u32,
    pub entry_point: FixedCharSlice<32>,
    pub travel_time: u32,
    pub default_entry_location: u32,
    pub random_encounter_area_1: [u8; 8],
    pub random_encounter_area_2: [u8; 8],
    pub random_encounter_area_3: [u8; 8],
    pub random_encounter_area_4: [u8; 8],
    pub random_encounter_area_5: [u8; 8],
    pub random_encounter_probability: u32,
    _unused: [u8; 128],
}
