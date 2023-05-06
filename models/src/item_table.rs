use std::rc::Rc;

use serde::Serialize;

use crate::resources::utils::{copy_buff_to_struct, copy_transmute_buff};
use crate::tlk::Lookup;
use crate::{common::fixed_char_array::FixedCharSlice, model::Model};

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct ItemTable {
    pub helmet: Option<ItemReferenceTable>,
    pub armor: Option<ItemReferenceTable>,
    pub shield: Option<ItemReferenceTable>,
    pub gloves: Option<ItemReferenceTable>,
    pub left_ring: Option<ItemReferenceTable>,
    pub right_ring: Option<ItemReferenceTable>,
    pub amulet: Option<ItemReferenceTable>,
    pub belt: Option<ItemReferenceTable>,
    pub boots: Option<ItemReferenceTable>,
    pub weapon_1: Option<ItemReferenceTable>,
    pub weapon_2: Option<ItemReferenceTable>,
    pub weapon_3: Option<ItemReferenceTable>,
    pub weapon_4: Option<ItemReferenceTable>,
    pub quiver_1: Option<ItemReferenceTable>,
    pub quiver_2: Option<ItemReferenceTable>,
    pub quiver_3: Option<ItemReferenceTable>,
    // Cannot be accesed from gui
    pub quiver_4: Option<ItemReferenceTable>,
    pub cloak: Option<ItemReferenceTable>,
    pub quick_item_1: Option<ItemReferenceTable>,
    pub quick_item_2: Option<ItemReferenceTable>,
    pub quick_item_3: Option<ItemReferenceTable>,
    pub inventory_item_1: Option<ItemReferenceTable>,
    pub inventory_item_2: Option<ItemReferenceTable>,
    pub inventory_item_3: Option<ItemReferenceTable>,
    pub inventory_item_4: Option<ItemReferenceTable>,
    pub inventory_item_5: Option<ItemReferenceTable>,
    pub inventory_item_6: Option<ItemReferenceTable>,
    pub inventory_item_7: Option<ItemReferenceTable>,
    pub inventory_item_8: Option<ItemReferenceTable>,
    pub inventory_item_9: Option<ItemReferenceTable>,
    pub inventory_item_10: Option<ItemReferenceTable>,
    pub inventory_item_11: Option<ItemReferenceTable>,
    pub inventory_item_12: Option<ItemReferenceTable>,
    pub inventory_item_13: Option<ItemReferenceTable>,
    pub inventory_item_14: Option<ItemReferenceTable>,
    pub inventory_item_15: Option<ItemReferenceTable>,
    pub inventory_item_16: Option<ItemReferenceTable>,
    pub magic_weapon: Option<ItemReferenceTable>,
    pub weapon_slot_selected: i16,
    pub weapon_ability_selected: i16,
}

impl ItemTable {
    fn lookup_value(index: i16, item_tables: &[ItemReferenceTable]) -> Option<ItemReferenceTable> {
        if index > -1 {
            return item_tables.get(index as usize).copied();
        }
        None
    }
    pub fn generate(
        buffer: &[u8],
        offset_to_items: usize,
        count_of_items: usize,
        offset_to_item_slots: usize,
    ) -> Self {
        let item_tables =
            copy_transmute_buff::<ItemReferenceTable>(buffer, offset_to_items, count_of_items);
        let item_slot_table = copy_buff_to_struct::<ItemSlotTable>(buffer, offset_to_item_slots);

        Self {
            helmet: Self::lookup_value(item_slot_table.helmet, &item_tables),
            armor: Self::lookup_value(item_slot_table.armor, &item_tables),
            shield: Self::lookup_value(item_slot_table.shield, &item_tables),
            gloves: Self::lookup_value(item_slot_table.gloves, &item_tables),
            left_ring: Self::lookup_value(item_slot_table.left_ring, &item_tables),
            right_ring: Self::lookup_value(item_slot_table.right_ring, &item_tables),
            amulet: Self::lookup_value(item_slot_table.amulet, &item_tables),
            belt: Self::lookup_value(item_slot_table.belt, &item_tables),
            boots: Self::lookup_value(item_slot_table.boots, &item_tables),
            weapon_1: Self::lookup_value(item_slot_table.weapon_1, &item_tables),
            weapon_2: Self::lookup_value(item_slot_table.weapon_2, &item_tables),
            weapon_3: Self::lookup_value(item_slot_table.weapon_3, &item_tables),
            weapon_4: Self::lookup_value(item_slot_table.weapon_4, &item_tables),
            quiver_1: Self::lookup_value(item_slot_table.quiver_1, &item_tables),
            quiver_2: Self::lookup_value(item_slot_table.quiver_2, &item_tables),
            quiver_3: Self::lookup_value(item_slot_table.quiver_3, &item_tables),
            quiver_4: Self::lookup_value(item_slot_table.quiver_4, &item_tables),
            cloak: Self::lookup_value(item_slot_table.cloak, &item_tables),
            quick_item_1: Self::lookup_value(item_slot_table.quick_item_1, &item_tables),
            quick_item_2: Self::lookup_value(item_slot_table.quick_item_2, &item_tables),
            quick_item_3: Self::lookup_value(item_slot_table.quick_item_3, &item_tables),
            inventory_item_1: Self::lookup_value(item_slot_table.inventory_item_1, &item_tables),
            inventory_item_2: Self::lookup_value(item_slot_table.inventory_item_2, &item_tables),
            inventory_item_3: Self::lookup_value(item_slot_table.inventory_item_3, &item_tables),
            inventory_item_4: Self::lookup_value(item_slot_table.inventory_item_4, &item_tables),
            inventory_item_5: Self::lookup_value(item_slot_table.inventory_item_5, &item_tables),
            inventory_item_6: Self::lookup_value(item_slot_table.inventory_item_6, &item_tables),
            inventory_item_7: Self::lookup_value(item_slot_table.inventory_item_7, &item_tables),
            inventory_item_8: Self::lookup_value(item_slot_table.inventory_item_8, &item_tables),
            inventory_item_9: Self::lookup_value(item_slot_table.inventory_item_9, &item_tables),
            inventory_item_10: Self::lookup_value(item_slot_table.inventory_item_10, &item_tables),
            inventory_item_11: Self::lookup_value(item_slot_table.inventory_item_11, &item_tables),
            inventory_item_12: Self::lookup_value(item_slot_table.inventory_item_12, &item_tables),
            inventory_item_13: Self::lookup_value(item_slot_table.inventory_item_13, &item_tables),
            inventory_item_14: Self::lookup_value(item_slot_table.inventory_item_14, &item_tables),
            inventory_item_15: Self::lookup_value(item_slot_table.inventory_item_15, &item_tables),
            inventory_item_16: Self::lookup_value(item_slot_table.inventory_item_16, &item_tables),
            magic_weapon: Self::lookup_value(item_slot_table.magic_weapon, &item_tables),
            weapon_slot_selected: item_slot_table.weapon_slot_selected,
            weapon_ability_selected: item_slot_table.weapon_ability_selected,
        }
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_Item
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
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
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_ItemSlots
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
struct ItemSlotTable {
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

impl Model for ItemSlotTable {
    fn new(buffer: &[u8]) -> Self {
        copy_buff_to_struct::<Self>(buffer, 0)
    }
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::creature::{BGEECreature, Creature};

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
        mem::size_of,
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
        let item_slot_table = ItemTable::generate(
            &buffer,
            header.offset_to_items as usize,
            header.count_of_items as usize,
            header.offset_to_item_slots as usize,
        );
        assert_eq!(
            item_slot_table,
            ItemTable {
                helmet: Some(ItemReferenceTable {
                    resource_name: "HELMNOAN".into(),
                    item_expiration_time_hour: 0,
                    item_expiration_time: 0,
                    quantity_1: 0,
                    quantity_2: 0,
                    quantity_3: 0,
                    identified: 0,
                    unstealable: 0,
                    stolen: 0,
                    undroppable: 0
                }),
                armor: None,
                shield: None,
                gloves: None,
                left_ring: Some(ItemReferenceTable {
                    resource_name: "RING07".into(),
                    item_expiration_time_hour: 0,
                    item_expiration_time: 0,
                    quantity_1: 0,
                    quantity_2: 0,
                    quantity_3: 0,
                    identified: 0,
                    unstealable: 0,
                    stolen: 0,
                    undroppable: 0
                }),
                right_ring: Some(ItemReferenceTable {
                    resource_name: "IMOENHP1".into(),
                    item_expiration_time_hour: 0,
                    item_expiration_time: 0,
                    quantity_1: 0,
                    quantity_2: 0,
                    quantity_3: 0,
                    identified: 0,
                    unstealable: 0,
                    stolen: 0,
                    undroppable: 0
                }),
                amulet: None,
                belt: None,
                boots: None,
                weapon_1: None,
                weapon_2: None,
                weapon_3: None,
                weapon_4: None,
                quiver_1: None,
                quiver_2: None,
                quiver_3: None,
                quiver_4: None,
                cloak: None,
                quick_item_1: None,
                quick_item_2: None,
                quick_item_3: None,
                inventory_item_1: Some(ItemReferenceTable {
                    resource_name: "DW#RND08".into(),
                    item_expiration_time_hour: 0,
                    item_expiration_time: 0,
                    quantity_1: 0,
                    quantity_2: 0,
                    quantity_3: 0,
                    identified: 0,
                    unstealable: 0,
                    stolen: 0,
                    undroppable: 0
                }),
                inventory_item_2: Some(ItemReferenceTable {
                    resource_name: "DW#RND16".into(),
                    item_expiration_time_hour: 0,
                    item_expiration_time: 0,
                    quantity_1: 0,
                    quantity_2: 0,
                    quantity_3: 0,
                    identified: 0,
                    unstealable: 0,
                    stolen: 0,
                    undroppable: 0
                }),
                inventory_item_3: None,
                inventory_item_4: None,
                inventory_item_5: None,
                inventory_item_6: None,
                inventory_item_7: None,
                inventory_item_8: None,
                inventory_item_9: None,
                inventory_item_10: None,
                inventory_item_11: None,
                inventory_item_12: None,
                inventory_item_13: None,
                inventory_item_14: None,
                inventory_item_15: None,
                inventory_item_16: None,
                magic_weapon: None,
                weapon_slot_selected: 0,
                weapon_ability_selected: 0
            }
        )
    }
    #[test]
    fn valid_creature_file_item_slot_table_parsed() {
        let file = File::open("fixtures/cutmelis.cre").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let creature = Creature::new(&buffer);
        let start = creature.header.offset_to_item_slots as usize;
        let end = start + size_of::<ItemSlotTable>();
        let item_slot_buffer = buffer.get(start..end).unwrap();
        let item_slot_table = ItemSlotTable::new(&item_slot_buffer);
        assert_eq!(
            item_slot_table,
            ItemSlotTable {
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
        );
    }
}
