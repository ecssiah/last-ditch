use crate::simulation::state::world::block::Block;
use ultraviolet::IVec3;

pub struct Sector {
    pub version: u64,
    pub sector_index: usize,
    pub grid_position: IVec3,
    pub block_vec: Vec<Option<Block>>,
}

impl Sector {
    pub fn get_block(cell_index: usize, block_slice: &[Option<Block>]) -> Option<&Block> {
        block_slice.get(cell_index).and_then(|block| block.as_ref())
    }

    pub fn get_block_mut(
        cell_index: usize,
        block_slice: &mut [Option<Block>],
    ) -> Option<&mut Block> {
        block_slice
            .get_mut(cell_index)
            .and_then(|block| block.as_mut())
    }
}
