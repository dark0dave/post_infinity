use std::rc::Rc;

use model::Model;
use resources::types::ResourceType;
use tlk::Lookup;

use crate::{
    area::Area, bio::Biography, character::ExpandedCharacter, creature::Creature,
    dialogue::Dialogue, effect_v2::EffectV2, game::Game, ids::Ids, item::Item, spell::Spell,
    store::Store, twoda::TwoDA, world_map::WorldMap,
};

pub mod area;
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
pub mod resources;
pub mod save;
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
        ResourceType::FileTypeDlg => Some(Dialogue::create_as_rc(buffer)),
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
        ResourceType::FileTypeTlk => Some(Lookup::create_as_rc(buffer)),
        ResourceType::FileTypeMenu => None,
        ResourceType::FileTypeTtf => None,
        ResourceType::FileTypePng => todo!(),
        ResourceType::FileTypeBah => todo!(),
        ResourceType::FileTypeIni => None,
        // Skipping sounds/ out of dialog text
        ResourceType::FileTypeSrc => None,
        ResourceType::NotFound => None,
        _ => None,
    }
}

pub fn from_json(buffer: &[u8], resource_type: ResourceType) -> Vec<u8> {
    let model: Rc<dyn Model> = match resource_type {
        // I am skipping image files
        ResourceType::FileTypeBmp => todo!(),
        ResourceType::FileTypeMve => todo!(),
        // I am skipping music files
        ResourceType::FileTypeWav => todo!(),
        // Skipping play back sounds
        ResourceType::FileTypeWfx => todo!(),
        // Skipping
        ResourceType::FileTypePlt => todo!(),
        // I am skipping image files
        ResourceType::FileTypeBam => todo!(),
        // I am skipping texture files
        ResourceType::FileTypeWed => todo!(),
        // I am skipping GUI defs
        ResourceType::FileTypeChu => todo!(),
        ResourceType::FileTypeTi => todo!(),
        // I am skipping compress graphic files
        ResourceType::FileTypeMos => todo!(),
        ResourceType::FileTypeItm => Rc::new(serde_json::from_slice::<Item>(buffer).unwrap()),
        ResourceType::FileTypeSpl => Rc::new(serde_json::from_slice::<Spell>(buffer).unwrap()),
        // I am ignoring scripting files
        ResourceType::FileTypeBcs => todo!(),
        ResourceType::FileTypeIds => Rc::new(serde_json::from_slice::<Ids>(buffer).unwrap()),
        ResourceType::FileTypeCre => Rc::new(serde_json::from_slice::<Creature>(buffer).unwrap()),
        ResourceType::FileTypeAre => Rc::new(serde_json::from_slice::<Area>(buffer).unwrap()),
        ResourceType::FileTypeDlg => Rc::new(serde_json::from_slice::<Dialogue>(buffer).unwrap()),
        ResourceType::FileType2da => Rc::new(serde_json::from_slice::<TwoDA>(buffer).unwrap()),
        // Game is a slow resource
        ResourceType::FileTypeGam => Rc::new(serde_json::from_slice::<Game>(buffer).unwrap()),
        ResourceType::FileTypeSto => Rc::new(serde_json::from_slice::<Store>(buffer).unwrap()),
        ResourceType::FileTypeWmap => Rc::new(serde_json::from_slice::<WorldMap>(buffer).unwrap()),
        ResourceType::FileTypeEff => Rc::new(serde_json::from_slice::<EffectV2>(buffer).unwrap()),
        ResourceType::FileTypeBs => todo!(),
        ResourceType::FileTypeChr => {
            Rc::new(serde_json::from_slice::<ExpandedCharacter>(buffer).unwrap())
        }
        // I am skipping spell casting graphics
        ResourceType::FileTypeVvc => todo!(),
        // Skip visual effects
        ResourceType::FileTypeVef => todo!(),
        // I am skipping projectiles
        ResourceType::FileTypePro => todo!(),
        ResourceType::FileTypeBio => Rc::new(serde_json::from_slice::<Biography>(buffer).unwrap()),
        ResourceType::FileTypeWbm => todo!(),
        ResourceType::FileTypeFnt => todo!(),
        ResourceType::FileTypeGui => todo!(),
        ResourceType::FileTypeSql => todo!(),
        // Skipping graphic data
        ResourceType::FileTypePvrz => todo!(),
        ResourceType::FileTypeGlsl => todo!(),
        ResourceType::FileTypeTlk => Rc::new(serde_json::from_slice::<Lookup>(buffer).unwrap()),
        ResourceType::FileTypeMenu => todo!(),
        ResourceType::FileTypeTtf => todo!(),
        ResourceType::FileTypePng => todo!(),
        ResourceType::FileTypeBah => todo!(),
        ResourceType::FileTypeIni => todo!(),
        // Skipping sounds/ out of dialog text
        ResourceType::FileTypeSrc => todo!(),
        ResourceType::NotFound => todo!(),
        _ => todo!(),
    };
    model.to_bytes()
}
