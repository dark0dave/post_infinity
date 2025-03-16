use binrw::{helpers::until_eof, io::Cursor, io::SeekFrom, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::char_array::CharArray;
use crate::common::header::Header;
use crate::common::strref::Strref;
use crate::common::Resref;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Area {
    #[serde(skip)]
    #[br(parse_with = until_eof, restore_position)]
    pub original_bytes: Vec<u8>,
    #[bw(ignore)]
    #[serde(flatten)]
    pub header: FileHeader,
    #[bw(ignore)]
    #[br(count=header.count_of_actors, seek_before=SeekFrom::Start(header.offset_to_actors as u64))]
    pub actors: Vec<Actor>,
    #[bw(ignore)]
    #[br(count=header.count_of_regions, seek_before=SeekFrom::Start(header.offset_to_regions as u64))]
    pub regions: Vec<Region>,
    #[bw(ignore)]
    #[br(count=header.count_of_spawn_points, seek_before=SeekFrom::Start(header.offset_to_spawn_points as u64))]
    pub spawn_points: Vec<SpawnPoint>,
    #[bw(ignore)]
    #[br(count=header.count_of_entrances, seek_before=SeekFrom::Start(header.offset_to_entrances as u64))]
    pub entrances: Vec<Entrance>,
    #[bw(ignore)]
    #[br(count=header.count_of_containers, seek_before=SeekFrom::Start(header.offset_to_containers as u64))]
    pub containers: Vec<Container>,
    #[bw(ignore)]
    #[br(count=header.count_of_items, seek_before=SeekFrom::Start(header.offset_to_items as u64))]
    pub items: Vec<Item>,
    #[bw(ignore)]
    #[br(count=header.count_of_vertices, seek_before=SeekFrom::Start(header.offset_to_vertices as u64))]
    pub vertices: Vec<Vertice>,
    #[bw(ignore)]
    #[br(count=header.count_of_ambients, seek_before=SeekFrom::Start(header.offset_to_ambients as u64))]
    pub ambients: Vec<Ambient>,
    #[bw(ignore)]
    #[br(count=header.count_of_variables, seek_before=SeekFrom::Start(header.offset_to_variables as u64))]
    pub variables: Vec<Variable>,
    #[bw(ignore)]
    #[br(count=header.size_of_explored_bitmask, seek_before=SeekFrom::Start(header.offset_to_explored_bitmask as u64))]
    pub explored_bitmasks: Vec<ExploredBitmask>,
    #[bw(ignore)]
    #[br(count=header.count_of_doors, seek_before=SeekFrom::Start(header.offset_to_doors as u64))]
    pub doors: Vec<Door>,
    #[bw(ignore)]
    #[br(count=header.count_of_animations, seek_before=SeekFrom::Start(header.offset_to_animations as u64))]
    pub animations: Vec<Animation>,
    #[bw(ignore)]
    #[br(count=header.count_of_automap_notes, seek_before=SeekFrom::Start(header.offset_to_automap_notes as u64))]
    pub automap_notes: Vec<AutomapNotesBGEE>,
    #[bw(ignore)]
    #[br(count=header.count_of_tiled_objects,seek_before=SeekFrom::Start(header.offset_to_tiled_objects as u64))]
    pub tiled_objects: Vec<TiledObject>,
    #[bw(ignore)]
    #[br(count=header.number_of_entries_in_the_projectile_traps, seek_before=SeekFrom::Start(header.offset_to_projectile_traps as u64))]
    pub projectile_traps: Vec<ProjectileTrap>,
    #[bw(ignore)]
    #[serde(flatten)]
    #[br(seek_before=SeekFrom::Start(header.offset_to_song_entries as u64))]
    pub songs: SongEntry,
    #[bw(ignore)]
    #[serde(flatten)]
    #[br(seek_before=SeekFrom::Start(header.offset_to_rest_interruptions as u64))]
    pub rest_interruptions: RestInterruption,
}

impl Model for Area {
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

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct FileHeader {
    #[serde(flatten)]
    pub header: Header,
    pub area_wed: Resref,
    pub last_saved: u32,
    pub area_flags: u32,
    pub resref_of_the_area_to_the_north_of_this_area: Resref,
    pub north_area_flags: u32,
    pub resref_of_the_area_to_the_east_of_this_area: Resref,
    pub east_area_flags: u32,
    pub resref_of_the_area_to_the_south_of_this_area: Resref,
    pub south_area_flags: u32,
    pub resref_of_the_area_to_the_west_of_this_area: Resref,
    pub west_area_flags: u32,
    pub area_type_flags: u16,
    pub rain_probability: u16,
    pub snow_probability: u16,
    // bgee only
    pub fog_probability: u16,
    pub lightning_probability: u16,
    pub wind_speed: u16,
    pub offset_to_actors: u32,
    pub count_of_actors: u16,
    pub count_of_regions: u16,
    pub offset_to_regions: u32,
    pub offset_to_spawn_points: u32,
    pub count_of_spawn_points: u32,
    pub offset_to_entrances: u32,
    pub count_of_entrances: u32,
    pub offset_to_containers: u32,
    pub count_of_containers: u16,
    pub count_of_items: u16,
    pub offset_to_items: u32,
    pub offset_to_vertices: u32,
    pub count_of_vertices: u16,
    pub count_of_ambients: u16,
    pub offset_to_ambients: u32,
    pub offset_to_variables: u32,
    pub count_of_variables: u32,
    pub offset_to_tiled_object_flags: u16,
    pub count_of_tiled_object_flags: u16,
    pub area_script: Resref,
    pub size_of_explored_bitmask: u32,
    pub offset_to_explored_bitmask: u32,
    pub count_of_doors: u32,
    pub offset_to_doors: u32,
    pub count_of_animations: u32,
    pub offset_to_animations: u32,
    pub count_of_tiled_objects: u32,
    pub offset_to_tiled_objects: u32,
    pub offset_to_song_entries: u32,
    pub offset_to_rest_interruptions: u32,
    pub offset_to_automap_notes: u32,
    pub count_of_automap_notes: u32,
    pub offset_to_projectile_traps: u32,
    pub number_of_entries_in_the_projectile_traps: u32,
    // bgee and bg2:tob
    pub rest_movie_day: Resref,
    // bgee and bg2:tob
    pub rest_movie_night: Resref,
    #[serde(skip)]
    #[br(count = 56)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Actor
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Actor {
    pub name: CharArray<32>,
    pub current_x_coordinate: u16,
    pub current_y_coordinate: u16,
    pub destination_x_coordinate: u16,
    pub destination_y_coordinate: u16,
    pub flags: u32,
    pub has_been_spawned: u16,
    pub first_letter_of_cre_resref: u8,
    #[serde(skip)]
    _unused_1: u8,
    pub actor_animation: u32,
    pub actor_orientation: u16,
    #[serde(skip)]
    _unused: u16,
    pub actor_removal_timer: u32,
    pub movement_restriction_distance: u16,
    pub movement_restriction_distance_move_to_object: u16,
    pub actor_appearence_schedule: u32,
    pub num_times_talked_to: u32,
    pub dialog: Resref,
    pub script_override: Resref,
    pub script_general: Resref,
    pub script_class: Resref,
    pub script_race: Resref,
    pub script_default: Resref,
    pub script_specific: Resref,
    pub cre_file: Resref,
    // for embedded cre files
    pub offset_to_cre_structure: u32,
    pub size_of_stored_cre_structure: u32,
    #[serde(skip)]
    #[br(count = 128)]
    _unused_2: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Info
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Region {
    pub name: CharArray<32>,
    pub region_type: u16,
    pub minimum_bounding_box_of_this_point: [u16; 4],
    pub count_of_vertices_composing_the_perimeter: u16,
    pub index_to_first_vertex: u32,
    pub trigger_value: u32,
    pub cursor_index: u32,
    // for travel regions
    pub destination_area: Resref,
    // for travel regions
    pub entrance_name_in_destination_area: CharArray<32>,
    pub flags: u32,
    // for info points
    pub information_text: Strref,
    pub trap_detection_difficulty_percent: u16,
    pub trap_removal_difficulty_percent: u16,
    // 0=no, 1=yes
    pub region_is_trapped: u16,
    // 0=no, 1=yes
    pub trap_detected: u16,
    pub trap_launch_location: [u16; 2],
    pub key_item: Resref,
    pub region_script: Resref,
    pub alternative_use_point_x_coordinate: u16,
    pub alternative_use_point_y_coordinate: u16,
    #[serde(skip)]
    _unknown_1: u32,
    #[serde(skip)]
    #[br(count = 32)]
    _unknown_2: Vec<u8>,
    // PST, PSTEE fields
    pub sound: Resref,
    pub talk_location_point_x: u16,
    pub talk_location_point_y: u16,
    pub speaker_name: Strref,
    pub dialog_file: Resref,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Spawn
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SpawnPoint {
    pub name: CharArray<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub resref_of_creature_to_spawn_1st: Resref,
    pub resref_of_creature_to_spawn_2nd: Resref,
    pub resref_of_creature_to_spawn_3rd: Resref,
    pub resref_of_creature_to_spawn_4th: Resref,
    pub resref_of_creature_to_spawn_5th: Resref,
    pub resref_of_creature_to_spawn_6th: Resref,
    pub resref_of_creature_to_spawn_7th: Resref,
    pub resref_of_creature_to_spawn_8th: Resref,
    pub resref_of_creature_to_spawn_9th: Resref,
    pub resref_of_creature_to_spawn_10th: Resref,
    pub count_of_spawn_creatures: u16,
    pub base_creature_number_to_spawn: u16,
    pub frequency: u16,
    pub spawn_method: u16,
    pub actor_removal_timer: u32,
    pub movement_restriction_distance: u16,
    pub movement_restriction_distance_move_to_object: u16,
    pub maximum_creatures_to_spawn: u16,
    // 0=Inactive, 1=Active
    pub spawn_point_enabled: u16,
    pub spawn_point_appearence_schedule: u32,
    pub probability_day: u16,
    pub probability_night: u16,
    // BGEE only
    pub spawn_frequency: u32,
    pub countdown: u32,
    // Offset 0x0024
    pub spawn_weight_of_1st_creature_slot: u8,
    // Offset 0x002c
    pub spawn_weight_of_2nd_creature_slot: u8,
    // Offset 0x0034
    pub spawn_weight_of_3rd_creature_slot: u8,
    // Offset 0x003c
    pub spawn_weight_of_4th_creature_slot: u8,
    // Offset 0x0044
    pub spawn_weight_of_5th_creature_slot: u8,
    // Offset 0x004c
    pub spawn_weight_of_6th_creature_slot: u8,
    // Offset 0x0054
    pub spawn_weight_of_7th_creature_slot: u8,
    // Offset 0x005c
    pub spawn_weight_of_8th_creature_slot: u8,
    // Offset 0x0064
    pub spawn_weight_of_9th_creature_slot: u8,
    // Offset 0x006c
    pub spawn_weight_of_10th_creature_slot: u8,
    #[serde(skip)]
    #[br(count = 38)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Entrance
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Entrance {
    pub name: CharArray<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub orientation: u16,
    #[serde(skip)]
    #[br(count = 66)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Container
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Container {
    pub name: CharArray<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub container_type: u16,
    pub lock_difficulty: u16,
    pub flags: u32,
    pub trap_detection_difficulty: u16,
    pub trap_removal_difficulty: u16,
    // 0=no, 1=yes
    pub container_is_trapped: u16,
    // 0=no, 1=yes
    pub trap_detected: u16,
    pub trap_launch_x_coordinate: u16,
    pub trap_launch_y_coordinate: u16,
    pub left_bounding_box_of_container_polygon: u16,
    pub top_bounding_box_of_container_polygon: u16,
    pub right_bounding_box_of_container_polygon: u16,
    pub bottom_bounding_box_of_container_polygon: u16,
    pub index_to_first_item_in_this_container: u32,
    pub count_of_items_in_this_container: u32,
    pub trap_script: Resref,
    pub index_to_first_vertex_of_the_outline: u32,
    pub count_of_vertices_making_up_the_outline: u16,
    pub trigger_range: u16,
    pub owner_script_name: CharArray<32>,
    pub key_item: Resref,
    pub break_difficulty: u32,
    pub lockpick_string: Strref,
    #[serde(skip)]
    #[br(count = 56)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Item
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Item {
    pub item_resref: Resref,
    pub item_expiration_time: u16,
    pub quantity_1: u16,
    pub quantity_2: u16,
    pub quantity_3: u16,
    pub flags: u32,
}

// An array of points used to create the outlines of regions and containers. Elements are 16-bit words stored x0, y0, x1, y1 etc.
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Vertice(pub [u16; 2]);

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Ambient
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Ambient {
    pub name: CharArray<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub radius: u16,
    pub height: u16,
    pub pitch_variance: u32,
    pub volume_variance: u16,
    pub volume_percentage: u16,
    pub resref_of_sound_1: Resref,
    pub resref_of_sound_2: Resref,
    pub resref_of_sound_3: Resref,
    pub resref_of_sound_4: Resref,
    pub resref_of_sound_5: Resref,
    pub resref_of_sound_6: Resref,
    pub resref_of_sound_7: Resref,
    pub resref_of_sound_8: Resref,
    pub resref_of_sound_9: Resref,
    pub resref_of_sound_10: Resref,
    pub count_of_sounds: u16,
    #[serde(skip)]
    _unused_1: u16,
    pub base_time_interval: u32,
    pub base_time_deviation: u32,
    pub ambient_appearence_schedule: u32,
    pub flags: u32,
    #[serde(skip)]
    #[br(count = 64)]
    _unused_2: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Variable
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Variable {
    pub name: CharArray<32>,
    /*
      bit 0: int
      bit 1: float
      bit 2: script name
      bit 3: resref
      bit 4: strref
      bit 5: dword
    */
    pub variable_type: u16,
    pub resource_value: u16,
    pub dword_value: u32,
    pub int_value: u32,
    pub double_value: i64,
    pub script_name_value: CharArray<32>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Explored
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct ExploredBitmask(pub u8);

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Door
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Door {
    pub name: CharArray<32>,
    // Link with WED
    pub door_id: CharArray<8>,
    pub flags: u32,
    pub index_of_first_vertex_of_the_door_outline_when_open: u32,
    pub count_of_vertices_of_the_door_outline_when_open: u16,
    pub count_of_vertices_of_the_door_outline_when_closed: u16,
    pub index_of_first_vertex_of_the_door_outline_when_closed: u32,
    pub minimum_bounding_box_of_the_door_polygon_when_open: [u16; 4],
    pub minimum_bounding_box_of_the_door_polygon_when_closed: [u16; 4],
    pub index_of_first_vertex_in_the_impeded_cell_block_when_open: u32,
    pub count_of_vertices_in_impeded_cell_block_when_open: u16,
    pub count_of_vertices_in_impeded_cell_block_when_closed: u16,
    pub index_of_first_vertex_in_the_impeded_cell_block_when_closed: u32,
    pub hit_points: u16,
    pub armor_class: u16,
    pub door_open_sound: Resref,
    pub door_close_sound: Resref,
    pub cursor_index: u32,
    pub trap_detection_difficulty: u16,
    pub trap_removal_difficulty: u16,
    // 0=No, 1=Yes
    pub door_is_trapped: u16,
    // 0=No, 1=Yes
    pub trap_detected: u16,
    pub trap_launch_target_x_coordinate: u16,
    pub trap_launch_target_y_coordinate: u16,
    pub key_item: Resref,
    pub door_script: Resref,
    // Secret doors
    pub detection_difficulty: u32,
    pub lock_difficulty: u32,
    pub two_points: [u16; 4],
    pub lockpick_string: Strref,
    pub travel_trigger_name: CharArray<24>,
    pub dialog_speaker_name: Strref,
    pub dialog_resref: Resref,
    #[serde(skip)]
    #[br(count = 8)]
    _unknown: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Anim
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Animation {
    pub name: CharArray<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub animation_appearence_schedule: u32,
    // bgee: bam/wbm/pvrz, others: bam
    pub animation_resref: Resref,
    pub bam_sequence_number: u16,
    pub bam_frame_number: u16,
    pub flags: u32,
    pub height: u16,
    pub transparency: u16,
    // 0 indicates random frame. synchronized will clear this
    pub starting_frame: u16,
    // 0 defaults to 100
    pub chance_of_looping: u8,
    pub skip_cycles: u8,
    pub palette: Resref,
    // note: only required for wbm and pvrz resources (see flags bit 13/15)
    pub animation_width: u16,
    // only required for wbm and pvrz resources (see flags bit 13/15)
    pub animation_height: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Automap
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct AutomapNotesBGEE {
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub note_text: Strref,
    //  0=external (toh/tot) or 1=internal (tlk)
    pub strref_location: u16,
    // bg2
    pub colour: u16,
    pub note_count: u32,
    #[serde(skip)]
    #[br(count = 36)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_TiledObj
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct TiledObject {
    pub name: CharArray<32>,
    pub tile_id: Resref,
    pub flags: u32,
    pub offset_to_open_search_squares: u32,
    pub count_of_open_search_squares: u16,
    pub count_of_closed_search_squares: u16,
    pub offset_to_closed_search_squares: u32,
    #[serde(skip)]
    #[br(count = 48)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_ProjTraps
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct ProjectileTrap {
    pub projectile_resref: Resref,
    pub effect_block_offset: u32,
    pub effect_block_size: u16,
    pub missile_ids_reference: u16,
    pub ticks_until_next_trigger_check: u16,
    pub triggers_remaining: u16,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub z_coordinate: u16,
    pub enemy_ally_targetting: u8,
    pub party_member_index: u8,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Song_entries
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SongEntry {
    pub day_song_reference_number: u32,
    pub night_song_reference_number: u32,
    pub win_song_reference_number: u32,
    pub battle_song_reference_number: u32,
    pub lose_song_reference_number: u32,
    pub alt_music_1: u32,
    pub alt_music_2: u32,
    pub alt_music_3: u32,
    pub alt_music_4: u32,
    pub alt_music_5: u32,
    pub main_day_ambient_1: Resref,
    pub main_day_ambient_2: Resref,
    pub main_day_ambient_volume_percent: u32,
    pub main_night_ambient_1: Resref,
    pub main_night_ambient_2: Resref,
    pub main_night_ambient_volume_percent: u32,
    pub reverb_or_unused: u32,
    #[serde(skip)]
    #[br(count = 60)]
    _unused: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Rest_Interruptions
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct RestInterruption {
    pub name: CharArray<32>,
    pub interruption_explanation_text: CharArray<40>,
    #[br(count = 10)]
    pub resref_of_creature_to_spawn: Vec<Resref>,
    pub count_of_creatures_in_spawn_table: u16,
    pub difficulty: u16,
    pub removal_time: u32,
    pub movement_restriction_distance: u16,
    pub movement_restriction_distance_move_to_object: u16,
    pub maximum_number_of_creatures_to_spawn: u16,
    //  0=inactive, 1=active
    pub interruption_point_enabled: u16,
    pub probability_day_per_hour: u16,
    pub probability_night_per_hour: u16,
    #[serde(skip)]
    #[br(count = 56)]
    _unused: Vec<u8>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use binrw::io::{BufReader, Read};
    use std::fs::File;

    #[test]
    fn test_ambients() {
        let file = File::open("fixtures/ar0011.are").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let area: Area = Area::new(&buffer);
        assert_eq!(
            area.ambients[0].name,
            "Main Ambient\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".into()
        );
        assert_eq!(area.ambients[0].resref_of_sound_1, "AM0011\0\0".into());
        assert_eq!(
            area.ambients[1].name,
            "SS-wispers\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".into()
        );
    }

    #[test]
    fn test_projectile_traps() {
        let file = File::open("fixtures/ar0002.are").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let area: Area = Area::new(&buffer);
        assert_eq!(area.projectile_traps, vec![])
    }

    #[test]
    fn test_actors() {
        let file = File::open("fixtures/ar0002.are").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let area: Area = Area::new(&buffer);
        assert_eq!(
            area.actors[0],
            Actor {
                name: "Priest of Helm\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".into(),
                current_x_coordinate: 446,
                current_y_coordinate: 333,
                destination_x_coordinate: 446,
                destination_y_coordinate: 333,
                flags: 1,
                has_been_spawned: 0,
                first_letter_of_cre_resref: 0,
                _unused_1: 0,
                actor_animation: 24576,
                actor_orientation: 0,
                _unused: 0,
                actor_removal_timer: 4294967295,
                movement_restriction_distance: 0,
                movement_restriction_distance_move_to_object: 0,
                actor_appearence_schedule: 4294967295,
                num_times_talked_to: 0,
                dialog: "\0\0\0\0\0\0\0\0".into(),
                script_override: "\0\0\0\0\0\0\0\0".into(),
                script_general: "\0\0\0\0\0\0\0\0".into(),
                script_class: "\0\0\0\0\0\0\0\0".into(),
                script_race: "\0\0\0\0\0\0\0\0".into(),
                script_default: "\0\0\0\0\0\0\0\0".into(),
                script_specific: "\0\0\0\0\0\0\0\0".into(),
                cre_file: "PRIHEL\0\0".into(),
                offset_to_cre_structure: 0,
                size_of_stored_cre_structure: 0,
                _unused_2: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ]
            }
        )
    }

    #[test]
    fn test_spawn_point() {
        let file = File::open("fixtures/ar0226.are").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let automapnotes = Area::new(&buffer).automap_notes;
        assert_eq!(automapnotes, vec![])
    }
}
