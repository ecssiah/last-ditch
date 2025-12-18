use crate::simulation::{constants::ID_NULL, state::world::{grid, object}};
use ultraviolet::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct ObjectView {
    pub object_id: u64,
    pub object_kind: object::Kind,
    pub grid_position: IVec3,
    pub direction: grid::Direction,
}

impl ObjectView {
    pub fn new() -> Self {
        Self {
            object_id: ID_NULL,
            object_kind: object::Kind::DoorClosed,
            grid_position: IVec3::default(),
            direction: grid::Direction::North,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
