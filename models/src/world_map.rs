use binrw::{io::Cursor, io::SeekFrom, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::char_array::CharArray;
use crate::common::resref::Resref;
use crate::common::strref::Strref;
use crate::model::Model;

#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct WorldMap {
    #[serde(flatten)]
    pub header: WorldMapHeader,
    #[br(count=header.count_of_worldmap_entries, seek_before=SeekFrom::Start(header.offset_to_worldmap_entries as u64))]
    pub world_map_entries: Vec<WorldMapEntry>,
    #[br(count=world_map_entries.iter().map(|x| x.count_of_area_entries).reduce(|a,b| a+b).unwrap_or_default())]
    pub area_entries: Vec<AreaEntry>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub area_link_entries: Vec<AreaLink>,
}

impl Model for WorldMap {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}, dumping buffer: {:?}", err, buffer);
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct WorldMapHeader {
    #[br(count = 4)]
    pub signature: CharArray,
    #[br(count = 4)]
    pub version: CharArray,
    pub count_of_worldmap_entries: u32,
    pub offset_to_worldmap_entries: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Entry
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct WorldMapEntry {
    pub background_image_mos_file: Resref,
    pub width: u32,
    pub height: u32,
    pub map_number: u32,
    pub area_name: Strref,
    pub start_centered_on_x: u32,
    pub start_centered_on_y: u32,
    pub count_of_area_entries: u32,
    pub offset_to_area_entries: u32,
    pub offset_to_area_link_entries: u32,
    pub count_of_area_link_entries: u32,
    pub map_icons_bam_file: Resref,
    // BGEE field only
    pub flags: u32,
    #[serde(skip)]
    #[br(count = 124)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Area
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct AreaEntry {
    pub area_resref: Resref,
    pub area_name_short: Resref,
    #[br(count = 32)]
    pub area_name_long: CharArray,
    #[br(count = 4)]
    pub bitmask_indicating_status_of_area: Vec<u8>,
    pub bam_file_sequence_icons: u32,
    pub x_coordinate: u32,
    pub y_coordinate: u32,
    pub name_caption: Strref,
    pub name_tooltips: Strref,
    pub loading_screen_mos_file: Resref,
    pub link_index_north: u32,
    pub link_count_north: u32,
    pub link_index_west: u32,
    pub link_count_west: u32,
    pub link_index_south: u32,
    pub link_count_south: u32,
    pub link_index_east: u32,
    pub link_count_east: u32,
    #[serde(skip)]
    #[br(count = 128)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_AreaLink
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct AreaLink {
    pub index_of_destination_area: u32,
    #[br(count = 32)]
    pub entry_point: CharArray,
    pub travel_time: u32,
    pub default_entry_location: u32,
    pub random_encounter_area_1: Resref,
    pub random_encounter_area_2: Resref,
    pub random_encounter_area_3: Resref,
    pub random_encounter_area_4: Resref,
    pub random_encounter_area_5: Resref,
    pub random_encounter_probability: u32,
    #[serde(skip)]
    #[br(count = 128)]
    _unused: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    use binrw::io::{BufReader, Read};
    use std::fs::File;

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
