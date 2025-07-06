//! Agent task processing

pub mod result;
pub mod task;

use crate::simulation::state::{population::Population, world::World};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;

pub struct Compute {
    pub thread_pool: rayon::ThreadPool,
    pub task_tx: Sender<task::Kind>,
    pub task_rx: Receiver<task::Kind>,
    pub result_tx: Sender<result::Kind>,
    pub result_rx: Receiver<result::Kind>,
}

impl Compute {
    pub fn new() -> Self {
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();

        let (task_tx, task_rx) = unbounded::<task::Kind>();
        let (result_tx, result_rx) = unbounded::<result::Kind>();

        Self {
            thread_pool,
            task_tx,
            task_rx,
            result_tx,
            result_rx,
        }
    }

    pub fn tick(&mut self, _world: &World, _population: &Population) {}
}

impl Default for Compute {
    fn default() -> Self {
        Self::new()
    }
}
