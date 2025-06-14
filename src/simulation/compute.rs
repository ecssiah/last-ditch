//! Agent task processing

pub mod result;
pub mod snapshot;
pub mod task;

pub use result::Result;
pub use snapshot::Snapshot;
pub use task::Task;

use crate::simulation::{compute, population::Population, world::World};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;
use std::sync::Arc;

pub struct Compute {
    pub task_tx: Sender<Box<dyn compute::Task>>,
    pub task_rx: Receiver<Box<dyn compute::Task>>,
    pub result_tx: Sender<Box<dyn compute::Result>>,
    pub result_rx: Receiver<Box<dyn compute::Result>>,
    pub thread_pool: rayon::ThreadPool,
}

impl Compute {
    pub fn new() -> Self {
        let (task_tx, task_rx) = unbounded::<Box<dyn compute::Task>>();
        let (result_tx, result_rx) = unbounded::<Box<dyn compute::Result>>();
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();

        Self {
            task_tx,
            task_rx,
            result_tx,
            result_rx,
            thread_pool,
        }
    }

    pub fn tick(&mut self, world: &World, population: &Population) {
        let tasks: Vec<Arc<dyn compute::Task>> = self.task_rx.try_iter().map(Arc::from).collect();

        self.thread_pool.scope(|scope| {
            for task in tasks {
                let task_clone = Arc::clone(&task);

                let result_tx = self.result_tx.clone();

                scope.spawn(move |_| {
                    let snapshot = task_clone.snapshot(world, population);
                    let result = task_clone.execute(snapshot);

                    let _ = result_tx.send(result);
                });
            }
        });
    }
}
