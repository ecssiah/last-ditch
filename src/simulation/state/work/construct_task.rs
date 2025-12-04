pub mod generate_population_data;
pub mod generate_world_data;

use crate::simulation::state::{
    work::construct_task::{
        generate_population_data::GeneratePopulationData, generate_world_data::GenerateWorldData,
    },
    State,
};

#[derive(Clone)]
pub enum ConstructTask {
    GenerateWorld(GenerateWorldData),
    GeneratePopulation(GeneratePopulationData),
}

impl ConstructTask {
    pub fn cost(construct_task: &Self) -> u32 {
        match construct_task {
            ConstructTask::GenerateWorld(generate_world_data) => {
                GenerateWorldData::cost(generate_world_data)
            }
            ConstructTask::GeneratePopulation(generate_population_data) => {
                GeneratePopulationData::cost(generate_population_data)
            }
        }
    }

    pub fn step(state: &mut State, construct_task: &mut Self) -> bool {
        match construct_task {
            ConstructTask::GenerateWorld(generate_world_data) => {
                GenerateWorldData::step(state, generate_world_data)
            }
            ConstructTask::GeneratePopulation(generate_population_data) => {
                GeneratePopulationData::step(state, generate_population_data)
            }
        }
    }
}
