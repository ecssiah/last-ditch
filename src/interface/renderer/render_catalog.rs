pub mod block_entry;
pub mod block_key;
pub mod person_entry;
pub mod person_key;

use crate::{
    interface::renderer::render_catalog::{block_key::BlockKey, person_key::PersonKey},
    simulation::state::{
        population::identity::ethnicity::skin_tone::SkinTone, world::block::block_kind::BlockKind,
    },
};
use std::collections::HashMap;

pub struct RenderCatalog {
    pub block_texture_map: HashMap<BlockKey, &'static str>,
    pub block_model_map: HashMap<BlockKey, &'static str>,
    pub person_texture_map: HashMap<PersonKey, &'static str>,
    pub person_model_map: HashMap<PersonKey, &'static str>,
}

impl RenderCatalog {
    pub const BLOCK_UV_ARRAY: [[f32; 2]; 4] = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    pub fn new() -> Self {
        let block_texture_map = Self::setup_block_texture_map();
        let block_model_map = Self::setup_block_model_map();

        let person_texture_map = Self::setup_person_texture_map();
        let person_model_map = Self::setup_person_model_map();

        Self {
            block_texture_map,
            block_model_map,
            person_texture_map,
            person_model_map,
        }
    }

    fn setup_block_texture_map() -> HashMap<BlockKey, &'static str> {
        let block_texture_map = HashMap::from([
            (BlockKey::new(&BlockKind::Engraved1), "engraved 1"),
            (BlockKey::new(&BlockKind::Engraved2), "engraved 2"),
            (BlockKey::new(&BlockKind::Engraved3), "engraved 3"),
            (BlockKey::new(&BlockKind::Engraved4), "engraved 4"),
        ]);

        block_texture_map
    }

    pub fn get_block_texture_name(block_kind: &BlockKind, render_catalog: &Self) -> &'static str {
        render_catalog
            .block_texture_map
            .get(&BlockKey::new(block_kind))
            .expect("All blocks should have a texture name")
            .clone()
    }

    fn setup_block_model_map() -> HashMap<BlockKey, &'static str> {
        let block_model_map = HashMap::from([
            (BlockKey::new(&BlockKind::Engraved1), "engraved 1"),
            (BlockKey::new(&BlockKind::Engraved2), "engraved 2"),
            (BlockKey::new(&BlockKind::Engraved3), "engraved 3"),
            (BlockKey::new(&BlockKind::Engraved4), "engraved 4"),
        ]);

        block_model_map
    }

    pub fn get_block_model_name(block_kind: &BlockKind, render_catalog: &Self) -> &'static str {
        render_catalog
            .block_texture_map
            .get(&BlockKey::new(block_kind))
            .expect("All blocks should have a texture name")
            .clone()
    }

    fn setup_person_texture_map() -> HashMap<PersonKey, &'static str> {
        let person_texture_map = HashMap::from([
            (PersonKey::new(&SkinTone::Color1), "person 1"),
            (PersonKey::new(&SkinTone::Color2), "person 2"),
            (PersonKey::new(&SkinTone::Color3), "person 3"),
            (PersonKey::new(&SkinTone::Color4), "person 4"),
            (PersonKey::new(&SkinTone::Color5), "person 5"),
            (PersonKey::new(&SkinTone::Color6), "person 6"),
            (PersonKey::new(&SkinTone::Color7), "person 7"),
            (PersonKey::new(&SkinTone::Color8), "person 8"),
        ]);

        person_texture_map
    }

    pub fn get_person_texture_name(skin_tone: &SkinTone, render_catalog: &Self) -> &'static str {
        render_catalog
            .person_texture_map
            .get(&PersonKey::new(&skin_tone))
            .expect("All person skin tones should have a texture")
            .clone()
    }

    pub fn setup_person_model_map() -> HashMap<PersonKey, &'static str> {
        let person_model_map = HashMap::from([
            (PersonKey::new(&SkinTone::Color1), "person 1"),
            (PersonKey::new(&SkinTone::Color2), "person 2"),
            (PersonKey::new(&SkinTone::Color3), "person 3"),
            (PersonKey::new(&SkinTone::Color4), "person 4"),
            (PersonKey::new(&SkinTone::Color5), "person 5"),
            (PersonKey::new(&SkinTone::Color6), "person 6"),
            (PersonKey::new(&SkinTone::Color7), "person 7"),
            (PersonKey::new(&SkinTone::Color8), "person 8"),
        ]);

        person_model_map
    }

    pub fn get_person_model_name(skin_tone: &SkinTone, render_catalog: &Self) -> &'static str {
        render_catalog
            .person_model_map
            .get(&PersonKey::new(&skin_tone))
            .expect("All person skin tones should have a texture")
            .clone()
    }
}
