pub mod construct_world_data;

use crate::simulation::{
    manager::work::world_task::construct_world_data::ConstructWorldData,
    state::{population::nation, World},
};

#[derive(Clone)]
pub enum WorldTask {
    ConstructWorld(ConstructWorldData),
}

impl WorldTask {
    pub fn cost(world_task: &WorldTask) -> u32 {
        match world_task {
            WorldTask::ConstructWorld(construct_world_data) => ConstructWorldData::cost(&construct_world_data),
        }
    }

    pub fn step(world: &mut World, world_task: &mut WorldTask) -> bool {
        match world_task {
            WorldTask::ConstructWorld(construct_world_data) => {
                Self::construct_world(world, construct_world_data)
            }
        }
    }

    fn construct_world(world: &mut World, construct_world_data: &mut ConstructWorldData) -> bool {
        match construct_world_data.stage {
            0 => {
                ConstructWorldData::build_ground(world);

                construct_world_data.stage += 1;

                false
            }
            1 => {
                ConstructWorldData::build_compass(world);

                ConstructWorldData::build_temple(34, 0, 0, nation::Kind::Wolf, world);
                ConstructWorldData::build_temple(-34, 0, 0, nation::Kind::Lion, world);
                ConstructWorldData::build_temple(0, 34, 0, nation::Kind::Eagle, world);
                ConstructWorldData::build_temple(0, -34, 0, nation::Kind::Horse, world);

                construct_world_data.stage += 1;

                false
            }
            2 => {
                ConstructWorldData::build_observation_deck(world);

                construct_world_data.stage += 1;

                false
            }
            _ => true,
        }
    }
}
