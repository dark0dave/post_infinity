use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::common::header::Header;
use crate::resources::utils::{
    copy_buff_to_struct, copy_transmute_buff, to_u8_slice, vec_to_u8_slice,
};
use crate::tlk::Lookup;
use crate::{common::fixed_char_array::FixedCharSlice, model::Model};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldMap {
    #[serde(flatten)]
    pub header: WorldMapHeader,
    #[serde(flatten)]
    pub world_map_entries: Vec<WorldMapEntry>,
    pub area_entries: Vec<AreaEntry>,
    pub area_link_entries: Vec<AreaLink>,
}

impl Model for WorldMap {
    fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<WorldMapHeader>(buffer, 0);

        let count = usize::try_from(header.count_of_worldmap_entries).unwrap_or(0);
        let start = usize::try_from(header.offset_to_worldmap_entries).unwrap_or(0);
        let world_map_entries: Vec<WorldMapEntry> =
            copy_transmute_buff::<WorldMapEntry>(buffer, start, count);

        let mut area_entries = vec![];
        let mut area_link_entries = vec![];
        world_map_entries.iter().for_each(|world_map_entry| {
            let start = usize::try_from(world_map_entry.offset_to_area_entries).unwrap_or(0);
            let count = usize::try_from(world_map_entry.count_of_area_entries).unwrap_or(0);
            area_entries.extend(copy_transmute_buff::<AreaEntry>(buffer, start, count));

            let start = usize::try_from(world_map_entry.offset_to_area_link_entries).unwrap_or(0);
            let count = usize::try_from(world_map_entry.count_of_area_link_entries).unwrap_or(0);
            area_link_entries.extend(copy_transmute_buff::<AreaLink>(buffer, start, count));
        });

        Self {
            header,
            world_map_entries,
            area_entries,
            area_link_entries,
        }
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut out = to_u8_slice(&self.header).to_vec();
        out.extend(vec_to_u8_slice(&self.world_map_entries));
        out.extend(vec_to_u8_slice(&self.area_entries));
        out.extend(vec_to_u8_slice(&self.area_link_entries));
        out
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Header
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct WorldMapHeader {
    pub header: Header<4, 4>,
    pub count_of_worldmap_entries: i32,
    pub offset_to_worldmap_entries: i32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Entry
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct WorldMapEntry {
    pub background_image_mos_file: FixedCharSlice<8>,
    pub width: u32,
    pub height: u32,
    pub map_number: u32,
    pub area_name: FixedCharSlice<4>,
    pub start_centered_on_x: u32,
    pub start_centered_on_y: u32,
    pub count_of_area_entries: i32,
    pub offset_to_area_entries: i32,
    pub offset_to_area_link_entries: i32,
    pub count_of_area_link_entries: i32,
    pub map_icons_bam_file: FixedCharSlice<8>,
    // BGEE feild only
    pub flags: u32,
    #[serde(skip)]
    _unused: FixedCharSlice<128>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Area
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct AreaEntry {
    pub area_resref: FixedCharSlice<8>,
    pub area_name_short: FixedCharSlice<8>,
    pub area_name_long: FixedCharSlice<8>,
    pub bitmask_indicating_status_of_area: FixedCharSlice<4>,
    pub bam_file_sequence_icons: u32,
    pub x_coordinate: u32,
    pub y_coordinate: u32,
    pub name_caption: FixedCharSlice<4>,
    pub name_tooltips: FixedCharSlice<4>,
    pub loading_screen_mos_file: FixedCharSlice<8>,
    pub link_index_north: u32,
    pub link_count_north: u32,
    pub link_index_west: u32,
    pub link_count_west: u32,
    pub link_index_south: u32,
    pub link_count_south: u32,
    pub link_index_east: u32,
    pub link_count_east: u32,
    #[serde(skip)]
    _unused: FixedCharSlice<128>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_AreaLink
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct AreaLink {
    pub index_of_destination_area: u32,
    pub entry_point: FixedCharSlice<32>,
    pub travel_time: u32,
    pub default_entry_location: u32,
    pub random_encounter_area_1: FixedCharSlice<8>,
    pub random_encounter_area_2: FixedCharSlice<8>,
    pub random_encounter_area_3: FixedCharSlice<8>,
    pub random_encounter_area_4: FixedCharSlice<8>,
    pub random_encounter_area_5: FixedCharSlice<8>,
    pub random_encounter_probability: u32,
    #[serde(skip)]
    _unused: FixedCharSlice<128>,
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
    fn world_test() {
        let file = File::open("fixtures/WORLDMAP.WMP").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let world = WorldMap::new(&buffer);
        assert_eq!(world.area_entries.len(), 58);
        assert_eq!(world.area_link_entries.len(), 208)
    }
}
