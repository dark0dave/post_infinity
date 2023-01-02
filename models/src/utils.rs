use std::{
    mem::{size_of, ManuallyDrop},
    ptr,
    rc::Rc,
};

use crate::{
    area::Area, bio::Biography, character::ExpandedCharacter,
    common::varriable_char_array::VarriableCharArray, creature::Creature, dialog::Dialog,
    effect_v2::EffectV2, game::Game, ids::Ids, item::Item, model::Model, spell::Spell,
    store::Store, twoda::TwoDA, world_map::WorldMap,
};

use super::{resources::types::ResourceType, resources::types::ResourceType::*};

pub fn from_buffer(buffer: &[u8], resource_type: ResourceType) -> Option<Rc<dyn Model>> {
    println!("{:#?}", resource_type);
    match resource_type {
        // I am skipping image files
        FileTypeBmp => None,
        FileTypeMve => todo!(),
        // I am skipping music files
        FileTypeWav => None,
        // Skipping play back sounds
        FileTypeWfx => None,
        // Skipping
        FileTypePlt => None,
        // I am skipping image files
        FileTypeBam => None,
        // I am skipping texture files
        FileTypeWed => None,
        // I am skipping GUI defs
        FileTypeChu => None,
        FileTypeTi => todo!(),
        // I am skipping compress graphic files
        FileTypeMos => None,
        FileTypeItm => Some(Item::create_as_rc(buffer)),
        FileTypeSpl => Some(Spell::create_as_rc(buffer)),
        // I am ignoring scripting files (Willie hears ya and willie don't care)
        FileTypeBcs => None,
        FileTypeIds => Some(Ids::create_as_rc(buffer)),
        FileTypeCre => Some(Creature::create_as_rc(buffer)),
        FileTypeAre => Some(Area::create_as_rc(buffer)),
        FileTypeDlg => Some(Dialog::create_as_rc(buffer)),
        FileType2da => Some(TwoDA::create_as_rc(buffer)),
        FileTypeGam => Some(Game::create_as_rc(buffer)),
        FileTypeSto => Some(Store::create_as_rc(buffer)),
        FileTypeWmap => Some(WorldMap::create_as_rc(buffer)),
        FileTypeEff => Some(EffectV2::create_as_rc(buffer)),
        FileTypeBs => todo!(),
        FileTypeChr => Some(ExpandedCharacter::create_as_rc(buffer)),
        // I am skipping spell casting graphics
        FileTypeVvc => None,
        // Skip visual effects
        FileTypeVef => None,
        // I am skipping projectiles
        FileTypePro => None,
        FileTypeBio => Some(Biography::create_as_rc(buffer)),
        FileTypeWbm => None,
        FileTypeFnt => None,
        FileTypeGui => None,
        FileTypeSql => None,
        // Skipping graphic data
        FileTypePvrz => None,
        FileTypeGlsl => None,
        FileTypeMenu => None,
        FileTypeTtf => None,
        FileTypePng => todo!(),
        FileTypeBah => todo!(),
        FileTypeIni => None,
        // Skipping sounds/ out of dialog text
        FileTypeSrc => None,
        NotFound => None,
    }
}

pub fn copy_buff_to_struct<T>(buffer: &[u8], start: usize) -> T {
    let end: usize = start + size_of::<T>();
    if let Some(buff) = buffer.get(start..end) {
        return unsafe { std::ptr::read(buff.as_ptr() as *const _) };
    }
    panic!("Could not extract buffer into struct")
}

pub fn copy_transmute_buff<T>(buffer: &[u8], start: usize, count: usize) -> Vec<T> {
    let end: usize = start + size_of::<T>() * count;
    if let Some(buff) = buffer.get(start..end) {
        let (head, aligned, tail) = unsafe { buff.align_to::<T>() };
        assert!(head.is_empty(), "Data was not aligned");
        assert!(tail.is_empty(), "Data was not aligned");
        assert!(aligned.len() == count, "Data was not aligned");

        let v: Vec<T> = Vec::with_capacity(count);
        let mut v = ManuallyDrop::new(v);
        let ptr: *mut T = v.as_mut_ptr();

        unsafe {
            for (counter, t) in aligned.iter().enumerate() {
                let tmp = ptr::read(t);
                ptr::write(ptr.add(counter), tmp);
            }
            Vec::from_raw_parts(ptr, count, size_of::<T>() * count)
        }
    } else {
        vec![]
    }
}

const CARRAGE_RETURN: u8 = 0xD;
const NEW_LINE: u8 = 0xA;

pub fn row_parser(buffer: &[u8], row_start: usize) -> (Vec<VarriableCharArray>, usize) {
    if let Some(end) = buffer
        .get(row_start..)
        .unwrap_or_default()
        .iter()
        .position(|&byte| byte == CARRAGE_RETURN || byte == NEW_LINE)
    {
        let row_end = row_start + end;
        let row_buff = buffer.get(row_start..row_end).unwrap_or_default();
        let out = row_buff
            .split(|num| num.is_ascii_whitespace())
            .flat_map(|buff| {
                if buff.is_empty() {
                    return None;
                }
                Some(VarriableCharArray(buff.to_vec()))
            })
            .collect();

        // Row end to end of line (This should only ever be run twice)
        return (out, row_end + set_to_end_of_line(row_end, buffer));
    }
    (vec![], row_start)
}

fn set_to_end_of_line(row_end: usize, buffer: &[u8]) -> usize {
    match buffer.get(row_end) {
        Some(&CARRAGE_RETURN) => 1 + set_to_end_of_line(row_end + 1, buffer),
        Some(&NEW_LINE) => 1 + set_to_end_of_line(row_end + 1, buffer),
        _ => 0,
    }
}
