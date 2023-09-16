use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::resources::utils::{copy_buff_to_struct, to_u8_slice};
use crate::tlk::Lookup;
use crate::{common::fixed_char_array::FixedCharSlice, model::Model};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_Item
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct ItemReferenceTable {
    pub resource_name: FixedCharSlice<8>,
    // Item expiration time - item creation hour (replace with drained item)
    pub item_expiration_time_hour: u8,
    /*
      Item expiration time - (elapsed hour count divided by 256, rounded down) + 1 (replace with drained item)
      When the game hour and elapsed hour count for the current game time exceed these values, the item is removed.
    */
    pub item_expiration_time: u8,
    pub quantity_1: u16,
    pub quantity_2: u16,
    pub quantity_3: u16,
    pub identified: u8,
    pub unstealable: u8,
    pub stolen: u8,
    pub undroppable: u8,
}

impl Model for ItemReferenceTable {
    fn new(buffer: &[u8]) -> Self {
        copy_buff_to_struct::<Self>(buffer, 0)
    }
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        self.resource_name.to_string()
    }

    fn to_bytes(&self) -> Vec<u8> {
        to_u8_slice(&self).to_vec()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_ItemSlots
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct ItemSlots {
    helmet: i16,
    armor: i16,
    shield: i16,
    gloves: i16,
    left_ring: i16,
    right_ring: i16,
    amulet: i16,
    belt: i16,
    boots: i16,
    weapon_1: i16,
    weapon_2: i16,
    weapon_3: i16,
    weapon_4: i16,
    quiver_1: i16,
    quiver_2: i16,
    quiver_3: i16,
    // Cannot be accesed from gui
    quiver_4: i16,
    cloak: i16,
    quick_item_1: i16,
    quick_item_2: i16,
    quick_item_3: i16,
    inventory_item_1: i16,
    inventory_item_2: i16,
    inventory_item_3: i16,
    inventory_item_4: i16,
    inventory_item_5: i16,
    inventory_item_6: i16,
    inventory_item_7: i16,
    inventory_item_8: i16,
    inventory_item_9: i16,
    inventory_item_10: i16,
    inventory_item_11: i16,
    inventory_item_12: i16,
    inventory_item_13: i16,
    inventory_item_14: i16,
    inventory_item_15: i16,
    inventory_item_16: i16,
    magic_weapon: i16,
    weapon_slot_selected: i16,
    weapon_ability_selected: i16,
}

impl Model for ItemSlots {
    fn new(buffer: &[u8]) -> Self {
        copy_buff_to_struct::<Self>(buffer, 0)
    }
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        to_u8_slice(&self).to_vec()
    }
}

#[cfg(test)]
mod tests {

    use crate::creature::BGEECreature;

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_creature_file_item_table_parsed() {
        let file = File::open("fixtures/cutmelis.cre").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let header = copy_buff_to_struct::<BGEECreature>(&buffer, 0);

        let start = usize::try_from(header.offset_to_item_slots).unwrap_or(0);
        let item_slots = copy_buff_to_struct::<ItemSlots>(&buffer, start);
        assert_eq!(
            item_slots,
            ItemSlots {
                helmet: 2,
                armor: -1,
                shield: -1,
                gloves: -1,
                left_ring: 1,
                right_ring: 3,
                amulet: -1,
                belt: -1,
                boots: -1,
                weapon_1: -1,
                weapon_2: -1,
                weapon_3: -1,
                weapon_4: -1,
                quiver_1: -1,
                quiver_2: -1,
                quiver_3: -1,
                quiver_4: -1,
                cloak: -1,
                quick_item_1: -1,
                quick_item_2: -1,
                quick_item_3: -1,
                inventory_item_1: 4,
                inventory_item_2: 5,
                inventory_item_3: -1,
                inventory_item_4: -1,
                inventory_item_5: -1,
                inventory_item_6: -1,
                inventory_item_7: -1,
                inventory_item_8: -1,
                inventory_item_9: -1,
                inventory_item_10: -1,
                inventory_item_11: -1,
                inventory_item_12: -1,
                inventory_item_13: -1,
                inventory_item_14: -1,
                inventory_item_15: -1,
                inventory_item_16: -1,
                magic_weapon: -1,
                weapon_slot_selected: 0,
                weapon_ability_selected: 0
            }
        )
    }
}
