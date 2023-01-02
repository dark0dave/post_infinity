use std::rc::Rc;

use crate::common::feature_block::FeatureBlock;
use crate::utils::copy_buff_to_struct;
use crate::{model::Model, utils::copy_transmute_buff};

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm
#[derive(Debug)]
pub struct Item {
    pub header: ItemHeader,
    pub extended_headers: Vec<ItemExtendedHeader>,
    pub equiping_feature_blocks: Vec<ItemFeatureBlock>,
}

impl Model for Item {
    fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<ItemHeader>(buffer, 0);

        let start = usize::try_from(header.offset_to_extended_headers).unwrap_or(0);
        let count = usize::try_from(header.count_of_extended_headers).unwrap_or(0);
        let extended_headers = copy_transmute_buff::<ItemExtendedHeader>(buffer, start, count);

        let start = usize::try_from(header.offset_to_feature_blocks).unwrap_or(0);
        let count = usize::try_from(header.count_of_feature_blocks).unwrap_or(0);
        let equiping_feature_blocks = copy_transmute_buff::<ItemFeatureBlock>(buffer, start, count);

        Self {
            header,
            extended_headers,
            equiping_feature_blocks,
        }
    }
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }
}

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm#itmv1_Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ItemHeader {
    signature: [u8; 4],
    version: [u8; 4],
    unidentified_item_name: [i8; 4],
    identified_item_name: [i8; 4],
    replacement_item: [i8; 8],
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm#Header_Flags
    type_flags: u32,
    category: u16,
    usability: u32,
    item_animation: [u8; 2],
    min_level: u16,
    min_strength: u16,
    min_strengthbonus: u8,
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
    item_icon: [u8; 8],
    lore: u16,
    ground_icon: [u8; 8],
    base_weight: u32,
    item_description_generic: [i8; 4],
    item_description_identified: [i8; 4],
    description_icon: [i8; 8],
    enchantment: u32,
    offset_to_extended_headers: i32,
    count_of_extended_headers: i16,
    offset_to_feature_blocks: i32,
    index_to_equiping_feature_blocks: i16,
    count_of_feature_blocks: i16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm#itmv1_Extended_Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ItemExtendedHeader {
    attack_type: u8, // Note zero is very bad here
    id_required: u8,
    location: u8,
    alternative_dice_sides: u8,
    use_icon: [u8; 8],
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
    flags: [u8; 4],
    projectile_animation: [u8; 2],
    melee_animation: [[u8; 2]; 3],
    is_arrow: u16,
    is_bolt: u16,
    is_bullet: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm#itmv1_Feature_Block
type ItemFeatureBlock = FeatureBlock;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_item_file_parsed() {
        let file = File::open("fixtures/gopoof.itm").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let item = Item::new(&buffer);
        assert_eq!({ item.header.max_stackable }, 1);
        assert_eq!({ item.extended_headers[0].attack_type }, 3);
        assert_eq!({ item.extended_headers[0].max_charges }, 2);
        assert_eq!({ item.extended_headers[0].is_arrow }, 0);
        assert_eq!({ item.extended_headers[0].is_bolt }, 0);
        assert_eq!({ item.equiping_feature_blocks[0].duration }, 1);
        assert_eq!({ item.equiping_feature_blocks[0].saving_throw_bonus }, 0);
        assert_eq!({ item.equiping_feature_blocks[0].stacking_id }, 0);
    }
}
