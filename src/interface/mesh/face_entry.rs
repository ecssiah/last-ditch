use crate::simulation::state::world::{block, grid};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct FaceEntry {
    pub kind: block::Kind,
    pub direction: grid::Direction,
}

impl FaceEntry {
    pub const NONE: Self = Self {
        kind: block::Kind::None,
        direction: grid::Direction::East,
    };
}
