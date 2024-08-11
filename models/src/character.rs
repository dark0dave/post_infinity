use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::resref::Resref;
use crate::tlk::Lookup;
use crate::{creature::Creature, model::Model};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/chr_v2.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct ExpandedCharacter {
    pub character: BGCharacter,
    pub creature: Creature,
}

impl Model for ExpandedCharacter {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
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

#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BGCharacter {
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub signature: String,
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub version: String,
    #[br(count = 32)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub name: String,
    pub offset_to_cre_structure: u32,
    pub length_of_the_cre_structure: u32,
    pub index_into_slots_ids_for_quick_weapon_1: u16,
    pub index_into_slots_ids_for_quick_weapon_2: u16,
    pub index_into_slots_ids_for_quick_weapon_3: u16,
    pub index_into_slots_ids_for_quick_weapon_4: u16,
    pub show_quick_weapon_1: u16,
    pub show_quick_weapon_2: u16,
    pub show_quick_weapon_3: u16,
    pub show_quick_weapon_4: u16,
    pub quick_spell_1_resource: Resref,
    pub quick_spell_2_resource: Resref,
    pub quick_spell_3_resource: Resref,
    pub index_into_slot_ids_for_quick_item_1: u16,
    pub index_into_slot_ids_for_quick_item_2: u16,
    pub index_into_slot_ids_for_quick_item_3: u16,
    pub show_quick_item_1: u16,
    pub show_quick_item_2: u16,
    pub show_quick_item_3: u16,
}
