use crate::simulation::state::world::block;
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_id: usize,
    pub sector_id: usize,
    pub grid_position: IVec3,
    pub block_kind: block::Kind,
    pub solid: bool,
}
