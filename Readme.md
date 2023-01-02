# Post Infinity

This tool can read some of the binary game files (see models/src/utils.rs for file types) for BGEE and BG2EE. Similar to NearInfinity. It prints the binary game data files.

This tool allows mod developers to quickly inspect binary files from the command line.

## Dependancies

[Rust](https://www.rust-lang.org/tools/install)

## How to run me

Ensure rust is installed via rustup

```
cargo run <path to chitin.key> / or target file
```

example
```
cargo run models/fixtures/gate1.spl
FileTypeSpl
Processed Some(
    Spell {
        header: SpellHeader {
            signature: "SPL ",
            version: "V1  ",
            unidentified_spell_name: [
                -76,
                55,
                0,
                0,
            ],
            identified_spell_name: [
                127,
                -106,
                -104,
                0,
            ],
            completion_sound: [
                67,
                65,
                83,
                95,
                77,
                48,
                51,
                0,
            ],
            flags: 0,
            spell_type: 1,
            exclusion_flags: 0,
            casting_graphics: [
                18,
                0,
            ],
            min_level: 0,
            primary_spell_school: 2,
            min_strength: 0,
            secondary_spell_school: 6,
            min_strenth_bonus: 0,
            kit_usability_1: 0,
            min_intelligence: 0,
            kit_usability_2: 0,
            min_dexterity: 0,
            kit_usability_3: 0,
            min_wisdom: 0,
            kit_usability_4: 0,
            min_constitution: 0,
            min_charisma: 0,
            spell_level: 9,
            max_stackable: 1,
            spellbook_icon: [
                83,
                80,
                87,
                73,
                57,
                48,
                53,
                67,
            ],
            lore: 0,
            ground_icon: [
                0,
                0,
                114,
                98,
                0,
                0,
                85,
                110,
            ],
            base_weight: 0,
            spell_description_generic: [
                -1,
                -1,
                -1,
                -1,
            ],
            spell_description_identified: [
                127,
                -106,
                -104,
                0,
            ],
            description_icon: [
                0,
                0,
                0,
                104,
                -122,
                64,
                0,
                5,
            ],
            enchantment: 0,
            offset_to_extended_headers: 114,
            count_of_extended_headers: 1,
            offset_to_feature_block_table: 154,
            offset_to_casting_feature_blocks: 0,
            count_of_casting_feature_blocks: 0,
        },
        extended_headers: [
            SpellExtendedHeader {
                spell_form: 1,
                freindly: 0,
                location: 2,
                memorised_icon: [
                    83,
                    80,
                    87,
                    73,
                    57,
                    48,
                    53,
                    66,
                ],
                target_type: 4,
                target_count: 0,
                range: 25,
                level_required: 1,
                casting_time: 4,
                times_per_day: 0,
                dice_sides: 6,
                dice_thrown: 0,
                enchanted: 0,
                damage_type: 1,
                count_of_feature_blocks: 1,
                offset_to_feature_blocks: 0,
                charges: 1,
                projectile: 1,
            },
        ],
        equiping_feature_blocks: [],
    },
)
```

## How to build

```
cargo build --release
```
