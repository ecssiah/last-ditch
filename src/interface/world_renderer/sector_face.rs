use crate::simulation::state::world::{block, grid};

#[derive(Clone, PartialEq)]
pub struct SectorFace {
    pub block_kind: block::Kind,
    pub direction: grid::Direction,
}

impl SectorFace {
    pub fn new() -> Self {
        Self {
            block_kind: block::Kind::Caution,
            direction: grid::Direction::East,
        }
    }
}

impl Default for SectorFace {
    fn default() -> Self {
        Self::new()
    }
}
