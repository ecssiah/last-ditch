pub mod kind;

pub use kind::Kind;

use crate::simulation::state::world::{grid, object};

#[derive(Clone, Debug)]
pub struct Object {
    pub object_kind: object::Kind,
    pub direction: grid::Direction,
}
