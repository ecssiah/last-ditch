use crate::simulation::state::world::{
    block::BlockKind,
    grid::{self, Direction},
};

#[derive(Clone, PartialEq)]
pub struct SectorFace {
    pub block_kind: BlockKind,
    pub direction: Direction,
}

impl SectorFace {
    pub fn new() -> Self {
        Self {
            block_kind: BlockKind::Caution1,
            direction: grid::Direction::East,
        }
    }
}

impl Default for SectorFace {
    fn default() -> Self {
        Self::new()
    }
}
