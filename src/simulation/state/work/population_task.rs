pub mod construct_population_data;

use crate::simulation::state::{
    work::population_task::construct_population_data::ConstructPopulationData, Population, World,
};

#[derive(Clone)]
pub enum PopulationTask {
    ConstructPopulation(ConstructPopulationData),
}

impl PopulationTask {
    pub fn cost(population_task: &PopulationTask) -> u32 {
        match population_task {
            PopulationTask::ConstructPopulation(construct_population_data) => {
                ConstructPopulationData::cost(&construct_population_data)
            }
        }
    }

    pub fn step(
        world: &World,
        population: &mut Population,
        population_task: &mut PopulationTask,
    ) -> bool {
        match population_task {
            PopulationTask::ConstructPopulation(construct_population_data) => {
                ConstructPopulationData::step(world, population, construct_population_data)
            }
        }
    }
}
