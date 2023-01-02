use std::{mem::size_of, rc::Rc};

use crate::{
    common::fixed_char_array::FixedCharSlice,
    creature::Creature,
    model::Model,
    utils::{copy_buff_to_struct, copy_transmute_buff},
};

#[derive(Debug)]
pub struct Game {
    pub header: BGEEGameHeader,
    pub party_npcs: Vec<Npc>,
    pub non_party_npcs: Vec<Npc>,
    pub global_varriables: Vec<GlobalVarriables>,
    pub journal_entries: Vec<JournalEntries>,
    pub familiar: Familiar,
    pub stored_locations: Vec<Location>,
    pub pocket_plane_locations: Vec<Location>,
    pub familiar_extra: Vec<FamiliarExtra>,
}

impl Model for Game {
    fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<BGEEGameHeader>(buffer, 0);

        // NPCs
        let start: usize =
            usize::try_from(header.offset_to_npc_structs_for_party_members).unwrap_or(0);
        let count: usize =
            usize::try_from(header.count_of_npc_structs_for_party_members).unwrap_or(0);
        let party_npcs = generate_npcs(buffer, start, count);
        let start: usize = usize::try_from(header.offset_to_npc_structs_for_npcs).unwrap_or(0);
        let count: usize = usize::try_from(header.count_of_npc_structs_for_npcs).unwrap_or(0);
        let non_party_npcs = generate_npcs(buffer, start, count);

        let start: usize =
            usize::try_from(header.offset_to_global_namespace_variables).unwrap_or(0);
        let count: usize = usize::try_from(header.count_of_global_namespace_variables).unwrap_or(0);
        let global_varriables = copy_transmute_buff::<GlobalVarriables>(buffer, start, count);

        let start: usize = usize::try_from(header.offset_to_journal_entries).unwrap_or(0);
        let count: usize = usize::try_from(header.count_of_journal_entries).unwrap_or(0);
        let journal_entries = copy_transmute_buff::<JournalEntries>(buffer, start, count);

        // Familar
        let start: usize = usize::try_from(header.offset_to_familar).unwrap_or(0);
        let familiar = copy_buff_to_struct::<Familiar>(buffer, start);

        let exists =
            usize::try_from(familiar.offset_to_familiar_resources).unwrap_or(0) - buffer.len();
        let familiar_extra = if exists > 0 {
            let start: usize = usize::try_from(familiar.offset_to_familiar_resources).unwrap_or(0);
            let count: usize = exists / size_of::<FamiliarExtra>();
            if count > 0 {
                copy_transmute_buff::<FamiliarExtra>(buffer, start, count)
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let start: usize = usize::try_from(header.offset_to_stored_locations).unwrap_or(0);
        let count: usize = party_npcs.len();
        let stored_locations = copy_transmute_buff::<Location>(buffer, start, count);

        let start: usize = usize::try_from(header.offset_to_pocket_plane_locations).unwrap_or(0);
        let count: usize = party_npcs.len();
        let pocket_plane_locations = copy_transmute_buff::<Location>(buffer, start, count);

        Self {
            header,
            party_npcs,
            non_party_npcs,
            global_varriables,
            journal_entries,
            familiar,
            stored_locations,
            pocket_plane_locations,
            familiar_extra,
        }
    }
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BGEEGameHeader {
    pub signature: FixedCharSlice<4>,
    pub version: FixedCharSlice<4>,
    pub game_time: u32,
    pub selected_formation: u16,
    pub formation_button_1: u16,
    pub formation_button_2: u16,
    pub formation_button_3: u16,
    pub formation_button_4: u16,
    pub formation_button_5: u16,
    pub party_gold: u32,
    pub view_of_party_member: u16,
    /*
    Weather bitfield
    bit0: Rain
    bit1: Snow
    bit2: Light Rain
    bit3: Medium Rain
    bit4: Light Wind
    bit5: Medium Wind
    bit6: Rare Lightning
    bit7: Lightning
    bit8: Storm Increasing
    bits 9->15 unused
    */
    pub weather_bitfield: [u8; 2],
    pub offset_to_npc_structs_for_party_members: i32,
    pub count_of_npc_structs_for_party_members: i32,
    pub offset_to_party_inventory: i32,
    pub count_of_party_inventory: i32,
    pub offset_to_npc_structs_for_npcs: i32,
    pub count_of_npc_structs_for_npcs: i32,
    pub offset_to_global_namespace_variables: i32,
    pub count_of_global_namespace_variables: i32,
    pub main_area: [u8; 8],
    pub offset_to_familiar_extra: i32,
    pub count_of_journal_entries: i32,
    pub offset_to_journal_entries: i32,
    pub party_reputation: i32,
    pub current_area: [u8; 8],
    pub gui_flags: [u8; 4],
    pub loading_progress: [u8; 4],
    pub offset_to_familar: i32,
    pub offset_to_stored_locations: i32,
    pub count_of_stored_locations: i32,
    pub game_time_real_seconds: i32,
    pub offset_to_pocket_plane_locations: i32,
    pub count_of_pocket_plane_locations: i32,
    // EE fields
    pub zoom_level: u32,
    pub random_encounter_area: [u8; 8],
    pub current_world_map: [u8; 8],
    pub familiar_owner: u32,
    pub random_encounter_script: [u8; 20],
}

#[derive(Debug, PartialEq, Eq)]
pub struct Npc {
    pub game_npc: GameNPC,
    pub creature: Creature,
}

fn generate_npcs(buffer: &[u8], start: usize, count: usize) -> Vec<Npc> {
    (0..count)
        .into_iter()
        .map(|counter| {
            let start: usize = start + counter * size_of::<GameNPC>();
            let game_npc = copy_buff_to_struct::<GameNPC>(buffer, start);

            let start = game_npc.offset_to_cre_resource as usize;
            let creature_buffer = buffer.get(start..).unwrap();
            let creature = Creature::new(creature_buffer);

            Npc { game_npc, creature }
        })
        .collect()
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_NPC
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct GameNPC {
    pub character_selection: u16,
    // x0-0x5 = player_x_fill, 0x_ffff = not in party
    pub party_order: u16,
    pub offset_to_cre_resource: u32,
    pub size_of_cre_resource: u32,
    pub character_name: FixedCharSlice<8>,
    pub character_orientation: i32,
    pub characters_current_area: [i8; 8],
    pub character_x_coordinate: i16,
    pub character_y_coordinate: i16,
    pub viewing_rectangle_x_coordinate: i16,
    pub viewing_rectangle_y_coordinate: i16,
    pub modal_action: i16,
    pub happiness: u16,
    // all of the num times interacted are not used so we just group them
    pub num_times_interacted: [u32; 24],
    // (0x_ffff = none)
    pub index_of_quick_weapon_1: i16,
    // (0x_ffff = none)
    pub index_of_quick_weapon_2: i16,
    // (0x_ffff = none)
    pub index_of_quick_weapon_3: i16,
    // (0x_ffff = none)
    pub index_of_quick_weapon_4: i16,
    // (0/1/2 or -1 disabled)
    pub quick_weapon_slot_1_ability: i16,
    // (0/1/2 or -1 disabled)
    pub quick_weapon_slot_2_ability: i16,
    // (0/1/2 or -1 disabled)
    pub quick_weapon_slot_3_ability: i16,
    // (0/1/2 or -1 disabled)
    pub quick_weapon_slot_4_ability: i16,
    pub quick_spell_1_resouce: i64,
    pub quick_spell_2_resouce: i64,
    pub quick_spell_3_resouce: i64,
    // (0x_ffff = none)
    pub index_of_quick_item_1: i16,
    // (0x_ffff = none)
    pub index_of_quick_item_2: i16,
    // (0x_ffff = none)
    pub index_of_quick_item_3: i16,
    // (0/1/2 or -1 disabled)
    pub quick_item_slot_1_ability: i16,
    // (0/1/2 or -1 disabled)
    pub quick_item_slot_2_ability: i16,
    // (0/1/2 or -1 disabled)
    pub quick_item_slot_3_ability: i16,
    pub name: FixedCharSlice<32>,
    pub talk_count: i32,
    pub character_kill_stats: CharacterKillStats,
    // filename prefix for voice set
    pub voice_set: [i8; 8],
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Stats
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CharacterKillStats {
    pub most_powerful_vanquished_name: u32,
    pub most_powerful_vanquished_xp_reward: u32,
    // 1/15 seconds
    pub time_in_party: u32,
    // 1/15 seconds
    pub time_joined: u32,
    // 0 = not in party, 1 = in party
    pub party_member: u8,
    pub unused: u16,
    // changed to *
    pub first_letter_of_cre_resref: u8,
    pub chapter_kills_xp_gained: i32,
    pub chapter_kills_number: i32,
    pub game_kills_xp_gained: i32,
    pub game_kills_number: i32,
    pub favourite_spells: [u64; 4],
    pub favourite_spell_count: [u16; 4],
    pub favourite_weapons: [u64; 4],
    // time equipped in combat - 1/15 seconds
    pub favourite_weapon_time: [u16; 4],
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Variable
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct GlobalVarriables {
    pub name: [i8; 32],
    /*
      bit 0: int
      bit 1: float
      bit 2: script name
      bit 3: resref
      bit 4: strref
      bit 5: dword
    */
    pub varriable_type: [i8; 2],
    pub resource_value: i16,
    pub dword_value: i32,
    pub int_value: i32,
    pub double_value: i64,
    pub script_name_value: [i8; 32],
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Journal
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct JournalEntries {
    pub journal_text: [u8; 4],
    // seconds
    pub time: u32,
    pub current_chapter_number: u8,
    pub read_by_character: u8,
    /*
    bit 0: Quests
    bit 1: Competed quest
    bit 2: Journal Info
    bit 4: Journal Info and Completed?
    NB. If no bits are set, the entry is a user-note
    */
    pub journal_section: u8,
    // 0x1F = external TOT/TOH, 0xFF = internal TLK
    pub location_flag: u8,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Familiar
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Familiar {
    pub lawful_good_familiar: FixedCharSlice<8>,
    pub lawful_neutral_familiar: FixedCharSlice<8>,
    pub lawful_evil_familiar: FixedCharSlice<8>,
    pub neutral_good_familiar: FixedCharSlice<8>,
    pub neutral_familiar: FixedCharSlice<8>,
    pub neutral_evil_familiar: FixedCharSlice<8>,
    pub chaotic_good_familiar: FixedCharSlice<8>,
    pub chaotic_neutral_familiar: FixedCharSlice<8>,
    pub chaotic_evil_familiar: FixedCharSlice<8>,
    pub offset_to_familiar_resources: i32,
    pub lg_familiar_spell_count: [u32; 9],
    pub ln_familiar_spell_count: [u32; 9],
    pub cg_familiar_spell_count: [u32; 9],
    pub ng_familiar_spell_count: [u32; 9],
    pub tn_familiar_spell_count: [u32; 9],
    pub ne_familiar_spell_count: [u32; 9],
    pub le_familiar_spell_count: [u32; 9],
    pub cn_familiar_spell_count: [u32; 9],
    pub ce_familiar_spell_count: [u32; 9],
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Stored
// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_PocketPlane
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Location {
    pub area: [i8; 8],
    pub x_coordinate: i16,
    pub y_coordinate: i16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_FamiliarExtra
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FamiliarExtra {
    pub data: [i8; 8],
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_headers_parsed() {
        let file = File::open("fixtures/BG2EEBALDUR.gam").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let header = Game::new(&buffer).header;
        assert_eq!(header.signature, "GAME".into());
        assert_eq!(header.version, "V2.0".into());
        assert_eq!({ header.party_gold }, 109741);
        assert_eq!({ header.game_time }, 1664811);
        assert_eq!({ header.count_of_journal_entries }, 188);
        assert_eq!({ header.game_time_real_seconds }, 2774117);
        assert_eq!({ header.zoom_level }, 58);
        assert_eq!({ header.familiar_owner }, 0);
    }

    #[test]
    fn valid_party_parsed() {
        let file = File::open("fixtures/BG2EEBALDUR.gam").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let game = Game::new(&buffer);
        let party = game.party_npcs;
        assert_ne!(party.first(), None);
        if let Some(party) = party.last() {
            assert_eq!(
                party.game_npc,
                GameNPC {
                    character_selection: 0,
                    party_order: 4,
                    offset_to_cre_resource: 71292,
                    size_of_cre_resource: 17084,
                    character_name: "*ERIE6".into(),
                    character_orientation: 6,
                    characters_current_area: [65, 82, 48, 56, 48, 48, 0, 0],
                    character_x_coordinate: 1000,
                    character_y_coordinate: 366,
                    viewing_rectangle_x_coordinate: 366,
                    viewing_rectangle_y_coordinate: 81,
                    modal_action: 0,
                    happiness: 80,
                    num_times_interacted: [0; 24],
                    index_of_quick_weapon_1: 35,
                    index_of_quick_weapon_2: 36,
                    index_of_quick_weapon_3: 10,
                    index_of_quick_weapon_4: 10,
                    quick_weapon_slot_1_ability: 0,
                    quick_weapon_slot_2_ability: 0,
                    quick_weapon_slot_3_ability: 0,
                    quick_weapon_slot_4_ability: 0,
                    quick_spell_1_resouce: 14126745667457107,
                    quick_spell_2_resouce: 15536323869233235,
                    quick_spell_3_resouce: 0,
                    index_of_quick_item_1: 15,
                    index_of_quick_item_2: -1,
                    index_of_quick_item_3: -1,
                    quick_item_slot_1_ability: 0,
                    quick_item_slot_2_ability: -1,
                    quick_item_slot_3_ability: -1,
                    name: "".into(),
                    talk_count: 6,
                    character_kill_stats: CharacterKillStats {
                        most_powerful_vanquished_name: 34947,
                        most_powerful_vanquished_xp_reward: 64000,
                        time_in_party: 8320583,
                        time_joined: 24563462,
                        party_member: 1,
                        unused: 0,
                        first_letter_of_cre_resref: 42,
                        chapter_kills_xp_gained: 1551,
                        chapter_kills_number: 5,
                        game_kills_xp_gained: 153155,
                        game_kills_number: 113,
                        favourite_spells: [
                            14127836589150291,
                            15535232947540051,
                            15815591383289939,
                            15815599822688339
                        ],
                        favourite_spell_count: [8, 1, 57, 33],
                        favourite_weapons: [
                            54083508262210,
                            21472643031384407,
                            1414744390,
                            20070800066560343
                        ],
                        favourite_weapon_time: [1910, 126, 1716, 11844]
                    },
                    voice_set: [0; 8],
                }
            );
        } else {
            panic!();
        }

        let non_party = game.non_party_npcs;
        assert_ne!(non_party.first(), None);
        if let Some(non_party) = non_party.last() {
            assert_eq!(
                non_party.game_npc,
                GameNPC {
                    character_selection: 0,
                    party_order: 65535,
                    offset_to_cre_resource: 196144,
                    size_of_cre_resource: 1868,
                    character_name: "*AZZY8".into(),
                    character_orientation: 14,
                    characters_current_area: [65, 82, 50, 48, 48, 50, 0, 0],
                    character_x_coordinate: 341,
                    character_y_coordinate: 400,
                    viewing_rectangle_x_coordinate: 0,
                    viewing_rectangle_y_coordinate: 0,
                    modal_action: 0,
                    happiness: 0,
                    num_times_interacted: [0; 24],
                    index_of_quick_weapon_1: 11,
                    index_of_quick_weapon_2: 36,
                    index_of_quick_weapon_3: 10,
                    index_of_quick_weapon_4: 10,
                    quick_weapon_slot_1_ability: 0,
                    quick_weapon_slot_2_ability: 0,
                    quick_weapon_slot_3_ability: 0,
                    quick_weapon_slot_4_ability: 0,
                    quick_spell_1_resouce: 0,
                    quick_spell_2_resouce: 0,
                    quick_spell_3_resouce: 0,
                    index_of_quick_item_1: -1,
                    index_of_quick_item_2: -1,
                    index_of_quick_item_3: -1,
                    quick_item_slot_1_ability: -1,
                    quick_item_slot_2_ability: -1,
                    quick_item_slot_3_ability: -1,
                    name: "".into(),
                    talk_count: 1,
                    character_kill_stats: CharacterKillStats {
                        most_powerful_vanquished_name: 4294967295,
                        most_powerful_vanquished_xp_reward: 0,
                        time_in_party: 0,
                        time_joined: 0,
                        party_member: 0,
                        unused: 0,
                        first_letter_of_cre_resref: 42,
                        chapter_kills_xp_gained: 0,
                        chapter_kills_number: 0,
                        game_kills_xp_gained: 0,
                        game_kills_number: 0,
                        favourite_spells: [0, 0, 0, 0],
                        favourite_spell_count: [0, 0, 0, 0],
                        favourite_weapons: [0, 0, 0, 0],
                        favourite_weapon_time: [0, 0, 0, 0]
                    },
                    voice_set: [0; 8],
                }
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn valid_familar_parsed() {
        let file = File::open("fixtures/BG2EEBALDUR.gam").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let familiar = Game::new(&buffer).familiar;
        assert_eq!(familiar.lawful_good_familiar, "FAMPSD".into());
        assert_eq!(familiar.lawful_neutral_familiar, "FAMFER".into());
        assert_eq!(familiar.lawful_evil_familiar, "FAMIMP".into());
        assert_eq!(familiar.neutral_good_familiar, "FAMPSD".into());
        assert_eq!(familiar.neutral_familiar, "FAMRAB".into());
        assert_eq!(familiar.neutral_evil_familiar, "FAMDUST".into());
        assert_eq!(familiar.chaotic_good_familiar, "FAMFAIR".into());
        assert_eq!(familiar.chaotic_neutral_familiar, "FAMCAT".into());
        assert_eq!(familiar.chaotic_evil_familiar, "FAMQUAS".into());
        assert_eq!({ familiar.offset_to_familiar_resources }, 318688);
    }

    #[test]
    fn valid_journal_entries_parsed() {
        let file = File::open("fixtures/BG2EEBALDUR.gam").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let journal = Game::new(&buffer).journal_entries;
        assert_eq!(
            journal.first(),
            Some(&JournalEntries {
                journal_text: [41, 133, 0, 0],
                time: 31595,
                current_chapter_number: 1,
                read_by_character: 255,
                journal_section: 4,
                location_flag: 255
            })
        );
        assert_eq!(
            journal.last(),
            Some(&JournalEntries {
                journal_text: [63, 124, 1, 0],
                time: 24711890,
                current_chapter_number: 6,
                read_by_character: 255,
                journal_section: 1,
                location_flag: 255
            })
        );
    }
}
