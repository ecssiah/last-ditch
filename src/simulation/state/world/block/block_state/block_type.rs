use crate::simulation::state::world::block::block_state::door_data::DoorData;

#[derive(Clone)]
pub enum BlockType {
    Block,
    Door(DoorData),
    Ladder,
    Stairs,
}
