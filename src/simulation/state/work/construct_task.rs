pub mod generate_data;

use crate::simulation::state::{work::construct_task::generate_data::GenerateData, State};

#[derive(Clone)]
pub enum ConstructTask {
    Generate(GenerateData),
}

impl ConstructTask {
    pub fn cost(construct_task: &Self) -> u32 {
        match construct_task {
            ConstructTask::Generate(generate_data) => GenerateData::cost(generate_data),
        }
    }

    pub fn step(state: &mut State, construct_task: &mut Self) -> bool {
        match construct_task {
            ConstructTask::Generate(generate_data) => GenerateData::step(state, generate_data),
        }
    }
}
