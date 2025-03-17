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

pub fn from_buffer(buffer: &[u8], resource_type: ResourceType) -> Option<Rc<dyn Model>> {
    match resource_type {
        // I am skipping image files
        ResourceType::FileTypeBmp => None,
        ResourceType::FileTypeMve => None,
        // I am skipping music files
        ResourceType::FileTypeWav => None,
        // Skipping play back sounds
        ResourceType::FileTypeWfx => None,
        // Skipping
        ResourceType::FileTypePlt => None,
        ResourceType::FileTypeBam => None,
        // I am skipping texture files
        ResourceType::FileTypeWed => None,
        // I am skipping GUI defs
        ResourceType::FileTypeChu => None,
        ResourceType::FileTypeTi => Some(Rc::new(Tileset::new(buffer))),
        // I am skipping compress graphic files
        ResourceType::FileTypeMos => None,
        ResourceType::FileTypeItm => Some(Rc::new(Item::new(buffer))),
        ResourceType::FileTypeSpl => Some(Rc::new(Spell::new(buffer))),
        // I am ignoring scripting files
        ResourceType::FileTypeBcs => None,
        ResourceType::FileTypeIds => Some(Rc::new(Ids::new(buffer))),
        ResourceType::FileTypeCre => Some(Rc::new(Creature::new(buffer))),
        ResourceType::FileTypeAre => Some(Rc::new(Area::new(buffer))),
        ResourceType::FileTypeDlg => Some(Rc::new(Dialogue::new(buffer))),
        ResourceType::FileType2da => Some(Rc::new(TwoDA::new(buffer))),
        // Game is a slow resource
        ResourceType::FileTypeGam => Some(Rc::new(Game::new(buffer))),
        ResourceType::FileTypeSto => Some(Rc::new(Store::new(buffer))),
        ResourceType::FileTypeWmap => Some(Rc::new(WorldMap::new(buffer))),
        ResourceType::FileTypeEff => Some(Rc::new(EffectV2::new(buffer))),
        ResourceType::FileTypeBs => None,
        ResourceType::FileTypeChr => Some(Rc::new(ExpandedCharacter::new(buffer))),
        // I am skipping spell casting graphics
        ResourceType::FileTypeVvc => None,
        // Skip visual effects
        ResourceType::FileTypeVef => None,
        // I am skipping projectiles
        ResourceType::FileTypePro => None,
        ResourceType::FileTypeBio => Some(Rc::new(Biography::new(buffer))),
        ResourceType::FileTypeWbm => None,
        ResourceType::FileTypeFnt => None,
        ResourceType::FileTypeGui => None,
        ResourceType::FileTypeSql => None,
        // Skipping graphic data
        ResourceType::FileTypePvrz => None,
        ResourceType::FileTypeGlsl => None,
        ResourceType::FileTypeTlk => None,
        ResourceType::FileTypeMenu => None,
        ResourceType::FileTypeTtf => None,
        ResourceType::FileTypePng => None,
        ResourceType::FileTypeBah => None,
        ResourceType::FileTypeIni => None,
        // Skipping sounds/ out of dialog text
        ResourceType::FileTypeSrc => None,
        ResourceType::NotFound => None,
        // Our invented file types:
        ResourceType::FileTypeSave => Some(Rc::new(Save::new(buffer))),
        _ => None,
    }
}

pub fn from_json(buffer: &[u8], resource_type: ResourceType) -> Result<Vec<u8>, Box<dyn Error>> {
    let model: Rc<dyn Model> = match resource_type {
        // I am skipping image files
        ResourceType::FileTypeBmp => return Err("Not implimented yet".into()),
        ResourceType::FileTypeMve => return Err("Not implimented yet".into()),
        // I am skipping music files
        ResourceType::FileTypeWav => return Err("Not implimented yet".into()),
        // Skipping play back sounds
        ResourceType::FileTypeWfx => return Err("Not implimented yet".into()),
        // Skipping
        ResourceType::FileTypePlt => return Err("Not implimented yet".into()),
        ResourceType::FileTypeBam => Rc::new(serde_json::from_slice::<Bam>(buffer)?),
        // I am skipping texture files
        ResourceType::FileTypeWed => return Err("Not implimented yet".into()),
        // I am skipping GUI defs
        ResourceType::FileTypeChu => return Err("Not implimented yet".into()),
        ResourceType::FileTypeTi => return Err("Not implimented yet".into()),
        // I am skipping compress graphic files
        ResourceType::FileTypeMos => return Err("Not implimented yet".into()),
        ResourceType::FileTypeItm => Rc::new(serde_json::from_slice::<Item>(buffer)?),
        ResourceType::FileTypeSpl => Rc::new(serde_json::from_slice::<Spell>(buffer)?),
        // I am ignoring scripting files
        ResourceType::FileTypeBcs => return Err("Not implimented yet".into()),
        ResourceType::FileTypeIds => Rc::new(serde_json::from_slice::<Ids>(buffer)?),
        ResourceType::FileTypeCre => Rc::new(serde_json::from_slice::<Creature>(buffer)?),
        ResourceType::FileTypeAre => Rc::new(serde_json::from_slice::<Area>(buffer)?),
        ResourceType::FileTypeDlg => Rc::new(serde_json::from_slice::<Dialogue>(buffer)?),
        ResourceType::FileType2da => Rc::new(serde_json::from_slice::<TwoDA>(buffer)?),
        // Game is a slow resource
        ResourceType::FileTypeGam => Rc::new(serde_json::from_slice::<Game>(buffer)?),
        ResourceType::FileTypeSto => Rc::new(serde_json::from_slice::<Store>(buffer)?),
        ResourceType::FileTypeWmap => Rc::new(serde_json::from_slice::<WorldMap>(buffer)?),
        ResourceType::FileTypeEff => Rc::new(serde_json::from_slice::<EffectV2>(buffer)?),
        ResourceType::FileTypeBs => return Err("Not implimented yet".into()),
        ResourceType::FileTypeChr => Rc::new(serde_json::from_slice::<ExpandedCharacter>(buffer)?),
        // I am skipping spell casting graphics
        ResourceType::FileTypeVvc => return Err("Not implimented yet".into()),
        // Skip visual effects
        ResourceType::FileTypeVef => return Err("Not implimented yet".into()),
        // I am skipping projectiles
        ResourceType::FileTypePro => return Err("Not implimented yet".into()),
        ResourceType::FileTypeBio => Rc::new(serde_json::from_slice::<Biography>(buffer)?),
        ResourceType::FileTypeWbm => return Err("Not implimented yet".into()),
        ResourceType::FileTypeFnt => return Err("Not implimented yet".into()),
        ResourceType::FileTypeGui => return Err("Not implimented yet".into()),
        ResourceType::FileTypeSql => return Err("Not implimented yet".into()),
        // Skipping graphic data
        ResourceType::FileTypePvrz => return Err("Not implimented yet".into()),
        ResourceType::FileTypeGlsl => return Err("Not implimented yet".into()),
        ResourceType::FileTypeTlk => return Err("Not implimented yet".into()),
        ResourceType::FileTypeMenu => return Err("Not implimented yet".into()),
        ResourceType::FileTypeTtf => return Err("Not implimented yet".into()),
        ResourceType::FileTypePng => return Err("Not implimented yet".into()),
        ResourceType::FileTypeBah => return Err("Not implimented yet".into()),
        ResourceType::FileTypeIni => return Err("Not implimented yet".into()),
        // Skipping sounds/ out of dialog text
        ResourceType::FileTypeSrc => return Err("Not implimented yet".into()),
        ResourceType::NotFound => return Err("Not implimented yet".into()),
        // Our invented file types:
        ResourceType::FileTypeSave => Rc::new(serde_json::from_slice::<Save>(buffer)?),
        _ => return Err("Not implimented yet".into()),
    };
    Ok(model.to_bytes())
}
