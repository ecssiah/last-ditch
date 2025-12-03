pub mod generation_data;

use crate::simulation::state::{work::construct_task::generation_data::GenerationData, State};

#[derive(Clone)]
pub enum ConstructTask {
    GenerationTask(GenerationData),
}

impl ConstructTask {
    pub fn cost(construct_task: &Self) -> u32 {
        match construct_task {
            Self::GenerationTask(generation_data) => {
                GenerationData::cost(&generation_data)
            }
        }
    }

    pub fn step(state: &mut State, construct_task: &mut Self) -> bool {
        match construct_task {
            Self::GenerationTask(generation_data) => {
                GenerationData::step(state, generation_data)
            }
        }
    }
}
