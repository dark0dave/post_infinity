use std::rc::Rc;

use serde::Serialize;

use crate::common::fixed_char_nd_array::FixedCharNDArray;
use crate::common::header::Header;
use crate::resources::utils::{copy_buff_to_struct, copy_transmute_buff};
use crate::tlk::Lookup;
use crate::{
    common::fixed_char_array::FixedCharSlice,
    effect_v2::EffectV2Body,
    item_table::ItemTable,
    model::Model,
    spell_table::{generate_spell_memorization, KnownSpells, MemorizedSpells},
};

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Creature {
    pub header: BGEECreature,
    pub item_slot_table: ItemTable,
    pub known_spells: Vec<KnownSpells>,
    pub memorized_spells: Vec<MemorizedSpells>,
    pub effects: Vec<EffectV2Body>,
}

impl Model for Creature {
    fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<BGEECreature>(buffer, 0);

        let item_slot_table = ItemTable::generate(
            buffer,
            usize::try_from(header.offset_to_items).unwrap_or(0),
            usize::try_from(header.count_of_items).unwrap_or(0),
            usize::try_from(header.offset_to_item_slots).unwrap_or(0),
        );

        let start = usize::try_from(header.offset_to_known_spells).unwrap_or(0);
        let count = usize::try_from(header.count_of_known_spells).unwrap_or(0);
        let known_spells = copy_transmute_buff::<KnownSpells>(buffer, start, count);

        let start = usize::try_from(header.offset_to_spell_memorization_info).unwrap_or(0);
        let count = usize::try_from(header.count_of_spell_memorization_info).unwrap_or(0);
        let memorized_spells = generate_spell_memorization(
            buffer,
            start,
            count,
            usize::try_from(header.offset_to_memorized_spell_table).unwrap_or(0),
        );

        let start = usize::try_from(header.offset_to_effects).unwrap_or(0);
        let count = usize::try_from(header.count_of_effects).unwrap_or(0);
        let effects = copy_transmute_buff::<EffectV2Body>(buffer, start, count);

        Creature {
            header,
            item_slot_table,
            known_spells,
            memorized_spells,
            effects,
        }
    }
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, lookup: &Lookup) -> String {
        self.header.dialog_ref.to_string()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct BGEECreature {
    pub header: Header<4, 4>,
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

    pub small_portrait: FixedCharSlice<8>,
    pub large_portrait: FixedCharSlice<8>,
    pub reputation: u8,
    pub hide_in_shadows: u8,
    pub nac_1: i16,
    pub nac_2: i16,
    pub nac_mod_crushing: i16,
    pub nac_mod_missile: i16,
    pub nac_mod_piercing: i16,
    pub nac_mod_slashing: i16,
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
    pub fatique: u8,
    pub intoxication: u8,
    pub luck: u8,

    pub proficiency_largeswords: u8,
    pub proficiency_smallswords: u8,
    pub proficiency_bows: u8,
    pub proficiency_spears: u8,
    pub proficiency_blunt: u8,
    pub proficiency_spiked: u8,
    pub proficiency_axes: u8,
    pub proficiency_missiles: u8,
    pub unused_proficencies: FixedCharSlice<7>,

    pub nightmare_mode: u8,
    pub translucency: u8,
    pub reputation_loss_if_killed: u8,
    pub reputation_loss_if_joins_party: u8,
    pub reputation_loss_if_leaves_party: u8,
    pub turn_undead_level: u8,
    pub tracking_skill: u8,
    // The following entry applies to BG1, BG2 and BGEE
    pub tracking: FixedCharSlice<32>,
    // Strrefs pertaining to the character.
    // Most are connected with the sound-set (see SOUNDOFF.IDS (BG1) and SNDSLOT.IDS for (BG2)).
    // This is broken, it should be 100 u32s
    pub strrefs: FixedCharNDArray<4, 100>,

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
    pub override_script: FixedCharSlice<8>,
    pub class_script: FixedCharSlice<8>,
    pub race_script: FixedCharSlice<8>,
    pub general_script: FixedCharSlice<8>,
    pub default_script: FixedCharSlice<8>,

    pub enemy_ally: u8,
    pub general: u8,
    pub race: u8,
    pub class: u8,
    pub specific: u8,
    pub gender: u8,

    // object.ids references
    pub object_references: FixedCharSlice<5>,

    pub alignment: u8,

    pub global_actor_enumeration: u16,
    pub local_actor_enumeration: u16,

    // death variable: sprite_is_dead on death
    pub death_variable: FixedCharSlice<32>,
    pub offset_to_known_spells: i32,
    pub count_of_known_spells: i32,
    pub offset_to_spell_memorization_info: i32,
    pub count_of_spell_memorization_info: i32,
    pub offset_to_memorized_spell_table: i32,
    pub count_of_memorized_spell_table: i32,
    pub offset_to_item_slots: i32,
    pub offset_to_items: i32,
    pub count_of_items: i32,
    pub offset_to_effects: i32,
    pub count_of_effects: i32,

    pub dialog_ref: FixedCharSlice<8>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_simple_creature_file_header_parsed() {
        let file = File::open("fixtures/dbeggar.cre").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let creature = copy_buff_to_struct::<BGEECreature>(&buffer, 0);
        assert_eq!({ creature.base_hp }, 8);
        assert_eq!({ creature.level_first_class }, 1);
        assert_eq!({ creature.level_second_class }, 1);
        assert_eq!({ creature.level_third_class }, 1);
        assert_eq!({ creature.sex }, 1);
        assert_eq!({ creature.strength }, 9);
        assert_eq!({ creature.strength_bonus }, 0);
        assert_eq!({ creature.intelligence }, 9);
        assert_eq!({ creature.wisdom }, 9);
        assert_eq!({ creature.dexterity }, 9);
        assert_eq!({ creature.constitution }, 9);
        assert_eq!({ creature.charisma }, 9);
        assert_eq!({ creature.morale }, 10);
        assert_eq!({ creature.offset_to_item_slots }, 996);
        assert_eq!({ creature.offset_to_memorized_spell_table }, 996);
        assert_eq!(creature.dialog_ref, "dbeggar\0".into());
    }

    #[test]
    fn valid_creature_file_header_parsed() {
        let file = File::open("fixtures/cutmelis.cre").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let creature = copy_buff_to_struct::<BGEECreature>(&buffer, 0);
        assert_eq!({ creature.base_hp }, 87);
        assert_eq!({ creature.level_first_class }, 30);
        assert_eq!({ creature.level_second_class }, 30);
        assert_eq!({ creature.level_third_class }, 1);
        assert_eq!({ creature.sex }, 2);
        assert_eq!({ creature.strength }, 13);
        assert_eq!({ creature.strength_bonus }, 0);
        assert_eq!({ creature.intelligence }, 18);
        assert_eq!({ creature.wisdom }, 17);
        assert_eq!({ creature.dexterity }, 16);
        assert_eq!({ creature.constitution }, 12);
        assert_eq!({ creature.charisma }, 15);
        assert_eq!({ creature.morale }, 10);
        assert_eq!({ creature.offset_to_item_slots }, 2628);
        assert_eq!({ creature.offset_to_memorized_spell_table }, 2100);
        assert_eq!({ creature.offset_to_spell_memorization_info }, 1828);
        assert_eq!(creature.dialog_ref, "None\0\0\0\0".into());
    }
}
