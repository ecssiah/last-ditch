use crate::simulation::state::world::{
    grid::Direction,
    structure::{structure_kind::StructureKind, Structure},
};
use ultraviolet::IVec3;

#[derive(Clone)]
pub struct StructureView {
    pub structure_kind: StructureKind,
    pub grid_position: IVec3,
    pub direction: Direction,
}

impl StructureView {
    pub fn new_from_structure(structure: &Structure) -> Self {
        Self {
            structure_kind: structure.structure_kind.clone(),
            grid_position: structure.grid_position,
            direction: structure.direction.clone(),
        }
    }
}
