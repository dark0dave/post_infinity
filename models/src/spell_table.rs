use serde::{Deserialize, Serialize};

use crate::common::resref::Resref;
use binrw::{BinRead, BinWrite};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_KnownSpell
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct KnownSpells {
    pub spell_name: Resref,
    pub spell_level: u16,
    pub spell_type: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_MemSpellInfo
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SpellMemorizationInfo {
    pub spell_level: u16,
    pub number_of_spells_memorizable: u16,
    pub number_of_spells_memorizable_after_effects: u16,
    pub spell_type: u16,
    pub index_to_spell_table: u32,
    pub count_of_memorizable_spell_tables: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_MemSpell
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SpellMemorizationTable {
    pub spell_name: Resref,
    pub memorised: u32,
}
