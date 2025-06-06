//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod result;
pub mod task;

use crossbeam::channel::{unbounded, Receiver, Sender};
use result::Result;
use task::Task;

pub struct Compute {
    pub task_tx: Sender<Task>,
    pub task_rx: Receiver<Task>,
    pub result_tx: Sender<Result>,
    pub result_rx: Receiver<Result>,
}

impl Compute {
    pub fn new() -> Compute {
        let (task_tx, task_rx) = unbounded();
        let (result_tx, result_rx) = unbounded();

        let compute = Compute {
            task_tx,
            task_rx,
            result_tx,
            result_rx,
        };

        compute
    }
}
