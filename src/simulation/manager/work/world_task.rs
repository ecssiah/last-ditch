use crate::simulation::state::{World};

pub enum WorldTask {
    Construct,
}

impl WorldTask {
    pub fn step(_world: &mut World, _world_task: &mut WorldTask) -> bool {
        false
    }
}