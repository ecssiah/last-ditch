//! Agent task processing

pub mod result;
pub mod task;

pub use result::Result;
pub use task::Task;

use crate::simulation::{compute, world::World};
use crossbeam::channel::{unbounded, Receiver, Sender};

pub struct Compute {
    pub task_tx: Sender<Box<dyn compute::Task>>,
    pub task_rx: Receiver<Box<dyn compute::Task>>,
    pub result_tx: Sender<Box<dyn compute::Result>>,
    pub result_rx: Receiver<Box<dyn compute::Result>>,
}

impl Compute {
    pub fn new() -> Self {
        let (task_tx, task_rx) = unbounded::<Box<dyn compute::Task>>();
        let (result_tx, result_rx) = unbounded::<Box<dyn compute::Result>>();

        Self {
            task_tx,
            task_rx,
            result_tx,
            result_rx,
        }
    }

    pub fn tick(&mut self, world: &World) {}
}
