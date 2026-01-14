use binrw::{BinRead, BinReaderExt, BinWrite, io::Cursor};
use serde::{Deserialize, Serialize};

use crate::common::Resref;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_Item
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct ItemReferenceTable {
    pub resource_name: Resref,
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
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_ItemSlots
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct ItemSlots {
    helmet: u16,
    armor: u16,
    shield: u16,
    gloves: u16,
    left_ring: u16,
    right_ring: u16,
    amulet: u16,
    belt: u16,
    boots: u16,
    weapon_1: u16,
    weapon_2: u16,
    weapon_3: u16,
    weapon_4: u16,
    quiver_1: u16,
    quiver_2: u16,
    quiver_3: u16,
    // Cannot be accessed from gui
    quiver_4: u16,
    cloak: u16,
    quick_item_1: u16,
    quick_item_2: u16,
    quick_item_3: u16,
    inventory_item_1: u16,
    inventory_item_2: u16,
    inventory_item_3: u16,
    inventory_item_4: u16,
    inventory_item_5: u16,
    inventory_item_6: u16,
    inventory_item_7: u16,
    inventory_item_8: u16,
    inventory_item_9: u16,
    inventory_item_10: u16,
    inventory_item_11: u16,
    inventory_item_12: u16,
    inventory_item_13: u16,
    inventory_item_14: u16,
    inventory_item_15: u16,
    inventory_item_16: u16,
    magic_weapon: u16,
    weapon_slot_selected: u16,
    weapon_ability_selected: u16,
}

impl Model for ItemSlots {
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
