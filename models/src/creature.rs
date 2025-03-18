use binrw::{
    helpers::until_eof,
    io::{Cursor, SeekFrom},
    BinRead, BinReaderExt, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::common::{header::Header, strref::Strref, Resref};
use crate::effect_v1::EffectV1;
use crate::item_table::ItemReferenceTable;
use crate::{common::char_array::CharArray, effect_v2::EffectV2Body};
use crate::{
    item_table::ItemSlots,
    model::Model,
    spell_table::{KnownSpells, SpellMemorizationInfo, SpellMemorizationTable},
};

// TODO: Fix writing this as a binary file
// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Creature {
    #[serde(skip)]
    #[br(parse_with = until_eof, restore_position)]
    pub original_bytes: Vec<u8>,
    #[serde(flatten)]
    pub header: BGEECreatureHeader,
    #[bw(ignore)]
    #[br(count=header.count_of_known_spells, seek_before=SeekFrom::Start(header.offset_to_known_spells as u64))]
    pub known_spells: Vec<KnownSpells>,
    #[bw(ignore)]
    #[br(count=header.count_of_spell_memorization_info, seek_before=SeekFrom::Start(header.offset_to_spell_memorization_info as u64))]
    pub memorized_spell_info: Vec<SpellMemorizationInfo>,
    #[bw(ignore)]
    #[br(count=header.count_of_memorized_spell_table, seek_before=SeekFrom::Start(header.offset_to_memorized_spell_table as u64))]
    pub memorized_spells: Vec<SpellMemorizationTable>,
    #[bw(ignore)]
    #[br(if(header.effstructure == 0), count=header.count_of_effects, seek_before=SeekFrom::Start(header.offset_to_effects as u64))]
    pub effects_v1: Vec<EffectV1>,
    #[bw(ignore)]
    #[br(if(header.effstructure == 1), count=header.count_of_effects, seek_before=SeekFrom::Start(header.offset_to_effects as u64))]
    pub effects_v2: Vec<EffectV2Body>,
    #[bw(ignore)]
    #[br(count=header.count_of_items, seek_before=SeekFrom::Start(header.offset_to_items as u64))]
    pub item_table: Vec<ItemReferenceTable>,
    #[bw(ignore)]
    #[br(seek_before=SeekFrom::Start(header.offset_to_item_slots as u64))]
    pub item_slots: Option<ItemSlots>,
}

impl Model for Creature {
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

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_Header
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BGEECreatureHeader {
    #[serde(flatten)]
    pub header: Header,
    pub long_creature_name: u32,
    pub short_creature_name: u32,
    // see CRE_FLAG_* above for possible flags
    pub flags: u32,
    pub exp_for_killing: u32,
    pub exp: u32,
    pub gold: u32,
    pub state_flags: u32,
    pub current_hp: u16,
    pub base_hp: u16,
    pub animation_id: u32,
    pub metal_color: u8,
    pub minor_color: u8,
    pub major_color: u8,
    pub skin_color: u8,
    pub leather_color: u8,
    pub armor_color: u8,
    pub hair_color: u8,
    // 0 = v1, 1 = v2
    pub effstructure: u8,
    pub small_portrait: Resref,
    pub large_portrait: Resref,
    pub reputation: u8,
    pub hide_in_shadows: u8,
    pub nac_1: u16,
    pub nac_2: u16,
    pub nac_mod_crushing: u16,
    pub nac_mod_missile: u16,
    pub nac_mod_piercing: u16,
    pub nac_mod_slashing: u16,
    pub thac0: u8,
    pub attacks: u8,
    pub save_death: u8,
    pub save_wands: u8,
    pub save_poly: u8,
    pub save_breath: u8,
    pub save_spells: u8,
    pub resist_fire: u8,
    pub resist_cold: u8,
    pub resist_electricity: u8,
    pub resist_acid: u8,
    pub resist_magic: u8,
    pub resist_magicfire: u8,
    pub resist_magiccold: u8,
    pub resist_slashing: u8,
    pub resist_crushing: u8,
    pub resist_piercing: u8,
    pub resist_missile: u8,
    pub detect_illusions: u8,
    pub set_traps: u8,
    pub lore: u8,
    pub open_locks: u8,
    pub move_silently: u8,
    pub find_traps: u8,
    pub pick_pockets: u8,
    pub fatigue: u8,
    pub intoxication: u8,
    pub luck: u8,
    pub proficiency_large_swords: u8,
    pub proficiency_small_swords: u8,
    pub proficiency_bows: u8,
    pub proficiency_spears: u8,
    pub proficiency_blunt: u8,
    pub proficiency_spiked: u8,
    pub proficiency_axes: u8,
    pub proficiency_missiles: u8,
    #[br(count = 7)]
    pub unused_proficiencies: Vec<u8>,
    pub nightmare_mode: u8,
    pub translucency: u8,
    pub reputation_loss_if_killed: u8,
    pub reputation_loss_if_joins_party: u8,
    pub reputation_loss_if_leaves_party: u8,
    pub turn_undead_level: u8,
    pub tracking_skill: u8,
    // The following entry applies to BG1, BG2 and BGEE
    pub tracking: CharArray<32>,
    // Strrefs pertaining to the character.
    // Most are connected with the sound-set (see SOUNDOFF.IDS (BG1) and SNDSLOT.IDS for (BG2)).
    // This is broken, it should be 100 u32s
    #[br(count = 100)]
    pub strrefs: Vec<Strref>,
    pub level_first_class: u8,
    pub level_second_class: u8,
    pub level_third_class: u8,
    // from gender.ids via sex stat
    pub sex: u8,
    pub strength: u8,
    pub strength_bonus: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub charisma: u8,
    pub morale: u8,
    pub morale_break: u8,
    pub racial_enemy: u8,
    pub morale_recovery_time: u16,
    pub kit: u32,
    pub override_script: Resref,
    pub class_script: Resref,
    pub race_script: Resref,
    pub general_script: Resref,
    pub creature_script_default: Resref,
    pub enemy_ally: u8,
    pub general: u8,
    pub race: u8,
    pub class: u8,
    pub specific: u8,
    pub gender: u8,
    // object.ids references
    #[br(count = 5)]
    pub object_references: Vec<u8>,
    pub alignment: u8,
    pub global_actor_enumeration: u16,
    pub local_actor_enumeration: u16,
    // death variable: sprite_is_dead on death
    pub death_variable: CharArray<32>,
    pub offset_to_known_spells: u32,
    pub count_of_known_spells: u32,
    pub offset_to_spell_memorization_info: u32,
    pub count_of_spell_memorization_info: u32,
    pub offset_to_memorized_spell_table: u32,
    pub count_of_memorized_spell_table: u32,
    pub offset_to_item_slots: u32,
    pub offset_to_items: u32,
    pub count_of_items: u32,
    pub offset_to_effects: u32,
    pub count_of_effects: u32,
    pub dialog_ref: Resref,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::io::Read;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::{error::Error, fs::File};

    const FIXTURES: [(&str, &str); 2] = [
        ("fixtures/dbeggar.cre", "fixtures/dbeggar.cre.json"),
        ("fixtures/cutmelis.cre", "fixtures/cutmelis.cre.json"),
    ];

    fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let creature: Creature = Creature::new(&read_file(file_path)?);
            let result: Value = serde_json::to_value(creature)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
