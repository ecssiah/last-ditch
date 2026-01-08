pub mod sector_index;

use crate::simulation::state::world::{
    block::Block, cell::cell_index::CellIndex, sector::sector_index::SectorIndex,
};
use ultraviolet::IVec3;

pub struct Sector {
    pub version: u64,
    pub sector_index: SectorIndex,
    pub grid_position: IVec3,
    pub block_vec: Vec<Option<Block>>,
}

impl Sector {
    pub fn get_block(cell_index: CellIndex, block_slice: &[Option<Block>]) -> Option<&Block> {
        block_slice.get(CellIndex::as_index(&cell_index))?.as_ref()
    }

    pub fn get_block_mut(
        cell_index: CellIndex,
        block_slice: &mut [Option<Block>],
    ) -> Option<&mut Block> {
        block_slice
            .get_mut(CellIndex::as_index(&cell_index))?
            .as_mut()
    }
}
