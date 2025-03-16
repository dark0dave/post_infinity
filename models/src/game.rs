use binrw::{
    helpers::until_eof,
    io::{Cursor, Read, Seek, SeekFrom},
    BinRead, BinReaderExt, BinResult, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::common::{char_array::CharArray, header::Header};
use crate::model::Model;
use crate::{
    common::{strref::Strref, Resref},
    creature::Creature,
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Game {
    #[serde(skip)]
    #[br(parse_with = until_eof, restore_position)]
    pub original_bytes: Vec<u8>,
    #[bw(ignore)]
    #[serde(flatten)]
    pub header: BGEEGameHeader,
    #[bw(ignore)]
    #[br(count=header.count_of_npc_structs_for_party_members)]
    pub party_npcs: Vec<GameNPC>,
    #[bw(ignore)]
    #[br(parse_with = |reader, _, _:()| parse_creatures(reader, &party_npcs))]
    pub party_npcs_cres: Vec<Creature>,
    #[bw(ignore)]
    #[br(count=header.count_of_npc_structs_for_npcs)]
    pub non_party_npcs: Vec<GameNPC>,
    #[bw(ignore)]
    #[br(parse_with = |reader, _, _:()| parse_creatures(reader, &non_party_npcs))]
    pub non_party_npcs_cres: Vec<Creature>,
    #[bw(ignore)]
    #[br(count=header.count_of_global_namespace_variables)]
    pub global_variables: Vec<GlobalVariables>,
    #[bw(ignore)]
    #[br(if(header.offset_to_journal_entries != u32::MAX), count=header.count_of_journal_entries, seek_before=SeekFrom::Start(header.offset_to_journal_entries as u64))]
    pub journal_entries: Vec<JournalEntries>,
    #[bw(ignore)]
    #[br(seek_before=SeekFrom::Start(header.offset_to_familiar as u64))]
    pub familiar: Option<Familiar>,
    #[bw(ignore)]
    #[br(count=header.count_of_stored_locations, seek_before=SeekFrom::Start(header.offset_to_stored_locations as u64))]
    pub stored_locations: Vec<Location>,
    #[bw(ignore)]
    #[br(count=header.count_of_pocket_plane_locations, seek_before=SeekFrom::Start(header.offset_to_pocket_plane_locations as u64))]
    pub pocket_plane_locations: Vec<Location>,
    #[bw(ignore)]
    #[br(if(header.offset_to_familiar_extra != u32::MAX), parse_with=binrw::helpers::until_eof, seek_before=SeekFrom::Start(header.offset_to_familiar_extra as u64))]
    pub familiar_extra: Vec<FamiliarExtra>,
}

fn parse_creatures<R: Read + Seek>(
    reader: &mut R,
    npcs: &Vec<GameNPC>,
) -> BinResult<Vec<Creature>> {
    let mut buff = vec![];

    let mut creatures = Vec::with_capacity(npcs.len());
    for npc in npcs {
        let end = npc.size_of_cre_resource as u64;
        if end == 0 {
            continue;
        }
        let mut handler = reader.take(end);
        handler.read_to_end(&mut buff).unwrap();
        creatures.push(Creature::new(&buff));
    }
    Ok(creatures)
}

impl Model for Game {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
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
    #[serde(flatten)]
    pub header: Header,
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
    pub current_campaign: CharArray<8>,
    pub familiar_owner: u32,
    pub random_encounter_script: CharArray<20>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_NPC
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct GameNPC {
    pub character_selection: u16,
    // x0-0x5 = player_x_fill, 0x_ffff = not in party
    pub party_order: u16,
    pub offset_to_cre_resource: u32,
    pub size_of_cre_resource: u32,
    pub character_name: CharArray<8>,
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
    pub name: CharArray<32>,
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
    #[br(count = 4)]
    pub favourite_spells: Vec<Resref>,
    #[br(count = 4)]
    pub favourite_spell_count: Vec<u16>,
    #[br(count = 4)]
    pub favourite_weapons: Vec<Resref>,
    // time equipped in combat - 1/15 seconds
    #[br(count = 4)]
    pub favourite_weapon_time: Vec<u16>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm#GAMEV2_0_Variable
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct GlobalVariables {
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
    pub double_value: u64,
    pub script_name_value: CharArray<32>,
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
    use binrw::io::Read;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::{error::Error, fs::File};

    const GAME_FIXTURE: &str = "fixtures/bg2eebaldur.gam";
    const GAME_JSON_FIXTURE: &str = "fixtures/bg2eebaldur.gam.json";

    fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse_party_npcs() -> Result<(), Box<dyn Error>> {
        let game: Game = Game::new(&read_file(GAME_FIXTURE)?);
        let expected: Value = serde_json::from_slice(&read_file(GAME_JSON_FIXTURE)?)?;

        let party_npcs: Vec<GameNPC> = serde_json::from_value(
            (expected
                .get("party_npcs")
                .ok_or("Failed to get party npcs")?)
            .clone(),
        )?;
        assert_eq!(game.party_npcs, party_npcs);
        Ok(())
    }

    #[test]
    fn parse_non_party_npcs() -> Result<(), Box<dyn Error>> {
        let game: Game = Game::new(&read_file(GAME_FIXTURE)?);
        let expected: Value = serde_json::from_slice(&read_file(GAME_JSON_FIXTURE)?)?;

        let party_npcs: Vec<GameNPC> = serde_json::from_value(
            (expected
                .get("non_party_npcs")
                .ok_or("Failed to get non party npcs")?)
            .clone(),
        )?;
        assert_eq!(game.non_party_npcs, party_npcs);
        Ok(())
    }
}
