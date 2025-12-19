use crate::simulation::state::world::{block::Block, Object};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Cell {
    pub sector_id: usize,
    pub cell_id: usize,
    pub grid_position: IVec3,
    pub block: Option<Block>,
    pub object: Option<Object>,
}
