use crate::simulation::state::world::grid::direction_set::DirectionSet;

#[derive(Clone)]
pub struct BlockData {
    pub exposure_set: DirectionSet,
}

impl BlockData {
    pub fn new() -> Self {
        Self {
            exposure_set: DirectionSet::ALL,
        }
    }
}
