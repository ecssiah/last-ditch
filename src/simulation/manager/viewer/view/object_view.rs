use crate::simulation::state::world::{grid, object};
use ultraviolet::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct ObjectView {
    pub object_id: u64,
    pub object_kind: object::Kind,
    pub grid_position: IVec3,
    pub direction: grid::Direction,
}
