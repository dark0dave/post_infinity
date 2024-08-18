# Post Infinity

This tool can read infinity engine binary game files for BGEE and BG2EE. Similar to NearInfinity. It prints the binary game data files.

### Supported files

Taken from [infinity file formats](https://gibberlings3.github.io/iesdp/file_formats/index.htm) on the iesdp.

- [area](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/are_v1.htm)
- [biff](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bif_v1.htm)
- [character](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/chr_v2.htm)
- [creature](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm)
- [dialogue](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm)
- [effect_v1](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v1.htm#effv1_Header)
- [effect_v2](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v2.htm)
- [game](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/gam_v2.0.htm)
- [ids](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/ids.htm)
- [item_table](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_Item)
- [item](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/itm_v1.htm)
- [key](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/key_v1.htm)
- [save](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sav_v1.htm)
- [spell_table](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_KnownSpell)
- [spell](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/spl_v1.htm)
- [store](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm)
- [tlk](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm)
- [twoda](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/2da.htm)
- [world_map](https://gibberlings3.github.io/iesdp/file_formats/ie_formats/wmap_v1.htm#wmapv1_0_Header)

### Other Supported files

- .bio files - Text format Baldur's Gate 2 only, Character biography
- embedded tileset files - Binary data for tilesets in maps can be viewed

## Dependencies

[Rust](https://www.rust-lang.org/tools/install)

## How to run me

Ensure rust is installed via rustup

```sh
cargo run <path to chitin.key> / or target file
```

Example:

[example.webm](https://github.com/dark0dave/post_infinity/assets/52840419/5d5974ad-83fe-4abd-b5fd-18c1b94ff49a)

```sh
post_infinity models/fixtures/gate1.spl
{
  "signature": "SPL ",
  "version": "V1  ",
  "unidentified_spell_name": 14260,
  "identified_spell_name": 9999999,
  "completion_sound": "CAS_M03\u0000",
  "flags": 0,
  "spell_type": 1,
  "exclusion_flags": 0,
  "casting_graphics": 18,
  "min_level": 0,
  "primary_spell_school": 2,
  "min_strength": 0,
  "secondary_spell_school": 6,
  "min_strength_bonus": 0,
  "kit_usability_1": 0,
  "min_intelligence": 0,
  "kit_usability_2": 0,
  "min_dexterity": 0,
  "kit_usability_3": 0,
  "min_wisdom": 0,
  "kit_usability_4": 0,
  "min_constitution": 0,
  "min_charisma": 0,
  "spell_level": 9,
  "max_stackable": 1,
  "spell_book_icon": "SPWI905C",
  "lore": 0,
  "ground_icon": "\u0000\u0000rb\u0000\u0000Un",
  "base_weight": 0,
  "spell_description_generic": 4294967295,
  "spell_description_identified": 9999999,
  "description_icon": "",
  "enchantment": 0,
  "offset_to_extended_headers": 114,
  "count_of_extended_headers": 1,
  "offset_to_feature_block_table": 154,
  "offset_to_casting_feature_blocks": 0,
  "count_of_casting_feature_blocks": 0,
  "extended_headers": [
    {
      "spell_form": 1,
      "friendly": 0,
      "location": 2,
      "memorised_icon": "SPWI905B",
      "target_type": 4,
      "target_count": 0,
      "range": 25,
      "level_required": 1,
      "casting_time": 4,
      "times_per_day": 0,
      "dice_sides": 6,
      "dice_thrown": 0,
      "enchanted": 0,
      "damage_type": 1,
      "count_of_feature_blocks": 1,
      "offset_to_feature_blocks": 0,
      "charges": 1,
      "charge_depletion_behaviour": 1,
      "projectile": 1
    }
  ],
  "equipping_feature_blocks": [
    {
      "opcode_number": 177,
      "target_type": 1,
      "power": 9,
      "parameter_1": 0,
      "parameter_2": 2,
      "timing_mode": 0,
      "dispel_resistance": 2,
      "duration": 100000,
      "probability_1": 39,
      "probability_2": 0,
      "resource": "balorsu\u0000",
      "dice_thrown_max_level": 0,
      "dice_sides_min_level": 0,
      "saving_throw_type": [
        0,
        0,
        0,
        0
      ],
      "saving_throw_bonus": 0,
      "stacking_id": 0
    }
  ]
}
```

## How to build

```sh
cargo build --release
```

## Performance

It takes about 0.07 ish to read all the supported files in an unmodded bg1ee game, into memory, without parsing them.

```sh
time cargo run -- <path to bgee dir>/chitin.key
0.07s user 0.58s system 99% cpu 0.655 total
```

you can analyze performance:

```sh
cargo isntall flamegraph
flamegraph -- target/release/post_infinity <path to chitin file>
```

