pub mod block_data;
pub mod door_data;

use crate::simulation::state::world::block::block_state::{
    block_data::BlockData, door_data::DoorData,
};

#[derive(Clone)]
pub enum BlockState {
    Block(BlockData),
    Ladder,
    Door(DoorData),
    Stairs,
}
