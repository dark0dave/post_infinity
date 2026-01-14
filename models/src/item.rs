use binrw::{BinRead, BinReaderExt, BinWrite, helpers::until_eof, io::Cursor};
use serde::{Deserialize, Serialize};

use crate::common::char_array::CharArray;
use crate::common::feature_block::FeatureBlock;
use crate::common::header::Header;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize, PartialEq)]
pub struct Item {
    #[serde(skip)]
    #[br(parse_with = until_eof, restore_position)]
    pub original_bytes: Vec<u8>,
    #[bw(ignore)]
    #[serde(flatten)]
    pub header: ItemHeader,
    #[bw(ignore)]
    #[br(count=header.count_of_extended_headers)]
    pub extended_headers: Vec<ItemExtendedHeader>,
    #[bw(ignore)]
    #[br(count=header.count_of_feature_blocks)]
    pub equipping_feature_blocks: Vec<ItemFeatureBlock>,
}
impl Model for Item {
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

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm#itmv1_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize, PartialEq)]
pub struct ItemHeader {
    #[serde(flatten)]
    pub header: Header,
    unidentified_item_name: u32,
    identified_item_name: u32,
    replacement_item: CharArray<8>,
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm#Header_Flags
    type_flags: u32,
    category: u16,
    usability: u32,
    item_animation: CharArray<2>,
    min_level: u16,
    min_strength: u16,
    min_strength_bonus: u8,
    kit_usability_1: u8,
    min_intelligence: u8,
    kit_usability_2: u8,
    min_dexterity: u8,
    kit_usability_3: u8,
    min_wisdom: u8,
    kit_usability_4: u8,
    min_constitution: u8,
    weapon_proficiency: u8,
    min_charisma: u16,
    base_value: u32,
    max_stackable: u16,
    item_icon: CharArray<8>,
    lore: u16,
    ground_icon: CharArray<8>,
    base_weight: u32,
    item_description_generic: u32,
    item_description_identified: u32,
    description_icon: CharArray<8>,
    enchantment: u32,
    offset_to_extended_headers: u32,
    count_of_extended_headers: u16,
    offset_to_feature_blocks: u32,
    index_to_equipping_feature_blocks: u16,
    count_of_feature_blocks: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm#itmv1_Extended_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize, PartialEq)]
pub struct ItemExtendedHeader {
    attack_type: u8, // Note zero is very bad here
    id_required: u8,
    location: u8,
    alternative_dice_sides: u8,
    use_icon: CharArray<8>,
    target_type: u8,
    target_count: u8,
    range: u16,
    launcher_required: u8,
    alternative_dice_thrown: u8,
    speed_factor: u8,
    alternative_damage_bonus: u8,
    thaco: u16,
    dice_sides: u8,
    primary_type_school: u8,
    dice_thrown: u8,
    secondary_type: u8,
    damage_bonus: u16,
    damage_type: u16,
    feature_blocks_count: u16,
    feature_blocks_index: u16,
    max_charges: u16,
    charge_depletion_behaviour: u16,
    #[br(count = 4)]
    flags: Vec<u8>,
    projectile_animation: u16,
    #[br(count = 6)]
    melee_animation: Vec<u8>,
    is_arrow: u16,
    is_bolt: u16,
    is_bullet: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm#itmv1_Feature_Block
type ItemFeatureBlock = FeatureBlock;

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::io::Read;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::{error::Error, fs::File};

    const FIXTURES: [(&str, &str); 3] = [
        ("fixtures/gopoof.itm", "fixtures/gopoof.itm.json"),
        ("fixtures/sw1h01.itm", "fixtures/sw1h01.itm.json"),
        ("fixtures/zbpdnote.itm", "fixtures/zbpdnote.itm.json"),
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
            let item: Item = Item::new(&read_file(file_path)?);
            let result: Value = serde_json::to_value(item)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
