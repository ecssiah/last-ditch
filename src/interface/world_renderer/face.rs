use crate::simulation::state::world::{block, grid};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Face {
    pub block_kind: block::Kind,
    pub direction: grid::Direction,
}

impl Face {
    pub fn new() -> Self {
        Self {
            block_kind: block::Kind::Caution,
            direction: grid::Direction::East,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
