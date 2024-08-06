use std::rc::Rc;

use binrw::{
    io::{Cursor, SeekFrom},
    BinRead, BinReaderExt, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::common::{resref::Resref, strref::Strref};
use crate::model::Model;
use crate::tlk::Lookup;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Game {
    #[serde(flatten)]
    pub header: BGEEGameHeader,
    #[serde(flatten)]
    #[br(count=header.count_of_npc_structs_for_party_members, seek_before=SeekFrom::Start(header.offset_to_npc_structs_for_party_members as u64))]
    pub party_npcs: Vec<GameNPC>,
    #[serde(flatten)]
    #[br(count=header.count_of_npc_structs_for_npcs, seek_before=SeekFrom::Start(header.offset_to_npc_structs_for_npcs as u64))]
    pub non_party_npcs: Vec<GameNPC>,
    #[serde(flatten)]
    #[br(count=header.count_of_global_namespace_variables, seek_before=SeekFrom::Start(header.offset_to_global_namespace_variables as u64))]
    pub global_variables: Vec<GlobalVariables>,
    #[serde(flatten)]
    #[br(count=header.count_of_journal_entries, seek_before=SeekFrom::Start(header.offset_to_journal_entries as u64))]
    pub journal_entries: Vec<JournalEntries>,
    #[br(seek_before=SeekFrom::Start(header.offset_to_familiar as u64))]
    pub familiar: Option<Familiar>,
    #[serde(flatten)]
    #[br(count=header.count_of_stored_locations, seek_before=SeekFrom::Start(header.offset_to_stored_locations as u64))]
    pub stored_locations: Vec<Location>,
    #[serde(flatten)]
    #[br(count=header.count_of_pocket_plane_locations, seek_before=SeekFrom::Start(header.offset_to_pocket_plane_locations as u64))]
    pub pocket_plane_locations: Vec<Location>,
    #[serde(flatten)]
    #[br(if(familiar.is_some()), parse_with=binrw::helpers::until_eof, seek_before=SeekFrom::Start(header.offset_to_familiar_extra as u64))]
    pub familiar_extra: Vec<FamiliarExtra>,
}

impl Model for Game {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        "BALDUR.GAM".to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Header
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BGEEGameHeader {
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub signature: String,
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub version: String,
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
    #[br(count = 2)]
    pub weather_bitfield: Vec<u8>,
    pub offset_to_npc_structs_for_party_members: u32,
    pub count_of_npc_structs_for_party_members: u32,
    pub offset_to_party_inventory: u32,
    pub count_of_party_inventory: u32,
    pub offset_to_npc_structs_for_npcs: u32,
    pub count_of_npc_structs_for_npcs: u32,
    pub offset_to_global_namespace_variables: u32,
    pub count_of_global_namespace_variables: u32,
    pub main_area: Resref,
    pub offset_to_familiar_extra: u32,
    pub count_of_journal_entries: u32,
    pub offset_to_journal_entries: u32,
    pub party_reputation: u32,
    pub current_area: Resref,
    #[br(count = 4)]
    pub gui_flags: Vec<u8>,
    #[br(count = 4)]
    pub loading_progress: Vec<u8>,
    pub offset_to_familiar: u32,
    pub offset_to_stored_locations: u32,
    pub count_of_stored_locations: u32,
    pub game_time_real_seconds: u32,
    pub offset_to_pocket_plane_locations: u32,
    pub count_of_pocket_plane_locations: u32,
    // EE fields
    pub zoom_level: u32,
    pub random_encounter_area: Resref,
    pub current_world_map: Resref,
    #[br(count = 8)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub current_campaign: String,
    pub familiar_owner: u32,
    #[br(count = 20)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub random_encounter_script: String,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_NPC
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct GameNPC {
    pub character_selection: u16,
    // x0-0x5 = player_x_fill, 0x_ffff = not in party
    pub party_order: u16,
    pub offset_to_cre_resource: u32,
    pub size_of_cre_resource: u32,
    #[br(count = 8)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub character_name: String,
    pub character_orientation: u32,
    pub characters_current_area: Resref,
    pub character_x_coordinate: u16,
    pub character_y_coordinate: u16,
    pub viewing_rectangle_x_coordinate: u16,
    pub viewing_rectangle_y_coordinate: u16,
    pub modal_action: u16,
    pub happiness: u16,
    // all of the num times interacted are not used so we just group them
    pub num_times_interacted: [u32; 24],
    // (0x_ffff = none)
    pub index_of_quick_weapon_1: u16,
    // (0x_ffff = none)
    pub index_of_quick_weapon_2: u16,
    // (0x_ffff = none)
    pub index_of_quick_weapon_3: u16,
    // (0x_ffff = none)
    pub index_of_quick_weapon_4: u16,
    // (0/1/2 or -1 disabled)
    pub quick_weapon_slot_1_ability: u16,
    // (0/1/2 or -1 disabled)
    pub quick_weapon_slot_2_ability: u16,
    // (0/1/2 or -1 disabled)
    pub quick_weapon_slot_3_ability: u16,
    // (0/1/2 or -1 disabled)
    pub quick_weapon_slot_4_ability: u16,
    pub quick_spell_1_resource: Resref,
    pub quick_spell_2_resource: Resref,
    pub quick_spell_3_resource: Resref,
    // (0x_ffff = none)
    pub index_of_quick_item_1: u16,
    // (0x_ffff = none)
    pub index_of_quick_item_2: u16,
    // (0x_ffff = none)
    pub index_of_quick_item_3: u16,
    // (0/1/2 or -1 disabled)
    pub quick_item_slot_1_ability: u16,
    // (0/1/2 or -1 disabled)
    pub quick_item_slot_2_ability: u16,
    // (0/1/2 or -1 disabled)
    pub quick_item_slot_3_ability: u16,
    #[br(count = 32)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub name: String,
    pub talk_count: u32,
    #[serde(flatten)]
    pub character_kill_stats: CharacterKillStats,
    // filename prefix for voice set
    #[br(count = 8)]
    pub voice_set: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Stats
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct CharacterKillStats {
    pub most_powerful_vanquished_name: Strref,
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
    pub chapter_kills_xp_gained: u32,
    pub chapter_kills_number: u32,
    pub game_kills_xp_gained: u32,
    pub game_kills_number: u32,
    #[serde(flatten)]
    #[br(count = 4)]
    pub favourite_spells: Vec<Resref>,
    #[serde(flatten)]
    #[br(count = 4)]
    pub favourite_spell_count: Vec<u16>,
    #[serde(flatten)]
    #[br(count = 4)]
    pub favourite_weapons: Vec<Resref>,
    // time equipped in combat - 1/15 seconds
    #[serde(flatten)]
    #[br(count = 4)]
    pub favourite_weapon_time: Vec<u16>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Variable
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct GlobalVariables {
    #[br(count = 32)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub name: String,
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
    pub double_value: u64,
    #[br(count = 32)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub script_name_value: String,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Journal
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct JournalEntries {
    pub journal_text: Strref,
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
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Familiar {
    pub lawful_good_familiar: Resref,
    pub lawful_neutral_familiar: Resref,
    pub lawful_evil_familiar: Resref,
    pub neutral_good_familiar: Resref,
    pub neutral_familiar: Resref,
    pub neutral_evil_familiar: Resref,
    pub chaotic_good_familiar: Resref,
    pub chaotic_neutral_familiar: Resref,
    pub chaotic_evil_familiar: Resref,
    pub offset_to_familiar_resources: u32,
    pub ce_level_1_familiar_spell_count: u32,
    pub ce_level_2_familiar_spell_count: u32,
    pub ce_level_3_familiar_spell_count: u32,
    pub ce_level_4_familiar_spell_count: u32,
    pub ce_level_5_familiar_spell_count: u32,
    pub ce_level_6_familiar_spell_count: u32,
    pub ce_level_7_familiar_spell_count: u32,
    pub ce_level_8_familiar_spell_count: u32,
    pub ce_level_9_familiar_spell_count: u32,
    pub cg_level_1_familiar_spell_count: u32,
    pub cg_level_2_familiar_spell_count: u32,
    pub cg_level_3_familiar_spell_count: u32,
    pub cg_level_4_familiar_spell_count: u32,
    pub cg_level_5_familiar_spell_count: u32,
    pub cg_level_6_familiar_spell_count: u32,
    pub cg_level_7_familiar_spell_count: u32,
    pub cg_level_8_familiar_spell_count: u32,
    pub cg_level_9_familiar_spell_count: u32,
    pub cn_level_1_familiar_spell_count: u32,
    pub cn_level_2_familiar_spell_count: u32,
    pub cn_level_3_familiar_spell_count: u32,
    pub cn_level_4_familiar_spell_count: u32,
    pub cn_level_5_familiar_spell_count: u32,
    pub cn_level_6_familiar_spell_count: u32,
    pub cn_level_7_familiar_spell_count: u32,
    pub cn_level_8_familiar_spell_count: u32,
    pub cn_level_9_familiar_spell_count: u32,
    pub le_level_1_familiar_spell_count: u32,
    pub le_level_2_familiar_spell_count: u32,
    pub le_level_3_familiar_spell_count: u32,
    pub le_level_4_familiar_spell_count: u32,
    pub le_level_5_familiar_spell_count: u32,
    pub le_level_6_familiar_spell_count: u32,
    pub le_level_7_familiar_spell_count: u32,
    pub le_level_8_familiar_spell_count: u32,
    pub le_level_9_familiar_spell_count: u32,
    pub lg_level_1_familiar_spell_count: u32,
    pub lg_level_2_familiar_spell_count: u32,
    pub lg_level_3_familiar_spell_count: u32,
    pub lg_level_4_familiar_spell_count: u32,
    pub lg_level_5_familiar_spell_count: u32,
    pub lg_level_6_familiar_spell_count: u32,
    pub lg_level_7_familiar_spell_count: u32,
    pub lg_level_8_familiar_spell_count: u32,
    pub lg_level_9_familiar_spell_count: u32,
    pub ln_level_1_familiar_spell_count: u32,
    pub ln_level_2_familiar_spell_count: u32,
    pub ln_level_3_familiar_spell_count: u32,
    pub ln_level_4_familiar_spell_count: u32,
    pub ln_level_5_familiar_spell_count: u32,
    pub ln_level_6_familiar_spell_count: u32,
    pub ln_level_7_familiar_spell_count: u32,
    pub ln_level_8_familiar_spell_count: u32,
    pub ln_level_9_familiar_spell_count: u32,
    pub ne_level_1_familiar_spell_count: u32,
    pub ne_level_2_familiar_spell_count: u32,
    pub ne_level_3_familiar_spell_count: u32,
    pub ne_level_4_familiar_spell_count: u32,
    pub ne_level_5_familiar_spell_count: u32,
    pub ne_level_6_familiar_spell_count: u32,
    pub ne_level_7_familiar_spell_count: u32,
    pub ne_level_8_familiar_spell_count: u32,
    pub ne_level_9_familiar_spell_count: u32,
    pub ng_level_1_familiar_spell_count: u32,
    pub ng_level_2_familiar_spell_count: u32,
    pub ng_level_3_familiar_spell_count: u32,
    pub ng_level_4_familiar_spell_count: u32,
    pub ng_level_5_familiar_spell_count: u32,
    pub ng_level_6_familiar_spell_count: u32,
    pub ng_level_7_familiar_spell_count: u32,
    pub ng_level_8_familiar_spell_count: u32,
    pub ng_level_9_familiar_spell_count: u32,
    pub tn_level_1_familiar_spell_count: u32,
    pub tn_level_2_familiar_spell_count: u32,
    pub tn_level_3_familiar_spell_count: u32,
    pub tn_level_4_familiar_spell_count: u32,
    pub tn_level_5_familiar_spell_count: u32,
    pub tn_level_6_familiar_spell_count: u32,
    pub tn_level_7_familiar_spell_count: u32,
    pub tn_level_8_familiar_spell_count: u32,
    pub tn_level_9_familiar_spell_count: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Stored
// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_PocketPlane
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Location {
    pub area: Resref,
    pub x_coordinate: u16,
    pub y_coordinate: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_FamiliarExtra
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct FamiliarExtra {
    pub data: Resref,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
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
        assert_eq!(header.signature, "GAME".to_string());
        assert_eq!(header.version, "V2.0".to_string());
        assert_eq!(header.party_gold, 109741);
        assert_eq!(header.game_time, 1664811);
        assert_eq!(header.count_of_journal_entries, 188);
        assert_eq!(header.game_time_real_seconds, 2774117);
        assert_eq!(header.zoom_level, 58);
        assert_eq!(header.familiar_owner, 0);
    }

    #[test]
    fn valid_party_npc_parsed() {
        let file = File::open("fixtures/BG2EEBALDUR.gam").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let game = Game::new(&buffer);
        let party = game.party_npcs;
        assert_eq!(
            *party.first().unwrap(),
            GameNPC {
                character_selection: 0,
                party_order: 0,
                offset_to_cre_resource: 2292,
                size_of_cre_resource: 19752,
                character_name: "*AV_PALA".to_string(),
                character_orientation: 6,
                characters_current_area: Resref("AR0800\0\0".to_string()),
                character_x_coordinate: 968,
                character_y_coordinate: 318,
                viewing_rectangle_x_coordinate: 366,
                viewing_rectangle_y_coordinate: 81,
                modal_action: 0,
                happiness: 80,
                num_times_interacted: [0; 24],
                index_of_quick_weapon_1: 35,
                index_of_quick_weapon_2: 36,
                index_of_quick_weapon_3: 37,
                index_of_quick_weapon_4: 10,
                quick_weapon_slot_1_ability: 0,
                quick_weapon_slot_2_ability: 0,
                quick_weapon_slot_3_ability: 0,
                quick_weapon_slot_4_ability: 0,
                quick_spell_1_resource: Resref("\0\0\0\0\0\0\0\0".to_string()),
                quick_spell_2_resource: Resref("\0\0\0\0\0\0\0\0".to_string()),
                quick_spell_3_resource: Resref("\0\0\0\0\0\0\0\0".to_string()),
                index_of_quick_item_1: 15,
                index_of_quick_item_2: 16,
                index_of_quick_item_3: 17,
                quick_item_slot_1_ability: 0,
                quick_item_slot_2_ability: 0,
                quick_item_slot_3_ability: 0,
                name: "Nimi Iluvia\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".to_string(),
                talk_count: 0,
                character_kill_stats: CharacterKillStats {
                    most_powerful_vanquished_name: Strref(14430),
                    most_powerful_vanquished_xp_reward: 25000,
                    time_in_party: 0,
                    time_joined: 31499,
                    party_member: 1,
                    unused: 0,
                    first_letter_of_cre_resref: 42,
                    chapter_kills_xp_gained: 109150,
                    chapter_kills_number: 27,
                    game_kills_xp_gained: 2111235,
                    game_kills_number: 701,
                    favourite_spells: vec![
                        Resref("SPCL211\0".to_string()),
                        Resref("SPPR111\0".to_string()),
                        Resref("SPIN101\0".to_string()),
                        Resref("SPIN103\0".to_string())
                    ],
                    favourite_spell_count: vec![3, 660, 294, 76],
                    favourite_weapons: vec![
                        Resref("SW1H62\0\0".to_string()),
                        Resref("SW1H24\0\0".to_string()),
                        Resref("SW1H25\0\0".to_string()),
                        Resref("SW1H51\0\0".to_string())
                    ],
                    favourite_weapon_time: vec![200, 14644, 13948, 49692]
                },
                voice_set: vec![0, 0, 0, 0, 0, 0, 0, 0]
            }
        );
        assert_eq!(
            *party.last().unwrap(),
            GameNPC {
                character_selection: 0,
                party_order: 4,
                offset_to_cre_resource: 71292,
                size_of_cre_resource: 17084,
                character_name: "*ERIE6\0\0".to_string(),
                character_orientation: 6,
                characters_current_area: Resref("AR0800\0\0".to_string()),
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
                quick_spell_1_resource: Resref("SPWI302\0".to_string()),
                quick_spell_2_resource: Resref("SPWI427\0".to_string()),
                quick_spell_3_resource: Resref("\0\0\0\0\0\0\0\0".to_string()),
                index_of_quick_item_1: 15,
                index_of_quick_item_2: 65535,
                index_of_quick_item_3: 65535,
                quick_item_slot_1_ability: 0,
                quick_item_slot_2_ability: 65535,
                quick_item_slot_3_ability: 65535,
                name: "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".into(),
                talk_count: 6,
                character_kill_stats: CharacterKillStats {
                    most_powerful_vanquished_name: Strref(34947),
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
                    favourite_spells: vec![
                        Resref("SPWI112\0".to_string()),
                        Resref("SPWI617\0".to_string()),
                        Resref("SPPR208\0".to_string()),
                        Resref("SPWI408\0".to_string())
                    ],
                    favourite_spell_count: vec![8, 1, 57, 33],
                    favourite_weapons: vec![
                        Resref("BULL01\0\0".to_string()),
                        Resref("WAFLAIL\0".to_string()),
                        Resref("FIST\0\0\0\0".to_string()),
                        Resref("WASLING\0".to_string())
                    ],
                    favourite_weapon_time: vec![1910, 126, 1716, 11844]
                },
                voice_set: vec![0, 0, 0, 0, 0, 0, 0, 0],
            }
        );
    }

    #[test]
    fn valid_npc_parsed() {
        let file = File::open("fixtures/BG2EEBALDUR.gam").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let game = Game::new(&buffer);
        let non_party = game.non_party_npcs;
        assert_ne!(non_party.first(), None);
        assert_eq!(
            *non_party.last().unwrap(),
            GameNPC {
                character_selection: 0,
                party_order: 65535,
                offset_to_cre_resource: 196144,
                size_of_cre_resource: 1868,
                character_name: "*AZZY8\0\0".into(),
                character_orientation: 14,
                characters_current_area: Resref("AR2002\0\0".to_string()),
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
                quick_spell_1_resource: Resref("\0\0\0\0\0\0\0\0".to_string()),
                quick_spell_2_resource: Resref("\0\0\0\0\0\0\0\0".to_string()),
                quick_spell_3_resource: Resref("\0\0\0\0\0\0\0\0".to_string()),
                index_of_quick_item_1: 65535,
                index_of_quick_item_2: 65535,
                index_of_quick_item_3: 65535,
                quick_item_slot_1_ability: 65535,
                quick_item_slot_2_ability: 65535,
                quick_item_slot_3_ability: 65535,
                name: "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
                    .to_string(),
                talk_count: 1,
                character_kill_stats: CharacterKillStats {
                    most_powerful_vanquished_name: Strref(4294967295),
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
                    favourite_spells: vec![
                        Resref("\0\0\0\0\0\0\0\0".to_string()),
                        Resref("\0\0\0\0\0\0\0\0".to_string()),
                        Resref("\0\0\0\0\0\0\0\0".to_string()),
                        Resref("\0\0\0\0\0\0\0\0".to_string())
                    ],
                    favourite_spell_count: vec![0, 0, 0, 0],
                    favourite_weapons: vec![
                        Resref("\0\0\0\0\0\0\0\0".to_string()),
                        Resref("\0\0\0\0\0\0\0\0".to_string()),
                        Resref("\0\0\0\0\0\0\0\0".to_string()),
                        Resref("\0\0\0\0\0\0\0\0".to_string())
                    ],
                    favourite_weapon_time: vec![0, 0, 0, 0]
                },
                voice_set: vec![0, 0, 0, 0, 0, 0, 0, 0],
            }
        )
    }

    #[test]
    fn valid_familar_parsed() {
        let file = File::open("fixtures/BG2EEBALDUR.gam").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let familiar = Game::new(&buffer).familiar.unwrap();
        assert_eq!(
            familiar.lawful_good_familiar,
            Resref("FAMPSD\0\0".to_string())
        );
        assert_eq!(
            familiar.lawful_neutral_familiar,
            Resref("FAMFER\0\0".to_string())
        );
        assert_eq!(
            familiar.lawful_evil_familiar,
            Resref("FAMIMP\0\0".to_string())
        );
        assert_eq!(
            familiar.neutral_good_familiar,
            Resref("FAMPSD\0\0".to_string())
        );
        assert_eq!(familiar.neutral_familiar, Resref("FAMRAB\0\0".to_string()));
        assert_eq!(
            familiar.neutral_evil_familiar,
            Resref("FAMDUST\0".to_string())
        );
        assert_eq!(
            familiar.chaotic_good_familiar,
            Resref("FAMFAIR\0".to_string())
        );
        assert_eq!(
            familiar.chaotic_neutral_familiar,
            Resref("FAMCAT\0\0".to_string())
        );
        assert_eq!(
            familiar.chaotic_evil_familiar,
            Resref("FAMQUAS\0".to_string())
        );
        assert_eq!(familiar.offset_to_familiar_resources, 318688);
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
                journal_text: Strref(34089),
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
                journal_text: Strref(97343),
                time: 24711890,
                current_chapter_number: 6,
                read_by_character: 255,
                journal_section: 1,
                location_flag: 255
            })
        );
    }
}
