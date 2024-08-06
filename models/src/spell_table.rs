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

#[cfg(test)]
mod tests {

    use crate::{creature::Creature, model::Model};
    use pretty_assertions::assert_eq;

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

        let creature = Creature::new(&buffer);
        assert_eq!(
            creature.memorized_spell_info.first(),
            Some(&SpellMemorizationInfo {
                spell_level: 0,
                number_of_spells_memorizable: 4,
                number_of_spells_memorizable_after_effects: 4,
                spell_type: 0,
                index_to_spell_table: 0,
                count_of_memorizable_spell_tables: 4,
            })
        );
        assert_eq!(
            creature.memorized_spells,
            vec![
                SpellMemorizationTable {
                    spell_name: Resref("SPPR103\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR103\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR109\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR101\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR203\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR208\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR211\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR212\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR312\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR313\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR315\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR401\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR413\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR411\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR502\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPPR503\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI113\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI112\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI110\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI105\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI213\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI220\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI211\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI203\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI312\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI311\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI318\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI308\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI408\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI405\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI406\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI510\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI505\0".to_string()),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: Resref("SPWI522\0".to_string()),
                    memorised: 1,
                },
            ]
        );
    }
}
