use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::common::fixed_char_array::FixedCharSlice;
use crate::common::signed_fixed_char_array::SignedFixedCharSlice;
use crate::resources::utils::{copy_buff_to_struct, to_u8_slice};
use crate::tlk::Lookup;
use crate::{common::header::Header, creature::Creature, model::Model};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedCharacter {
    pub character: BGCharacter,
    pub creature: Creature,
}

impl Model for ExpandedCharacter {
    fn new(buffer: &[u8]) -> Self {
        let character = copy_buff_to_struct::<BGCharacter>(buffer, 0);

        let start = usize::try_from(character.offset_to_cre_structure).unwrap_or(0);
        let end = start + usize::try_from(character.length_of_the_cre_structure).unwrap_or(0);

        let creature = Creature::new(buffer.get(start..end).unwrap());

        Self {
            character,
            creature,
        }
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(to_u8_slice(&self.character).to_vec());
        out.extend(self.creature.to_bytes());
        out
    }
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct BGCharacter {
    pub header: Header<4, 4>,
    pub name: FixedCharSlice<32>,
    pub offset_to_cre_structure: i32,
    pub length_of_the_cre_structure: i32,
    pub index_into_slots_ids_for_quick_weapon_1: i16,
    pub index_into_slots_ids_for_quick_weapon_2: i16,
    pub index_into_slots_ids_for_quick_weapon_3: i16,
    pub index_into_slots_ids_for_quick_weapon_4: i16,
    pub show_quick_weapon_1: i16,
    pub show_quick_weapon_2: i16,
    pub show_quick_weapon_3: i16,
    pub show_quick_weapon_4: i16,
    pub quick_spell_1_resource: SignedFixedCharSlice<8>,
    pub quick_spell_2_resource: SignedFixedCharSlice<8>,
    pub quick_spell_3_resource: SignedFixedCharSlice<8>,
    pub index_into_slot_ids_for_quick_item_1: i16,
    pub index_into_slot_ids_for_quick_item_2: i16,
    pub index_into_slot_ids_for_quick_item_3: i16,
    pub show_quick_item_1: i16,
    pub show_quick_item_2: i16,
    pub show_quick_item_3: i16,
}
