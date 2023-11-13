use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::common::header::Header;
use crate::item_table::ItemReferenceTable;
use crate::model::Model;
use crate::resources::utils::{
    copy_buff_to_struct, copy_transmute_buff, to_u8_slice, vec_to_u8_slice,
};
use crate::tlk::Lookup;
use crate::{common::fixed_char_array::FixedCharSlice, game::GlobalVariables};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm
#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    pub header: FileHeader,
    pub actors: Vec<Actor>,
    pub regions: Vec<Region>,
    pub spawn_points: Vec<SpawnPoint>,
    pub entrances: Vec<Entrance>,
    pub containers: Vec<Container>,
    pub items: Vec<ItemReferenceTable>,
    pub vertices: Vec<Vertice>,
    pub ambients: Vec<Ambient>,
    pub variables: Vec<AreaVariable>,
    pub explored_bitmasks: Vec<ExploredBitmask>,
    pub doors: Vec<Door>,
    pub animations: Vec<Animation>,
    pub automap_notes: Vec<AutomapNotesBGEE>,
    pub tiled_objects: Vec<TiledObject>,
    pub tiled_object_flags: Vec<TiledObject>,
    pub projectile_traps: Vec<ProjectileTrap>,
    pub songs: Vec<SongEntry>,
    pub rest_interruptions: Vec<RestInterruption>,
}

impl Model for Area {
    fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<FileHeader>(buffer, 0);

        let start = usize::try_from(header.offset_to_actors).unwrap_or(0);
        let count = usize::try_from(header.count_of_actors).unwrap_or(0);
        let actors = copy_transmute_buff::<Actor>(buffer, start, count);

        let start = usize::try_from(header.offset_to_regions).unwrap_or(0);
        let count = usize::try_from(header.count_of_regions).unwrap_or(0);
        let regions = copy_transmute_buff::<Region>(buffer, start, count);

        let start = usize::try_from(header.offset_to_spawn_points).unwrap_or(0);
        let count = usize::try_from(header.count_of_spawn_points).unwrap_or(0);
        let spawn_points = copy_transmute_buff::<SpawnPoint>(buffer, start, count);

        let start = usize::try_from(header.offset_to_entrances).unwrap_or(0);
        let count = usize::try_from(header.count_of_entrances).unwrap_or(0);
        let entrances = copy_transmute_buff::<Entrance>(buffer, start, count);

        let start = usize::try_from(header.offset_to_containers).unwrap_or(0);
        let count = usize::try_from(header.count_of_containers).unwrap_or(0);
        let containers = copy_transmute_buff::<Container>(buffer, start, count);

        let start = usize::try_from(header.offset_to_items).unwrap_or(0);
        let count = usize::try_from(header.count_of_items).unwrap_or(0);
        let items = copy_transmute_buff::<ItemReferenceTable>(buffer, start, count);

        let start = usize::try_from(header.offset_to_vertices).unwrap_or(0);
        let count = usize::try_from(header.count_of_vertices).unwrap_or(0);
        let vertices = copy_transmute_buff::<Vertice>(buffer, start, count);

        let start = usize::try_from(header.offset_to_ambients).unwrap_or(0);
        let count = usize::try_from(header.count_of_ambients).unwrap_or(0);
        let ambients = copy_transmute_buff::<Ambient>(buffer, start, count);

        let start = usize::try_from(header.offset_to_variables).unwrap_or(0);
        let count = usize::try_from(header.count_of_variables).unwrap_or(0);
        let variables = copy_transmute_buff::<AreaVariable>(buffer, start, count);

        let start = usize::try_from(header.offset_to_tiled_object_flags).unwrap_or(0);
        let count = usize::try_from(header.count_of_tiled_object_flags).unwrap_or(0);
        let tiled_object_flags = copy_transmute_buff::<TiledObject>(buffer, start, count);

        let start = usize::try_from(header.offset_to_doors).unwrap_or(0);
        let count = usize::try_from(header.count_of_doors).unwrap_or(0);
        let doors = copy_transmute_buff::<Door>(buffer, start, count);

        let start = usize::try_from(header.offset_to_animations).unwrap_or(0);
        let count = usize::try_from(header.count_of_animations).unwrap_or(0);
        let animations = copy_transmute_buff::<Animation>(buffer, start, count);

        let start = usize::try_from(header.offset_to_tiled_objects).unwrap_or(0);
        let count = usize::try_from(header.count_of_tiled_objects).unwrap_or(0);
        let tiled_objects = copy_transmute_buff::<TiledObject>(buffer, start, count);

        let start = usize::try_from(header.offset_to_explored_bitmask).unwrap_or(0);
        let count = usize::try_from(header.size_of_explored_bitmask).unwrap_or(1)
            / std::mem::size_of::<ExploredBitmask>();
        let explored_bitmasks = copy_transmute_buff::<ExploredBitmask>(buffer, start, count);

        let start = usize::try_from(header.offset_to_automap_notes).unwrap_or(0);
        let count = usize::try_from(header.number_of_entries_in_the_automap_notes).unwrap_or(0);
        let automap_notes = copy_transmute_buff::<AutomapNotesBGEE>(buffer, start, count);

        let start = usize::try_from(header.offset_to_projectile_traps).unwrap_or(0);
        let count = usize::try_from(header.number_of_entries_in_the_projectile_traps).unwrap_or(0);
        let projectile_traps = copy_transmute_buff::<ProjectileTrap>(buffer, start, count);

        let start = usize::try_from(header.offset_to_song_entries).unwrap_or(0);
        let count = if start > 0 { 0 } else { start + 144 };
        let songs = copy_transmute_buff::<SongEntry>(buffer, start, count);

        let start = usize::try_from(header.offset_to_rest_interruptions).unwrap_or(0);
        let count = if start > 0 { 0 } else { start + 228 };
        let rest_interruptions = copy_transmute_buff::<RestInterruption>(buffer, start, count);

        Self {
            header,
            actors,
            regions,
            spawn_points,
            entrances,
            containers,
            items,
            vertices,
            ambients,
            variables,
            tiled_object_flags,
            doors,
            animations,
            tiled_objects,
            explored_bitmasks,
            automap_notes,
            projectile_traps,
            songs,
            rest_interruptions,
        }
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        self.header.area_wed.to_string().replace(".WED", ".ARE")
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![
            (1, to_u8_slice(&self.header).to_vec()),
            (
                self.header.offset_to_actors as i32,
                vec_to_u8_slice(&self.actors),
            ),
            (
                self.header.offset_to_regions,
                vec_to_u8_slice(&self.regions),
            ),
            (
                self.header.offset_to_spawn_points,
                vec_to_u8_slice(&self.spawn_points),
            ),
            (
                self.header.offset_to_entrances,
                vec_to_u8_slice(&self.entrances),
            ),
            (
                self.header.offset_to_containers,
                vec_to_u8_slice(&self.containers),
            ),
            (self.header.offset_to_items, vec_to_u8_slice(&self.items)),
            (
                self.header.offset_to_vertices,
                vec_to_u8_slice(&self.vertices),
            ),
            (
                self.header.offset_to_ambients,
                vec_to_u8_slice(&self.ambients),
            ),
            (
                self.header.offset_to_variables,
                vec_to_u8_slice(&self.variables),
            ),
            (
                self.header.offset_to_tiled_object_flags as i32,
                vec_to_u8_slice(&self.tiled_object_flags),
            ),
            (
                self.header.offset_to_explored_bitmask as i32,
                vec_to_u8_slice(&self.explored_bitmasks),
            ),
            (self.header.offset_to_doors, vec_to_u8_slice(&self.doors)),
            (
                self.header.offset_to_animations,
                vec_to_u8_slice(&self.animations),
            ),
            (
                self.header.offset_to_tiled_objects,
                vec_to_u8_slice(&self.tiled_objects),
            ),
            (
                self.header.offset_to_song_entries,
                vec_to_u8_slice(&self.songs),
            ),
            (
                self.header.offset_to_rest_interruptions,
                vec_to_u8_slice(&self.rest_interruptions),
            ),
            (
                self.header.number_of_entries_in_the_automap_notes,
                vec_to_u8_slice(&self.automap_notes),
            ),
            (
                self.header.number_of_entries_in_the_automap_notes,
                vec_to_u8_slice(&self.projectile_traps),
            ),
        ];
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out.into_iter()
            .filter(|data| data.0 < 1 && !data.1.is_empty())
            .flat_map(|(_order, data)| data)
            .collect()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Header
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct FileHeader {
    pub header: Header<4, 4>,
    pub area_wed: FixedCharSlice<8>,
    pub last_saved: u32,
    pub area_flags: u32,
    pub resref_of_the_area_to_the_north_of_this_area: FixedCharSlice<8>,
    pub north_area_flags: u32,
    pub resref_of_the_area_to_the_east_of_this_area: FixedCharSlice<8>,
    pub east_area_flags: u32,
    pub resref_of_the_area_to_the_south_of_this_area: FixedCharSlice<8>,
    pub south_area_flags: u32,
    pub resref_of_the_area_to_the_west_of_this_area: FixedCharSlice<8>,
    pub west_area_flags: u32,
    pub area_type_flags: u16,
    pub rain_probability: u16,
    pub snow_probability: u16,
    // bgee only
    pub fog_probability: u16,
    pub lightning_probability: u16,
    pub wind_speed: u16,
    pub offset_to_actors: u32,
    pub count_of_actors: i16,
    pub count_of_regions: i16,
    pub offset_to_regions: i32,
    pub offset_to_spawn_points: i32,
    pub count_of_spawn_points: i32,
    pub offset_to_entrances: i32,
    pub count_of_entrances: i32,
    pub offset_to_containers: i32,
    pub count_of_containers: i16,
    pub count_of_items: i16,
    pub offset_to_items: i32,
    pub offset_to_vertices: i32,
    pub count_of_vertices: i16,
    pub count_of_ambients: i16,
    pub offset_to_ambients: i32,
    pub offset_to_variables: i32,
    pub count_of_variables: i32,
    pub offset_to_tiled_object_flags: i16,
    pub count_of_tiled_object_flags: i16,
    pub area_script: FixedCharSlice<8>,
    pub size_of_explored_bitmask: u32,
    pub offset_to_explored_bitmask: u32,
    pub count_of_doors: i32,
    pub offset_to_doors: i32,
    pub count_of_animations: i32,
    pub offset_to_animations: i32,
    pub count_of_tiled_objects: i32,
    pub offset_to_tiled_objects: i32,
    pub offset_to_song_entries: i32,
    pub offset_to_rest_interruptions: i32,
    pub offset_to_automap_notes: i32,
    pub number_of_entries_in_the_automap_notes: i32,
    pub offset_to_projectile_traps: i32,
    pub number_of_entries_in_the_projectile_traps: i32,
    // bgee and bg2:tob
    pub rest_movie_day: FixedCharSlice<8>,
    // bgee and bg2:tob
    pub rest_movie_night: FixedCharSlice<8>,
    #[serde(skip)]
    _unused: FixedCharSlice<56>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Actor
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
pub struct Actor {
    pub name: FixedCharSlice<32>,
    pub current_x_coordinate: u16,
    pub current_y_coordinate: u16,
    pub destination_x_coordinate: u16,
    pub destination_y_coordinate: u16,
    pub flags: u32,
    pub has_been_spawned: u16,
    pub first_letter_of_cre_resref: FixedCharSlice<1>,
    #[serde(skip)]
    _unused_1: u8,
    pub actor_animation: u32,
    pub actor_orientation: u16,
    _unused: u16,
    pub actor_removal_timer: u32,
    pub movement_restriction_distance: u16,
    pub movement_restriction_distance_move_to_object: u16,
    pub actor_appearence_schedule: u32,
    pub num_times_talked_to: u32,
    pub dialog: FixedCharSlice<8>,
    pub script_override: FixedCharSlice<8>,
    pub script_general: FixedCharSlice<8>,
    pub script_class: FixedCharSlice<8>,
    pub script_race: FixedCharSlice<8>,
    pub script_default: FixedCharSlice<8>,
    pub script_specific: FixedCharSlice<8>,
    pub cre_file: FixedCharSlice<8>,
    // for embedded cre files
    pub offset_to_cre_structure: u32,
    pub size_of_stored_cre_structure: u32,
    #[serde(skip)]
    _unused_2: FixedCharSlice<128>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Info
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: FixedCharSlice<32>,
    pub region_type: u16,
    pub minimum_bounding_box_of_this_point: [u16; 4],
    pub count_of_vertices_composing_the_perimeter: u16,
    pub index_to_first_vertex: u32,
    pub trigger_value: u32,
    pub cursor_index: u32,
    // for travel regions
    pub destination_area: FixedCharSlice<8>,
    // for travel regions
    pub entrance_name_in_destination_area: FixedCharSlice<32>,
    pub flags: u32,
    // for info points
    pub information_text: FixedCharSlice<4>,
    pub trap_detection_difficulty_percent: u16,
    pub trap_removal_difficulty_percent: u16,
    // 0=no, 1=yes
    pub region_is_trapped: u16,
    // 0=no, 1=yes
    pub trap_detected: u16,
    pub trap_launch_location: [u16; 2],
    pub key_item: FixedCharSlice<8>,
    pub region_script: FixedCharSlice<8>,
    pub alternative_use_point_x_coordinate: u16,
    pub alternative_use_point_y_coordinate: u16,
    #[serde(skip)]
    _unknown_1: u32,
    #[serde(skip)]
    _unknown_2: [u8; 32],
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Spawn
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct SpawnPoint {
    pub name: FixedCharSlice<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub resref_of_creature_to_spawn_1st: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_2nd: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_3rd: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_4th: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_5th: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_6th: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_7th: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_8th: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_9th: FixedCharSlice<8>,
    pub resref_of_creature_to_spawn_10th: FixedCharSlice<8>,
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
    _unused: FixedCharSlice<38>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Entrance
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Entrance {
    pub name: FixedCharSlice<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub orientation: u16,
    #[serde(skip)]
    _unused: FixedCharSlice<66>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Container
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Container {
    pub name: FixedCharSlice<32>,
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
    pub trap_script: FixedCharSlice<8>,
    pub index_to_first_vertex_of_the_outline: u32,
    pub count_of_vertices_making_up_the_outline: u16,
    pub trigger_range: u16,
    pub owner_script_name: FixedCharSlice<32>,
    pub key_item: FixedCharSlice<8>,
    pub break_difficulty: u32,
    pub lockpick_string: FixedCharSlice<4>,
    #[serde(skip)]
    _unused: FixedCharSlice<56>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Item
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct AreaItem(pub ItemReferenceTable);

// An array of points used to create the outlines of regions and containers. Elements are 16-bit words stored x0, y0, x1, y1 etc.
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Vertice(pub u16);

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Ambient
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Ambient {
    pub name: FixedCharSlice<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub radius: u16,
    pub height: u16,
    pub pitch_variance: u32,
    pub volume_variance: u16,
    pub volume_percentage: u16,
    pub resref_of_sound_1: FixedCharSlice<8>,
    pub resref_of_sound_2: FixedCharSlice<8>,
    pub resref_of_sound_3: FixedCharSlice<8>,
    pub resref_of_sound_4: FixedCharSlice<8>,
    pub resref_of_sound_5: FixedCharSlice<8>,
    pub resref_of_sound_6: FixedCharSlice<8>,
    pub resref_of_sound_7: FixedCharSlice<8>,
    pub resref_of_sound_8: FixedCharSlice<8>,
    pub resref_of_sound_9: FixedCharSlice<8>,
    pub resref_of_sound_10: FixedCharSlice<8>,
    pub count_of_sounds: u16,
    #[serde(skip)]
    _unused_1: u16,
    pub base_time_interval: u32,
    pub base_time_deviation: u32,
    pub ambient_appearence_schedule: u32,
    pub flags: u32,
    #[serde(skip)]
    _unused_2: FixedCharSlice<64>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Variable
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct AreaVariable(pub GlobalVariables);

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Explored
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct ExploredBitmask(pub u8);

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Door
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Door {
    pub name: FixedCharSlice<32>,
    // Link with WED
    pub door_id: FixedCharSlice<8>,
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
    pub door_open_sound: FixedCharSlice<8>,
    pub door_close_sound: FixedCharSlice<8>,
    pub cursor_index: u32,
    pub trap_detection_difficulty: u16,
    pub trap_removal_difficulty: u16,
    // 0=No, 1=Yes
    pub door_is_trapped: u16,
    // 0=No, 1=Yes
    pub trap_detected: u16,
    pub trap_launch_target_x_coordinate: u16,
    pub trap_launch_target_y_coordinate: u16,
    pub key_item: FixedCharSlice<8>,
    pub door_script: FixedCharSlice<8>,
    // Secret doors
    pub detection_difficulty: u32,
    pub lock_difficulty: u32,
    pub two_points: [u16; 4],
    pub lockpick_string: FixedCharSlice<4>,
    pub travel_trigger_name: FixedCharSlice<24>,
    pub dialog_speaker_name: FixedCharSlice<4>,
    pub dialog_resref: FixedCharSlice<8>,
    #[serde(skip)]
    _unknown: FixedCharSlice<8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Anim
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: FixedCharSlice<32>,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub animation_appearence_schedule: u32,
    // bgee: bam/wbm/pvrz, others: bam
    pub animation_resref: FixedCharSlice<8>,
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
    pub palette: FixedCharSlice<8>,
    // note: only required for wbm and pvrz resources (see flags bit 13/15)
    pub animation_width: u16,
    // only required for wbm and pvrz resources (see flags bit 13/15)
    pub animation_height: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Automap
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct AutomapNotesBGEE {
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub note_text: FixedCharSlice<4>,
    //  0=external (toh/tot) or 1=internal (tlk)
    pub strref_location: u16,
    // bg2
    pub colour: u16,
    pub note_count: u32,
    #[serde(skip)]
    _unused: FixedCharSlice<36>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_TiledObj
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct TiledObject {
    pub name: FixedCharSlice<32>,
    pub tile_id: FixedCharSlice<8>,
    pub flags: u32,
    pub offset_to_open_search_squares: u32,
    pub count_of_open_search_squares: u16,
    pub count_of_closed_search_squares: u16,
    pub offset_to_closed_search_squares: u32,
    #[serde(skip)]
    _unused: FixedCharSlice<48>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_ProjTraps
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct ProjectileTrap {
    pub projectile_resref: FixedCharSlice<8>,
    pub effect_block_offset: u32,
    pub effect_block_size: u16,
    pub missile_ids_reference: u16,
    pub ticks_until_next_trigger_check: u16,
    pub triggers_remaining: u16,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
    pub z_coordinate: u16,
    pub enemy_ally_targetting: u8,
    pub creatorparty_member_index: u8,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Song_entries
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
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
    pub main_day_ambient_1: FixedCharSlice<8>,
    pub main_day_ambient_2: FixedCharSlice<8>,
    pub main_day_ambient_volume_percent: u32,
    pub main_night_ambient_1: FixedCharSlice<8>,
    pub main_night_ambient_2: FixedCharSlice<8>,
    pub main_night_ambient_volume_percent: u32,
    pub reverb_or_unused: u32,
    #[serde(skip)]
    _unused: FixedCharSlice<60>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm#formAREAV1_0_Rest_Interruptions
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct RestInterruption {
    pub name: FixedCharSlice<32>,
    pub interruption_explanation_text: [u32; 10],
    pub resref_of_creature_to_spawn: [u64; 10],
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
    _unused: FixedCharSlice<56>,
}
