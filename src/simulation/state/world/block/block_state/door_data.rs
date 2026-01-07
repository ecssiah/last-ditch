use crate::simulation::state::world::block::block_state::door_part::DoorPart;

#[derive(Clone)]
pub struct DoorData {
    pub is_open: bool,
    pub door_part: DoorPart,
}
