use binrw::{
    io::{Cursor, SeekFrom},
    BinRead, BinReaderExt, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::common::{resref::Resref, strref::Strref};
use crate::effect_v1::EffectV1;
use crate::item_table::ItemReferenceTable;
use crate::{
    item_table::ItemSlots,
    model::Model,
    spell_table::{KnownSpells, SpellMemorizationInfo, SpellMemorizationTable},
};

// TODO: Fix writing this as a binary file
// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Creature {
    #[serde(flatten)]
    pub header: BGEECreatureHeader,
    #[br(count=header.count_of_known_spells, seek_before=SeekFrom::Start(header.offset_to_known_spells as u64))]
    pub known_spells: Vec<KnownSpells>,
    #[br(count=header.count_of_spell_memorization_info, seek_before=SeekFrom::Start(header.offset_to_spell_memorization_info as u64))]
    pub memorized_spell_info: Vec<SpellMemorizationInfo>,
    #[br(count=header.count_of_memorized_spell_table, seek_before=SeekFrom::Start(header.offset_to_memorized_spell_table as u64))]
    pub memorized_spells: Vec<SpellMemorizationTable>,
    #[br(count=header.count_of_effects, seek_before=SeekFrom::Start(header.offset_to_effects as u64))]
    pub effects: Vec<EffectV1>,
    #[br(count=header.count_of_items, seek_before=SeekFrom::Start(header.offset_to_items as u64))]
    pub item_table: Vec<ItemReferenceTable>,
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
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub signature: String,
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub version: String,
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
    #[br(count = 32)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub tracking: String,
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
    #[br(count = 32)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub death_variable: String,
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
    use binrw::io::{BufReader, Read};
    use pretty_assertions::assert_eq;
    use std::fs::File;

    #[test]
    fn valid_simple_creature_file_header_parsed() {
        let file = File::open("fixtures/dbeggar.cre").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let mut reader = Cursor::new(&buffer);
        let header: BGEECreatureHeader = reader.read_le().unwrap();
        assert_eq!(
            header,
            BGEECreatureHeader {
                signature: "CRE ".into(),
                version: "V1.0".into(),
                long_creature_name: 15855,
                short_creature_name: 15856,
                flags: 0,
                exp_for_killing: 15,
                exp: 0,
                gold: 0,
                state_flags: 0,
                current_hp: 8,
                base_hp: 8,
                animation_id: 51456,
                metal_color: 30,
                minor_color: 209,
                major_color: 208,
                skin_color: 201,
                leather_color: 228,
                armor_color: 28,
                hair_color: 200,
                effstructure: 1,
                small_portrait: Resref("S9BEGG4\0".into(),),
                large_portrait: Resref("None\0\0\0\0".into(),),
                reputation: 0,
                hide_in_shadows: 0,
                nac_1: 10,
                nac_2: 10,
                nac_mod_crushing: 0,
                nac_mod_missile: 0,
                nac_mod_piercing: 0,
                nac_mod_slashing: 0,
                thac0: 19,
                attacks: 1,
                save_death: 14,
                save_wands: 16,
                save_poly: 15,
                save_breath: 17,
                save_spells: 17,
                resist_fire: 0,
                resist_cold: 0,
                resist_electricity: 0,
                resist_acid: 0,
                resist_magic: 0,
                resist_magicfire: 0,
                resist_magiccold: 0,
                resist_slashing: 0,
                resist_crushing: 0,
                resist_piercing: 0,
                resist_missile: 0,
                detect_illusions: 0,
                set_traps: 0,
                lore: 0,
                open_locks: 0,
                move_silently: 0,
                find_traps: 0,
                pick_pockets: 0,
                fatigue: 0,
                intoxication: 0,
                luck: 0,
                proficiency_large_swords: 0,
                proficiency_small_swords: 0,
                proficiency_bows: 0,
                proficiency_spears: 0,
                proficiency_blunt: 0,
                proficiency_spiked: 0,
                proficiency_axes: 0,
                proficiency_missiles: 0,
                unused_proficiencies: vec![0, 0, 0, 0, 0, 0, 0],
                nightmare_mode: 0,
                translucency: 0,
                reputation_loss_if_killed: 0,
                reputation_loss_if_joins_party: 0,
                reputation_loss_if_leaves_party: 0,
                turn_undead_level: 0,
                tracking_skill: 0,
                tracking: "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".into(),
                strrefs: vec![
                    Strref(61815,),
                    Strref(123267,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(61823,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(61821,),
                    Strref(61822,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(61815,),
                    Strref(61816,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                    Strref(4294967295,),
                ],
                level_first_class: 1,
                level_second_class: 1,
                level_third_class: 1,
                sex: 1,
                strength: 9,
                strength_bonus: 0,
                intelligence: 9,
                wisdom: 9,
                dexterity: 9,
                constitution: 9,
                charisma: 9,
                morale: 10,
                morale_break: 7,
                racial_enemy: 255,
                morale_recovery_time: 60,
                kit: 0,
                override_script: Resref("shoutdl3".into(),),
                class_script: Resref("None\0\0\0\0".into(),),
                race_script: Resref("None\0\0\0\0".into(),),
                general_script: Resref("None\0\0\0\0".into(),),
                creature_script_default: Resref("WTRUNSGT".into(),),
                enemy_ally: 128,
                general: 1,
                race: 1,
                class: 155,
                specific: 0,
                gender: 1,
                object_references: vec![0, 0, 0, 0, 0],
                alignment: 34,
                global_actor_enumeration: 65535,
                local_actor_enumeration: 65535,
                death_variable: "None\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
                    .into(),
                offset_to_known_spells: 724,
                count_of_known_spells: 0,
                offset_to_spell_memorization_info: 724,
                count_of_spell_memorization_info: 17,
                offset_to_memorized_spell_table: 996,
                count_of_memorized_spell_table: 0,
                offset_to_item_slots: 996,
                offset_to_items: 996,
                count_of_items: 0,
                offset_to_effects: 996,
                count_of_effects: 0,
                dialog_ref: Resref("dbeggar\0".into(),),
            }
        )
    }

    #[test]
    fn valid_creature_file_header_parsed() {
        let file = File::open("fixtures/cutmelis.cre").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let mut reader = Cursor::new(&buffer);
        let creature: Creature = reader.read_le().unwrap();
        assert_eq!(creature.header.current_hp, 87);
        assert_eq!(creature.header.level_first_class, 30);
        assert_eq!(creature.header.level_second_class, 30);
        assert_eq!(creature.header.level_third_class, 1);
        assert_eq!(creature.header.sex, 2);
        assert_eq!(creature.header.strength, 13);
        assert_eq!(creature.header.strength_bonus, 0);
        assert_eq!(creature.header.intelligence, 18);
        assert_eq!(creature.header.wisdom, 17);
        assert_eq!(creature.header.dexterity, 16);
        assert_eq!(creature.header.constitution, 12);
        assert_eq!(creature.header.charisma, 15);
        assert_eq!(creature.header.morale, 10);
        assert_eq!(creature.header.offset_to_item_slots, 2628);
        assert_eq!(creature.header.offset_to_memorized_spell_table, 2100);
        assert_eq!(creature.header.offset_to_spell_memorization_info, 1828);
        assert_eq!(creature.known_spells.len(), 92);
        assert_eq!(creature.memorized_spell_info.len(), 17);
        assert_eq!(
            creature.known_spells.last(),
            Some(&KnownSpells {
                spell_name: Resref("SPWI113\0".into(),),
                spell_level: 0,
                spell_type: 1,
            },)
        );
        assert_eq!(
            creature.memorized_spell_info.first(),
            Some(&SpellMemorizationInfo {
                spell_level: 0,
                number_of_spells_memorizable: 4,
                number_of_spells_memorizable_after_effects: 4,
                spell_type: 0,
                index_to_spell_table: 0,
                count_of_memorizable_spell_tables: 4,
            })
        );
        assert_eq!(
            creature.memorized_spells,
            vec![
                SpellMemorizationTable {
                    spell_name: Resref("SPPR103\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR103\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR109\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR101\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR203\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR208\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR211\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR212\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR312\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR313\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR315\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR401\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR413\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR411\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR502\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR503\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI113\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI112\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI110\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI105\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI213\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI220\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI211\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI203\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI312\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI311\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI318\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI308\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI408\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI405\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI406\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI510\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI505\0".into()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI522\0".into()),
                    memorised: 1,
                },
            ]
        );
    }
}
