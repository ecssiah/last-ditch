pub mod construct_population_data;
pub mod construct_world_data;

use crate::simulation::state::{
    work::construct_task::{
        construct_population_data::ConstructPopulationData,
        construct_world_data::ConstructWorldData,
    },
    State,
};

#[derive(Clone)]
pub enum ConstructTask {
    ConstructWorld(ConstructWorldData),
    ConstructPopulation(ConstructPopulationData),
}

impl ConstructTask {
    pub fn cost(construct_task: &ConstructTask) -> u32 {
        match construct_task {
            ConstructTask::ConstructWorld(construct_world_data) => {
                ConstructWorldData::cost(&construct_world_data)
            },
            ConstructTask::ConstructPopulation(construct_population_data) => {
                ConstructPopulationData::cost(&construct_population_data)
            }
        }
    }

    pub fn step(state: &mut State, construct_task: &mut ConstructTask) -> bool {
        match construct_task {
            ConstructTask::ConstructWorld(construct_world_data) => {
                ConstructWorldData::step(state, construct_world_data)
            },
            ConstructTask::ConstructPopulation(construct_population_data) => {
                ConstructPopulationData::step(state, construct_population_data)
            }
        }
    }
}
