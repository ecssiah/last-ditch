//! Agent task processing

pub mod result;
pub mod task;

pub use result::Result;
pub use task::Task;

use crate::simulation::state::{population::Population, world::World};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;

pub struct Compute {
    pub thread_pool: rayon::ThreadPool,
    pub task_tx: Sender<Task>,
    pub task_rx: Receiver<Task>,
    pub result_tx: Sender<Result>,
    pub result_rx: Receiver<Result>,
}

impl Compute {
    pub fn new() -> Self {
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();

        let (task_tx, task_rx) = unbounded::<Task>();
        let (result_tx, result_rx) = unbounded::<Result>();

        Self {
            thread_pool,
            task_tx,
            task_rx,
            result_tx,
            result_rx,
        }
    }

    pub fn tick(_compute: &Compute, _world: &World, _population: &Population) {}
}
