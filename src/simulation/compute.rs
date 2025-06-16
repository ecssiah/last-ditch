//! Agent task processing

pub mod result;
pub mod snapshot;
pub mod task;

pub use task::Task;

use crate::simulation::{population::Population, world::World};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;

pub struct Compute {
    pub task_tx: Sender<task::Kind>,
    pub task_rx: Receiver<task::Kind>,
    pub result_tx: Sender<result::Kind>,
    pub result_rx: Receiver<result::Kind>,
    pub thread_pool: rayon::ThreadPool,
}

impl Compute {
    pub fn new() -> Self {
        let (task_tx, task_rx) = unbounded::<task::Kind>();
        let (result_tx, result_rx) = unbounded::<result::Kind>();

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
        for task in self.task_rx.try_iter() {
            match task {
                task::Kind::ChunkPath(task) => {
                    let result_tx = self.result_tx.clone();
                    let snapshot = task.snapshot(world, population);

                    self.thread_pool.spawn(move || {
                        let result = task.execute(snapshot);
                        let _ = result_tx.send(result::Kind::ChunkPath(result));
                    });
                }
                task::Kind::WorldPath(task) => {
                    let result_tx = self.result_tx.clone();
                    let snapshot = task.snapshot(world, population);

                    self.thread_pool.spawn(move || {
                        let result = task.execute(snapshot);
                        let _ = result_tx.send(result::Kind::WorldPath(result));
                    });
                }
            }
        }
    }
}
