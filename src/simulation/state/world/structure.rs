pub mod structure_info;
pub mod structure_kind;

use crate::simulation::state::world::{grid::Direction, structure::structure_kind::StructureKind};
use ultraviolet::IVec3;

#[derive(Clone)]
pub struct Structure {
    pub structure_kind: StructureKind,
    pub grid_position: IVec3,
    pub direction: Direction,
}

impl Structure {
    pub fn new(
        structure_kind: &StructureKind,
        grid_position: IVec3,
        direction: &Direction,
    ) -> Self {
        Self {
            structure_kind: structure_kind.clone(),
            grid_position,
            direction: direction.clone(),
        }
    }
}
