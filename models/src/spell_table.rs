use std::mem::size_of;

use serde::Serialize;

use crate::common::fixed_char_array::FixedCharSlice;
use crate::resources::utils::copy_transmute_buff;

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct KnownSpells {
    pub spell_name: FixedCharSlice<8>,
    pub spell_level: u16,
    pub spell_type: u16,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct SpellMemorizationInfo {
    pub spell_level: u16,
    pub number_of_spells_memorizable: u16,
    pub number_of_spells_memorizable_after_effects: u16,
    pub spell_type: u16,
    pub index_to_spell_table: i32,
    pub count_of_memorizable_spell_tables: i32,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct SpellMemorizationTable {
    pub spell_name: FixedCharSlice<8>,
    pub memorised: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct MemorizedSpells {
    pub spell_memorization_info: SpellMemorizationInfo,
    pub spell_memorization_table: Vec<SpellMemorizationTable>,
}
// Slow
pub fn generate_spell_memorization(
    buffer: &[u8],
    start: usize,
    count: usize,
    spell_table_start: usize,
) -> Vec<MemorizedSpells> {
    copy_transmute_buff::<SpellMemorizationInfo>(buffer, start, count)
        .iter()
        .map(|spell_memorization_info| {
            let start = spell_table_start
                + size_of::<SpellMemorizationTable>()
                    * usize::try_from(spell_memorization_info.index_to_spell_table).unwrap_or(0);
            let spell_memorization_table = copy_transmute_buff::<SpellMemorizationTable>(
                buffer,
                start,
                usize::try_from(spell_memorization_info.count_of_memorizable_spell_tables)
                    .unwrap_or(0),
            );

            MemorizedSpells {
                spell_memorization_info: *spell_memorization_info,
                spell_memorization_table,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use crate::{creature::Creature, model::Model};

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
        let memorized_spells = creature.memorized_spells;
        let spells = memorized_spells.first().unwrap();
        assert_eq!(
            spells.spell_memorization_info,
            SpellMemorizationInfo {
                spell_level: 0,
                number_of_spells_memorizable: 4,
                number_of_spells_memorizable_after_effects: 4,
                spell_type: 0,
                index_to_spell_table: 0,
                count_of_memorizable_spell_tables: 4,
            }
        );
        assert_eq!(
            spells.spell_memorization_table,
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
            ]
        );
    }
}
