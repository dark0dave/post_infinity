use std::rc::Rc;

use serde::Serialize;

use crate::common::feature_block::FeatureBlock;
use crate::common::fixed_char_array::FixedCharSlice;
use crate::common::header::Header;
use crate::common::signed_fixed_char_array::SignedFixedCharSlice;
use crate::model::Model;
use crate::resources::utils::{copy_buff_to_struct, copy_transmute_buff};
use crate::tlk::Lookup;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm
#[derive(Debug, Serialize)]
pub struct Spell {
    pub header: SpellHeader,
    pub extended_headers: Vec<SpellExtendedHeader>,
    pub equiping_feature_blocks: Vec<SpellFeatureBlock>,
}

impl Model for Spell {
    fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<SpellHeader>(buffer, 0);

        let start = usize::try_from(header.offset_to_extended_headers).unwrap_or(0);
        let count = usize::try_from(header.count_of_extended_headers).unwrap_or(0);
        let extended_headers = copy_transmute_buff::<SpellExtendedHeader>(buffer, start, count);

        let start = usize::try_from(header.offset_to_casting_feature_blocks).unwrap_or(0);
        let count = usize::try_from(header.count_of_casting_feature_blocks).unwrap_or(0);
        let equiping_feature_blocks =
            copy_transmute_buff::<SpellFeatureBlock>(buffer, start, count);

        Self {
            header,
            extended_headers,
            equiping_feature_blocks,
        }
    }
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, lookup: &Lookup) -> String {
        let name = if self.header.identified_spell_name > -1 {
            lookup
                .data_entries
                .get(self.header.identified_spell_name as usize)
                .unwrap()
                .strings
                .to_string()
        } else if self.header.unidentified_spell_name > -1 {
            lookup
                .data_entries
                .get(self.header.unidentified_spell_name as usize)
                .unwrap()
                .strings
                .to_string()
        } else {
            format!("{}", { self.header.identified_spell_name })
        };
        format!("{}.spl", name)
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm#splv1_Header
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct SpellHeader {
    header: Header<4, 4>,
    unidentified_spell_name: i32,
    identified_spell_name: i32,
    completion_sound: FixedCharSlice<8>,
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v2.htm#Header_Flags
    flags: u32,
    spell_type: u16,
    exclusion_flags: u32,
    casting_graphics: FixedCharSlice<2>,
    min_level: u8,
    primary_spell_school: u8,
    min_strength: u8,
    secondary_spell_school: u8,
    min_strenth_bonus: u8,
    kit_usability_1: u8,
    min_intelligence: u8,
    kit_usability_2: u8,
    min_dexterity: u8,
    kit_usability_3: u8,
    min_wisdom: u8,
    kit_usability_4: u8,
    min_constitution: u16,
    min_charisma: u16,
    spell_level: u32,
    max_stackable: u16,
    spellbook_icon: FixedCharSlice<8>,
    lore: u16,
    ground_icon: FixedCharSlice<8>,
    base_weight: u32,
    spell_description_generic: SignedFixedCharSlice<4>,
    spell_description_identified: SignedFixedCharSlice<4>,
    description_icon: SignedFixedCharSlice<8>,
    enchantment: u32,
    offset_to_extended_headers: i32,
    count_of_extended_headers: i16,
    offset_to_feature_block_table: i32,
    offset_to_casting_feature_blocks: i16,
    count_of_casting_feature_blocks: i16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm#splv1_Extended_Header
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct SpellExtendedHeader {
    spell_form: u8,
    freindly: u8,
    location: u16,
    memorised_icon: FixedCharSlice<8>,
    target_type: u8,
    target_count: u8,
    range: u16,
    level_required: u16,
    casting_time: u16,
    times_per_day: u16,
    dice_sides: u16,
    dice_thrown: u16,
    enchanted: u16,
    damage_type: u16,
    count_of_feature_blocks: i16,
    offset_to_feature_blocks: i16,
    charges: u16,
    projectile: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm#splv1_Feature_Block
type SpellFeatureBlock = FeatureBlock;

#[cfg(test)]
mod tests {

    use crate::spell::Spell;

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_creature_file_item_table_parsed() {
        let file = File::open("fixtures/gate1.spl").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let spell = Spell::new(&buffer);
        assert_eq!(
            spell.header,
            SpellHeader {
                header: Header {
                    signature: "SPL ".into(),
                    version: "V1  ".into(),
                },
                unidentified_spell_name: 14260,
                identified_spell_name: 9999999,
                completion_sound: FixedCharSlice([67, 65, 83, 95, 77, 48, 51, 0]),
                flags: 0,
                spell_type: 1,
                exclusion_flags: 0,
                casting_graphics: FixedCharSlice([18, 0]),
                min_level: 0,
                primary_spell_school: 2,
                min_strength: 0,
                secondary_spell_school: 6,
                min_strenth_bonus: 0,
                kit_usability_1: 0,
                min_intelligence: 0,
                kit_usability_2: 0,
                min_dexterity: 0,
                kit_usability_3: 0,
                min_wisdom: 0,
                kit_usability_4: 0,
                min_constitution: 0,
                min_charisma: 0,
                spell_level: 9,
                max_stackable: 1,
                spellbook_icon: FixedCharSlice([83, 80, 87, 73, 57, 48, 53, 67]),
                lore: 0,
                ground_icon: FixedCharSlice([0, 0, 114, 98, 0, 0, 85, 110]),
                base_weight: 0,
                spell_description_generic: SignedFixedCharSlice([-1, -1, -1, -1]),
                spell_description_identified: SignedFixedCharSlice([127, -106, -104, 0]),
                description_icon: SignedFixedCharSlice([0, 0, 0, 104, -122, 64, 0, 5]),
                enchantment: 0,
                offset_to_extended_headers: 114,
                count_of_extended_headers: 1,
                offset_to_feature_block_table: 154,
                offset_to_casting_feature_blocks: 0,
                count_of_casting_feature_blocks: 0
            }
        )
    }
}
