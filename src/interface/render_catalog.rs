pub mod block_entry;

use strum::IntoEnumIterator;

use crate::{
    interface::render_catalog::block_entry::BlockEntry, simulation::state::world::block::BlockKind,
};
use std::collections::HashMap;

pub struct RenderCatalog {
    pub block_map: HashMap<BlockKind, BlockEntry>,
}

impl RenderCatalog {
    pub fn new() -> Self {
        let block_map = Self::setup_block_map();

        Self { block_map }
    }

    fn setup_block_map() -> HashMap<BlockKind, BlockEntry> {
        let block_map = HashMap::from([
            (
                BlockKind::Engraved1,
                BlockEntry {
                    north_face: "engraved 1",
                    west_face: "engraved 1",
                    south_face: "engraved 1",
                    east_face: "engraved 1",
                    up_face: "engraved 1",
                    down_face: "engraved 1",
                },
            )
        ]);

        block_map
    }
}
