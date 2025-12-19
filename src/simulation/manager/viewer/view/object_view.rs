use crate::simulation::state::world::{grid, object};

#[derive(Clone, Copy, Debug)]
pub struct ObjectView {
    pub object_kind: object::Kind,
    pub direction: grid::Direction,
}

impl ObjectView {
    pub fn new() -> Self {
        Self {
            object_kind: object::Kind::DoorClosed,
            direction: grid::Direction::North,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
