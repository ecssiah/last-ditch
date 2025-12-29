use crate::simulation::state::world::object::{Block, Door, Ladder, Stairs};
use ultraviolet::IVec3;

pub struct Sector {
    pub version: u64,
    pub sector_index: usize,
    pub grid_position: IVec3,
    pub block_vec: Vec<Option<Block>>,
    pub door_vec: Vec<Option<Door>>,
    pub stairs_vec: Vec<Option<Stairs>>,
    pub ladder_vec: Vec<Option<Ladder>>,
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

    pub fn get_door(cell_index: usize, door_slice: &[Option<Door>]) -> Option<&Door> {
        door_slice.get(cell_index).and_then(|door| door.as_ref())
    }

    pub fn get_door_mut(cell_index: usize, door_slice: &mut [Option<Door>]) -> Option<&mut Door> {
        door_slice
            .get_mut(cell_index)
            .and_then(|door| door.as_mut())
    }

    pub fn get_stairs(cell_index: usize, stairs_slice: &[Option<Stairs>]) -> Option<&Stairs> {
        stairs_slice
            .get(cell_index)
            .and_then(|stairs| stairs.as_ref())
    }

    pub fn get_stairs_mut(
        cell_index: usize,
        stairs_slice: &mut [Option<Stairs>],
    ) -> Option<&mut Stairs> {
        stairs_slice
            .get_mut(cell_index)
            .and_then(|stairs| stairs.as_mut())
    }

    pub fn get_ladder(cell_index: usize, ladder_slice: &[Option<Ladder>]) -> Option<&Ladder> {
        ladder_slice
            .get(cell_index)
            .and_then(|ladder| ladder.as_ref())
    }

    pub fn get_ladder_mut(
        cell_index: usize,
        ladder_slice: &mut [Option<Ladder>],
    ) -> Option<&mut Ladder> {
        ladder_slice
            .get_mut(cell_index)
            .and_then(|ladder| ladder.as_mut())
    }
}
