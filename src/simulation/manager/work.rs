use crate::simulation::{manager::work::worker::Worker, state::State};

pub mod navigation_task;
pub mod navigation_worker;
pub mod population_task;
pub mod population_worker;
pub mod worker;
pub mod world_task;
pub mod world_worker;

pub struct Work {
    pub worker_vec: Vec<Box<dyn Worker>>,
}

impl Work {
    pub fn new() -> Self {
        let worker_vec = Vec::new();

        Self { worker_vec }
    }

    pub fn tick(_state: &mut State, _work: &mut Work) {

    }
}
