pub mod kind;

pub use kind::Kind;

use crate::simulation::state::world::{grid, object};
use ultraviolet::IVec3;

pub struct Object {
    pub object_id: u64,
    pub kind: object::Kind,
    pub grid_position: IVec3,
    pub direction: grid::Direction,
}
