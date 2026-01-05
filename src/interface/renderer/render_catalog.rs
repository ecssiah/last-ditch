pub mod block_entry;
pub mod block_key;
pub mod person_entry;
pub mod person_key;

use crate::{
    interface::renderer::render_catalog::{
        block_entry::BlockEntry, block_key::BlockKey, person_entry::PersonEntry,
        person_key::PersonKey,
    },
    simulation::state::{
        population::identity::ethnicity::skin_tone::SkinTone, world::block::BlockKind,
    },
};
use std::collections::HashMap;

pub struct RenderCatalog {
    pub block_map: HashMap<BlockKey, BlockEntry>,
    pub person_map: HashMap<PersonKey, PersonEntry>,
}

impl RenderCatalog {
    pub fn new() -> Self {
        let block_map = Self::setup_block_map();
        let person_map = Self::setup_person_map();

        Self {
            block_map,
            person_map,
        }
    }

    fn setup_block_map() -> HashMap<BlockKey, BlockEntry> {
        let block_map = HashMap::from([
            (
                BlockKey::new(&BlockKind::Engraved1),
                BlockEntry::from_face("engraved 1"),
            ),
            (
                BlockKey::new(&BlockKind::Engraved2),
                BlockEntry::from_face("engraved 2"),
            ),
            (
                BlockKey::new(&BlockKind::Engraved3),
                BlockEntry::from_face("engraved 3"),
            ),
            (
                BlockKey::new(&BlockKind::Engraved4),
                BlockEntry::from_face("engraved 4"),
            ),
        ]);

        block_map
    }

    pub fn get_block_entry(block_kind: &BlockKind, render_catalog: &Self) -> BlockEntry {
        render_catalog
            .block_map
            .get(&BlockKey::new(block_kind))
            .expect("All blocks should have an entry")
            .clone()
    }

    fn setup_person_map() -> HashMap<PersonKey, PersonEntry> {
        let person_map = HashMap::from([
            (
                PersonKey::new(&SkinTone::Color1),
                PersonEntry::new("person 1"),
            ),
            (
                PersonKey::new(&SkinTone::Color2),
                PersonEntry::new("person 2"),
            ),
            (
                PersonKey::new(&SkinTone::Color3),
                PersonEntry::new("person 3"),
            ),
            (
                PersonKey::new(&SkinTone::Color4),
                PersonEntry::new("person 4"),
            ),
            (
                PersonKey::new(&SkinTone::Color5),
                PersonEntry::new("person 5"),
            ),
            (
                PersonKey::new(&SkinTone::Color6),
                PersonEntry::new("person 6"),
            ),
            (
                PersonKey::new(&SkinTone::Color7),
                PersonEntry::new("person 7"),
            ),
            (
                PersonKey::new(&SkinTone::Color8),
                PersonEntry::new("person 8"),
            ),
        ]);

        person_map
    }

    pub fn get_person_entry(skin_tone: &SkinTone, render_catalog: &Self) -> PersonEntry {
        render_catalog
            .person_map
            .get(&PersonKey::new(&skin_tone))
            .expect("All persons should have an entry")
            .clone()
    }
}
