pub mod construct_population_data;

use crate::simulation::state::{Population, World, work::population_task::construct_population_data::ConstructPopulationData};

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
                Self::construct_population(world, population, construct_population_data)
            }
        }
    }

    fn construct_population(
        world: &World,
        population: &mut Population,
        construct_population_data: &mut ConstructPopulationData,
    ) -> bool {
        match construct_population_data.stage {
            0 => {
                ConstructPopulationData::setup_judge(world, population);

                construct_population_data.stage += 1;

                false
            }
            1 => {
                ConstructPopulationData::setup_agent_map(world, population);

                construct_population_data.stage += 1;

                false
            }
            _ => true,
        }
    }
}
