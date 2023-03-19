use std::rc::Rc;

use model::Model;
use resources::types::ResourceType;

use crate::{
    area::Area, bio::Biography, character::ExpandedCharacter, creature::Creature, dialog::Dialog,
    effect_v2::EffectV2, game::Game, ids::Ids, item::Item, spell::Spell, store::Store,
    twoda::TwoDA, world_map::WorldMap,
};

pub mod area;
pub mod biff;
pub mod bio;
pub mod character;
pub mod common;
pub mod creature;
pub mod dialog;
pub mod effect_v1;
pub mod effect_v2;
pub mod game;
pub mod ids;
pub mod item;
pub mod item_table;
pub mod key;
pub mod model;
pub mod resources;
pub mod spell;
pub mod spell_table;
pub mod store;
pub mod tlk;
pub mod twoda;
pub mod world_map;

pub fn from_buffer(buffer: &[u8], resource_type: ResourceType) -> Option<Rc<dyn Model>> {
    match resource_type {
        // I am skipping image files
        ResourceType::FileTypeBmp => None,
        ResourceType::FileTypeMve => todo!(),
        // I am skipping music files
        ResourceType::FileTypeWav => None,
        // Skipping play back sounds
        ResourceType::FileTypeWfx => None,
        // Skipping
        ResourceType::FileTypePlt => None,
        // I am skipping image files
        ResourceType::FileTypeBam => None,
        // I am skipping texture files
        ResourceType::FileTypeWed => None,
        // I am skipping GUI defs
        ResourceType::FileTypeChu => None,
        ResourceType::FileTypeTi => todo!(),
        // I am skipping compress graphic files
        ResourceType::FileTypeMos => None,
        ResourceType::FileTypeItm => Some(Item::create_as_rc(buffer)),
        ResourceType::FileTypeSpl => Some(Spell::create_as_rc(buffer)),
        // I am ignoring scripting files
        ResourceType::FileTypeBcs => None,
        ResourceType::FileTypeIds => Some(Ids::create_as_rc(buffer)),
        ResourceType::FileTypeCre => Some(Creature::create_as_rc(buffer)),
        ResourceType::FileTypeAre => Some(Area::create_as_rc(buffer)),
        ResourceType::FileTypeDlg => Some(Dialog::create_as_rc(buffer)),
        ResourceType::FileType2da => Some(TwoDA::create_as_rc(buffer)),
        // Game is a slow resource
        ResourceType::FileTypeGam => Some(Game::create_as_rc(buffer)),
        ResourceType::FileTypeSto => Some(Store::create_as_rc(buffer)),
        ResourceType::FileTypeWmap => Some(WorldMap::create_as_rc(buffer)),
        ResourceType::FileTypeEff => Some(EffectV2::create_as_rc(buffer)),
        ResourceType::FileTypeBs => todo!(),
        ResourceType::FileTypeChr => Some(ExpandedCharacter::create_as_rc(buffer)),
        // I am skipping spell casting graphics
        ResourceType::FileTypeVvc => None,
        // Skip visual effects
        ResourceType::FileTypeVef => None,
        // I am skipping projectiles
        ResourceType::FileTypePro => None,
        ResourceType::FileTypeBio => Some(Biography::create_as_rc(buffer)),
        ResourceType::FileTypeWbm => None,
        ResourceType::FileTypeFnt => None,
        ResourceType::FileTypeGui => None,
        ResourceType::FileTypeSql => None,
        // Skipping graphic data
        ResourceType::FileTypePvrz => None,
        ResourceType::FileTypeGlsl => None,
        ResourceType::FileTypeMenu => None,
        ResourceType::FileTypeTtf => None,
        ResourceType::FileTypePng => todo!(),
        ResourceType::FileTypeBah => todo!(),
        ResourceType::FileTypeIni => None,
        // Skipping sounds/ out of dialog text
        ResourceType::FileTypeSrc => None,
        ResourceType::NotFound => None,
    }
}
