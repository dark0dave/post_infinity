use serde::{Deserialize, Serialize};

use crate::common::fixed_char_array::FixedCharSlice;

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct KnownSpells {
    pub spell_name: FixedCharSlice<8>,
    pub spell_level: u16,
    pub spell_type: u16,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct SpellMemorizationInfo {
    pub spell_level: u16,
    pub number_of_spells_memorizable: u16,
    pub number_of_spells_memorizable_after_effects: u16,
    pub spell_type: u16,
    pub index_to_spell_table: i32,
    pub count_of_memorizable_spell_tables: i32,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct SpellMemorizationTable {
    pub spell_name: FixedCharSlice<8>,
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
                    spell_name: "SPPR103".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR103".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR109".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR101".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR203".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR208".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR211".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR212".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR312".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR313".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR315".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR401".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR413".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR411".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR502".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPPR503".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI113".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI112".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI110".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI105".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI213".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI220".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI211".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI203".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI312".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI311".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI318".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI308".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI408".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI405".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI406".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI510".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI505".into(),
                    memorised: 1,
                },
                SpellMemorizationTable {
                    spell_name: "SPWI522".into(),
                    memorised: 1,
                },
            ]
        );
    }
}
