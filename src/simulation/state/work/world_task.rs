pub mod construct_world_data;

use crate::simulation::state::{work::world_task::construct_world_data::ConstructWorldData, World};

#[derive(Clone)]
pub enum WorldTask {
    ConstructWorld(ConstructWorldData),
}

impl WorldTask {
    pub fn cost(world_task: &WorldTask) -> u32 {
        match world_task {
            WorldTask::ConstructWorld(construct_world_data) => {
                ConstructWorldData::cost(&construct_world_data)
            }
        }
    }

    pub fn step(world: &mut World, world_task: &mut WorldTask) -> bool {
        match world_task {
            WorldTask::ConstructWorld(construct_world_data) => {
                ConstructWorldData::step(world, construct_world_data)
            }
        }
    }
}
