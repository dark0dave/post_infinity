use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::feature_block::FeatureBlock;
use crate::common::resref::Resref;
use crate::common::strref::Strref;
use crate::model::Model;

use crate::tlk::Lookup;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Spell {
    #[serde(flatten)]
    pub header: SpellHeader,
    #[br(count=header.count_of_extended_headers)]
    pub extended_headers: Vec<SpellExtendedHeader>,
    #[br(parse_with=binrw::helpers::until_eof)]
    pub equipping_feature_blocks: Vec<SpellFeatureBlock>,
}

impl Model for Spell {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}, dumping buffer: {:?}", err, buffer);
            }
        }
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm#splv1_Header
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SpellHeader {
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    signature: String,
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    version: String,
    unidentified_spell_name: u32,
    identified_spell_name: u32,
    completion_sound: Resref,
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v2.htm#Header_Flags
    flags: u32,
    spell_type: u16,
    exclusion_flags: u32,
    casting_graphics: u16,
    min_level: u8,
    primary_spell_school: u8,
    min_strength: u8,
    secondary_spell_school: u8,
    min_strength_bonus: u8,
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
    spell_book_icon: Resref,
    lore: u16,
    ground_icon: Resref,
    base_weight: u32,
    spell_description_generic: Strref,
    spell_description_identified: Strref,
    description_icon: Resref,
    enchantment: u32,
    offset_to_extended_headers: u32,
    count_of_extended_headers: u16,
    offset_to_feature_block_table: u32,
    offset_to_casting_feature_blocks: u16,
    count_of_casting_feature_blocks: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm#splv1_Extended_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SpellExtendedHeader {
    spell_form: u8,
    friendly: u8,
    location: u16,
    memorised_icon: Resref,
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
    count_of_feature_blocks: u16,
    offset_to_feature_blocks: u16,
    charges: u16,
    charge_depletion_behaviour: u16,
    projectile: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm#splv1_Feature_Block
type SpellFeatureBlock = FeatureBlock;

#[cfg(test)]
mod tests {

    use crate::common::resref::Resref;

    use super::*;
    use pretty_assertions::assert_eq;
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
                signature: "SPL ".to_string(),
                version: "V1  ".to_string(),
                unidentified_spell_name: 14260,
                identified_spell_name: 9999999,
                completion_sound: Resref("CAS_M03\0".to_string()),
                flags: 0,
                spell_type: 1,
                exclusion_flags: 0,
                casting_graphics: 18,
                min_level: 0,
                primary_spell_school: 2,
                min_strength: 0,
                secondary_spell_school: 6,
                min_strength_bonus: 0,
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
                spell_book_icon: Resref("SPWI905C".to_string()),
                lore: 0,
                ground_icon: Resref("\0\0rb\0\0Un".to_string()),
                base_weight: 0,
                spell_description_generic: Strref(4294967295),
                spell_description_identified: Strref(9999999),
                description_icon: Resref("".to_string()),
                enchantment: 0,
                offset_to_extended_headers: 114,
                count_of_extended_headers: 1,
                offset_to_feature_block_table: 154,
                offset_to_casting_feature_blocks: 0,
                count_of_casting_feature_blocks: 0
            }
        );
        assert_eq!(
            spell.equipping_feature_blocks,
            vec![FeatureBlock {
                opcode_number: 177,
                target_type: 1,
                power: 9,
                parameter_1: 0,
                parameter_2: 2,
                timing_mode: 0,
                dispel_resistance: 2,
                duration: 100000,
                probability_1: 39,
                probability_2: 0,
                resource: Resref("balorsu\0".to_string()),
                dice_thrown_max_level: 0,
                dice_sides_min_level: 0,
                saving_throw_type: vec![0, 0, 0, 0],
                saving_throw_bonus: 0,
                stacking_id: 0
            }]
        )
    }
}
