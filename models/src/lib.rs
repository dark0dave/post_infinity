use std::error::Error;
use std::rc::Rc;

use bam::Bam;
use common::types::ResourceType;
use model::Model;
use tileset::Tileset;

use crate::{
    area::Area, bio::Biography, character::ExpandedCharacter, creature::Creature,
    dialogue::Dialogue, effect_v2::EffectV2, game::Game, ids::Ids, item::Item, save::Save,
    spell::Spell, store::Store, twoda::TwoDA, world_map::WorldMap,
};

pub mod area;
pub mod bam;
pub mod biff;
pub mod bio;
pub mod character;
pub mod common;
pub mod creature;
pub mod dialogue;
pub mod effect_v1;
pub mod effect_v2;
pub mod game;
pub mod ids;
pub mod item;
pub mod item_table;
pub mod key;
pub mod model;
pub mod save;
pub mod spell;
pub mod spell_table;
pub mod store;
pub mod tileset;
pub mod tlk;
pub mod twoda;
pub mod world_map;

pub type IEModel = Rc<dyn Model>;

const NOT_IMPLIMENTED: &str = "Not implimented yet";

pub fn from_buffer(buffer: &[u8], resource_type: ResourceType) -> Result<IEModel, Box<dyn Error>> {
    match resource_type {
        // I am skipping image files
        ResourceType::FileTypeBmp => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeMve => Err(NOT_IMPLIMENTED.into()),
        // I am skipping music files
        ResourceType::FileTypeWav => Err(NOT_IMPLIMENTED.into()),
        // Skipping play back sounds
        ResourceType::FileTypeWfx => Err(NOT_IMPLIMENTED.into()),
        // Skipping
        ResourceType::FileTypePlt => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeBam => Err(NOT_IMPLIMENTED.into()),
        // I am skipping texture files
        ResourceType::FileTypeWed => Err(NOT_IMPLIMENTED.into()),
        // I am skipping GUI defs
        ResourceType::FileTypeChu => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeTi => Ok(Rc::new(Tileset::new(buffer))),
        // I am skipping compress graphic files
        ResourceType::FileTypeMos => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeItm => Ok(Rc::new(Item::new(buffer))),
        ResourceType::FileTypeSpl => Ok(Rc::new(Spell::new(buffer))),
        // I am ignoring scripting files
        ResourceType::FileTypeBcs => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeIds => Ok(Rc::new(Ids::new(buffer))),
        ResourceType::FileTypeCre => Ok(Rc::new(Creature::new(buffer))),
        ResourceType::FileTypeAre => Ok(Rc::new(Area::new(buffer))),
        ResourceType::FileTypeDlg => Ok(Rc::new(Dialogue::new(buffer))),
        ResourceType::FileType2da => Ok(Rc::new(TwoDA::new(buffer))),
        // Game is a slow resource
        ResourceType::FileTypeGam => Ok(Rc::new(Game::new(buffer))),
        ResourceType::FileTypeSto => Ok(Rc::new(Store::new(buffer))),
        ResourceType::FileTypeWmap => Ok(Rc::new(WorldMap::new(buffer))),
        ResourceType::FileTypeEff => Ok(Rc::new(EffectV2::new(buffer))),
        ResourceType::FileTypeBs => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeChr => Ok(Rc::new(ExpandedCharacter::new(buffer))),
        // I am skipping spell casting graphics
        ResourceType::FileTypeVvc => Err(NOT_IMPLIMENTED.into()),
        // Skip visual effects
        ResourceType::FileTypeVef => Err(NOT_IMPLIMENTED.into()),
        // I am skipping projectiles
        ResourceType::FileTypePro => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeBio => Ok(Rc::new(Biography::new(buffer))),
        ResourceType::FileTypeWbm => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeFnt => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeGui => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeSql => Err(NOT_IMPLIMENTED.into()),
        // Skipping graphic data
        ResourceType::FileTypePvrz => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeGlsl => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeTlk => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeMenu => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeTtf => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypePng => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeBah => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeIni => Err(NOT_IMPLIMENTED.into()),
        // Skipping sounds/ out of dialog text
        ResourceType::FileTypeSrc => Err(NOT_IMPLIMENTED.into()),
        ResourceType::NotFound => Err(NOT_IMPLIMENTED.into()),
        // Our invented file types:
        ResourceType::FileTypeSave => Ok(Rc::new(Save::new(buffer))),
        _ => Err(NOT_IMPLIMENTED.into()),
    }
}

pub fn from_json(buffer: &[u8], resource_type: ResourceType) -> Result<Vec<u8>, Box<dyn Error>> {
    match resource_type {
        // I am skipping image files
        ResourceType::FileTypeBmp => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeMve => Err(NOT_IMPLIMENTED.into()),
        // I am skipping music files
        ResourceType::FileTypeWav => Err(NOT_IMPLIMENTED.into()),
        // Skipping play back sounds
        ResourceType::FileTypeWfx => Err(NOT_IMPLIMENTED.into()),
        // Skipping
        ResourceType::FileTypePlt => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeBam => Ok(serde_json::from_slice::<Bam>(buffer)?.to_bytes()),
        // I am skipping texture files
        ResourceType::FileTypeWed => Err(NOT_IMPLIMENTED.into()),
        // I am skipping GUI defs
        ResourceType::FileTypeChu => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeTi => Err(NOT_IMPLIMENTED.into()),
        // I am skipping compress graphic files
        ResourceType::FileTypeMos => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeItm => Ok(serde_json::from_slice::<Item>(buffer)?.to_bytes()),
        ResourceType::FileTypeSpl => Ok(serde_json::from_slice::<Spell>(buffer)?.to_bytes()),
        // I am ignoring scripting files
        ResourceType::FileTypeBcs => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeIds => Ok(serde_json::from_slice::<Ids>(buffer)?.to_bytes()),
        ResourceType::FileTypeCre => Ok(serde_json::from_slice::<Creature>(buffer)?.to_bytes()),
        ResourceType::FileTypeAre => Ok(serde_json::from_slice::<Area>(buffer)?.to_bytes()),
        ResourceType::FileTypeDlg => Ok(serde_json::from_slice::<Dialogue>(buffer)?.to_bytes()),
        ResourceType::FileType2da => Ok(serde_json::from_slice::<TwoDA>(buffer)?.to_bytes()),
        // Game is a slow resource
        ResourceType::FileTypeGam => Ok(serde_json::from_slice::<Game>(buffer)?.to_bytes()),
        ResourceType::FileTypeSto => Ok(serde_json::from_slice::<Store>(buffer)?.to_bytes()),
        ResourceType::FileTypeWmap => Ok(serde_json::from_slice::<WorldMap>(buffer)?.to_bytes()),
        ResourceType::FileTypeEff => Ok(serde_json::from_slice::<EffectV2>(buffer)?.to_bytes()),
        ResourceType::FileTypeBs => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeChr => {
            Ok(serde_json::from_slice::<ExpandedCharacter>(buffer)?.to_bytes())
        }
        // I am skipping spell casting graphics
        ResourceType::FileTypeVvc => Err(NOT_IMPLIMENTED.into()),
        // Skip visual effects
        ResourceType::FileTypeVef => Err(NOT_IMPLIMENTED.into()),
        // I am skipping projectiles
        ResourceType::FileTypePro => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeBio => Ok(serde_json::from_slice::<Biography>(buffer)?.to_bytes()),
        ResourceType::FileTypeWbm => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeFnt => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeGui => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeSql => Err(NOT_IMPLIMENTED.into()),
        // Skipping graphic data
        ResourceType::FileTypePvrz => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeGlsl => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeTlk => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeMenu => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeTtf => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypePng => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeBah => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeIni => Err(NOT_IMPLIMENTED.into()),
        // Skipping sounds/ out of dialog text
        ResourceType::FileTypeSrc => Err(NOT_IMPLIMENTED.into()),
        ResourceType::NotFound => Err(NOT_IMPLIMENTED.into()),
        // Our invented file types:
        ResourceType::FileTypeSave => Ok(serde_json::from_slice::<Save>(buffer)?.to_bytes()),
        _ => Err(NOT_IMPLIMENTED.into()),
    }
}
