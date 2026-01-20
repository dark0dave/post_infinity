use std::error::Error;

use bam::Bam;
use common::types::ResourceType;
use model::Model;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tileset::Tileset;

use crate::{
    area::Area, bio::Biography, character::ExpandedCharacter, creature::Creature,
    dialogue::Dialogue, effect_v2::EffectV2, game::Game, ids::Ids, item::Item, key::Key,
    save::Save, spell::Spell, store::Store, twoda::TwoDA, world_map::WorldMap,
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

const NOT_IMPLIMENTED: &str = "Not implimented yet";

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, bound(deserialize = "'de:'a"))]
pub enum IEModels<'a> {
    Area(Area),
    Biography(Biography),
    Creature(Creature),
    Dialogue(Dialogue),
    EffectV2(EffectV2),
    ExpandedCharacter(ExpandedCharacter),
    Game(Game),
    Ids(Ids<'a>),
    Item(Item),
    Key(Key<'a>),
    Save(Save<'a>),
    Spell(Spell),
    Store(Store),
    Tileset(Tileset<'a>),
    TwoDA(TwoDA),
    WorldMap(WorldMap),
}

impl<'a> IEModels<'a> {
    pub fn to_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        match self {
            IEModels::Area(area) => Ok(area.to_bytes()),
            IEModels::Biography(biography) => Ok(biography.to_bytes()),
            IEModels::Creature(creature) => Ok(creature.to_bytes()),
            IEModels::Dialogue(dialogue) => Ok(dialogue.to_bytes()),
            IEModels::EffectV2(effect_v2) => Ok(effect_v2.to_bytes()),
            IEModels::ExpandedCharacter(expanded_character) => Ok(expanded_character.to_bytes()),
            IEModels::Game(game) => Ok(game.to_bytes()),
            IEModels::Ids(ids) => ids.try_into(),
            IEModels::Item(item) => Ok(item.to_bytes()),
            IEModels::Key(key) => key.try_into(),
            IEModels::Save(save) => save.try_into(),
            IEModels::Spell(spell) => Ok(spell.to_bytes()),
            IEModels::Store(store) => Ok(store.to_bytes()),
            IEModels::Tileset(tileset) => tileset.try_into(),
            IEModels::TwoDA(two_da) => Ok(two_da.to_bytes()),
            IEModels::WorldMap(world_map) => Ok(world_map.to_bytes()),
        }
    }
    pub fn to_json(&self) -> Result<Value, Box<dyn Error>> {
        Ok(match self {
            IEModels::Area(area) => serde_json::to_value(area),
            IEModels::Biography(biography) => serde_json::to_value(biography),
            IEModels::Creature(creature) => serde_json::to_value(creature),
            IEModels::Dialogue(dialogue) => serde_json::to_value(dialogue),
            IEModels::EffectV2(effect_v2) => serde_json::to_value(effect_v2),
            IEModels::ExpandedCharacter(expanded_character) => {
                serde_json::to_value(expanded_character)
            }
            IEModels::Game(game) => serde_json::to_value(game),
            IEModels::Ids(ids) => serde_json::to_value(ids),
            IEModels::Item(item) => serde_json::to_value(item),
            IEModels::Key(key) => serde_json::to_value(key),
            IEModels::Save(save) => serde_json::to_value(save),
            IEModels::Spell(spell) => serde_json::to_value(spell),
            IEModels::Store(store) => serde_json::to_value(store),
            IEModels::Tileset(tileset) => serde_json::to_value(tileset),
            IEModels::TwoDA(two_da) => serde_json::to_value(two_da),
            IEModels::WorldMap(world_map) => serde_json::to_value(world_map),
        }?)
    }
}

pub fn from_buffer_with_resouce_type(
    buffer: &'_ [u8],
    resource_type: ResourceType,
) -> Result<IEModels<'_>, Box<dyn Error>> {
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
        ResourceType::FileTypeTis => Ok(IEModels::Tileset(Tileset::try_from(buffer)?)),
        // I am skipping compress graphic files
        ResourceType::FileTypeMos => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeItm => Ok(IEModels::Item(Item::new(buffer))),
        ResourceType::FileTypeSpl => Ok(IEModels::Spell(Spell::new(buffer))),
        // I am ignoring scripting files
        ResourceType::FileTypeBcs => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeIds => Ok(IEModels::Ids(Ids::try_from(buffer)?)),
        ResourceType::FileTypeCre => Ok(IEModels::Creature(Creature::new(buffer))),
        ResourceType::FileTypeAre => Ok(IEModels::Area(Area::new(buffer))),
        ResourceType::FileTypeDlg => Ok(IEModels::Dialogue(Dialogue::new(buffer))),
        ResourceType::FileType2da => Ok(IEModels::TwoDA(TwoDA::new(buffer))),
        // Game is a slow resource
        ResourceType::FileTypeGam => Ok(IEModels::Game(Game::new(buffer))),
        ResourceType::FileTypeSto => Ok(IEModels::Store(Store::new(buffer))),
        ResourceType::FileTypeWmap => Ok(IEModels::WorldMap(WorldMap::new(buffer))),
        ResourceType::FileTypeEff => Ok(IEModels::EffectV2(EffectV2::new(buffer))),
        ResourceType::FileTypeBs => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeChr => {
            Ok(IEModels::ExpandedCharacter(ExpandedCharacter::new(buffer)))
        }
        // I am skipping spell casting graphics
        ResourceType::FileTypeVvc => Err(NOT_IMPLIMENTED.into()),
        // Skip visual effects
        ResourceType::FileTypeVef => Err(NOT_IMPLIMENTED.into()),
        // I am skipping projectiles
        ResourceType::FileTypePro => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeBio => Ok(IEModels::Biography(Biography::new(buffer))),
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
        ResourceType::FileTypeSave => Ok(IEModels::Save(Save::try_from(buffer)?)),
        ResourceType::FileTypeKey => Ok(IEModels::Key(Key::try_from(buffer)?)),
        _ => Err(NOT_IMPLIMENTED.into()),
    }
}

pub fn from_buffer(buffer: &'_ [u8], resource_type: u16) -> Result<IEModels<'_>, Box<dyn Error>> {
    let resource_type: ResourceType = unsafe { std::mem::transmute(resource_type) };
    from_buffer_with_resouce_type(buffer, resource_type)
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
        ResourceType::FileTypeTis => Err(NOT_IMPLIMENTED.into()),
        // I am skipping compress graphic files
        ResourceType::FileTypeMos => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeItm => Ok(serde_json::from_slice::<Item>(buffer)?.to_bytes()),
        ResourceType::FileTypeSpl => Ok(serde_json::from_slice::<Spell>(buffer)?.to_bytes()),
        // I am ignoring scripting files
        ResourceType::FileTypeBcs => Err(NOT_IMPLIMENTED.into()),
        ResourceType::FileTypeIds => {
            TryInto::<Vec<u8>>::try_into(serde_json::from_slice::<Ids>(buffer)?)
        }
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
        ResourceType::FileTypeSave => {
            TryInto::<Vec<u8>>::try_into(serde_json::from_slice::<Save>(buffer)?)
        }
        _ => Err(NOT_IMPLIMENTED.into()),
    }
}
