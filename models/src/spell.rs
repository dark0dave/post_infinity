use binrw::{helpers::until_eof, io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::feature_block::FeatureBlock;
use crate::common::header::Header;
use crate::common::strref::Strref;
use crate::common::Resref;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Spell {
    #[serde(skip)]
    #[br(parse_with = until_eof, restore_position)]
    pub original_bytes: Vec<u8>,
    #[bw(ignore)]
    #[serde(flatten)]
    pub header: SpellHeader,
    #[bw(ignore)]
    #[br(count=header.count_of_extended_headers)]
    pub extended_headers: Vec<SpellExtendedHeader>,
    #[bw(ignore)]
    #[br(parse_with=binrw::helpers::until_eof)]
    pub equipping_feature_blocks: Vec<SpellFeatureBlock>,
}

impl Model for Spell {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {err:?}, dumping buffer: {buffer:?}");
            }
        }
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
    #[serde(flatten)]
    pub header: Header,
    pub unidentified_spell_name: u32,
    pub identified_spell_name: u32,
    pub completion_sound: Resref,
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v2.htm#Header_Flags
    pub flags: u32,
    pub spell_type: u16,
    pub exclusion_flags: u32,
    pub casting_graphics: u16,
    pub min_level: u8,
    pub primary_spell_school: u8,
    pub min_strength: u8,
    pub secondary_spell_school: u8,
    pub min_strength_bonus: u8,
    pub kit_usability_1: u8,
    pub min_intelligence: u8,
    pub kit_usability_2: u8,
    pub min_dexterity: u8,
    pub kit_usability_3: u8,
    pub min_wisdom: u8,
    pub kit_usability_4: u8,
    pub min_constitution: u16,
    pub min_charisma: u16,
    pub spell_level: u32,
    pub max_stackable: u16,
    pub spell_book_icon: Resref,
    pub lore: u16,
    pub ground_icon: Resref,
    pub base_weight: u32,
    pub spell_description_generic: Strref,
    pub spell_description_identified: Strref,
    pub description_icon: Resref,
    pub enchantment: u32,
    pub offset_to_extended_headers: u32,
    pub count_of_extended_headers: u16,
    pub offset_to_feature_block_table: u32,
    pub offset_to_casting_feature_blocks: u16,
    pub count_of_casting_feature_blocks: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm#splv1_Extended_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SpellExtendedHeader {
    pub spell_form: u8,
    pub friendly: u8,
    pub location: u16,
    pub memorised_icon: Resref,
    pub target_type: u8,
    pub target_count: u8,
    pub range: u16,
    pub level_required: u16,
    pub casting_time: u16,
    pub times_per_day: u16,
    pub dice_sides: u16,
    pub dice_thrown: u16,
    pub enchanted: u16,
    pub damage_type: u16,
    pub count_of_feature_blocks: u16,
    pub offset_to_feature_blocks: u16,
    pub charges: u16,
    pub charge_depletion_behaviour: u16,
    pub projectile: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm#splv1_Feature_Block
type SpellFeatureBlock = FeatureBlock;

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::io::Read;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::{error::Error, fs::File};

    const FIXTURES: [(&str, &str); 1] = [("fixtures/gate1.spl", "fixtures/gate1.spl.json")];

    fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let spell: Spell = Spell::new(&read_file(file_path)?);
            let result: Value = serde_json::to_value(spell)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
