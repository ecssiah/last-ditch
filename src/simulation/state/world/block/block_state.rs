pub mod block_type;
pub mod door_data;
pub mod door_part;

use crate::simulation::state::world::{
    block::block_state::block_type::BlockType,
    grid::{direction_set::DirectionSet, Direction},
};

#[derive(Clone)]
pub struct BlockState {
    pub direction: Direction,
    pub exposure_set: DirectionSet,
    pub block_type: BlockType,
}
