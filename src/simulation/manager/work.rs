use crate::simulation::manager::work::worker::Worker;

pub mod navigation_task;
pub mod navigation_worker;
pub mod population_task;
pub mod population_worker;
pub mod worker;
pub mod world_task;
pub mod world_worker;

pub struct Work {
    pub budget: u32,
    pub worker_vec: Vec<Box<dyn Worker>>,
}

impl Work {
    pub fn new() -> Self {
        let budget = 500;
        let worker_vec = Vec::new();

        Self { budget, worker_vec }
    }
}
