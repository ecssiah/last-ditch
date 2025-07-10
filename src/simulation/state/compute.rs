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

    pub fn tick(compute: &mut Compute, population: &mut Population, _world: &World) {
        for task in population.task_vec.drain(..) {
            match compute.task_tx.send(task) {
                Ok(()) => {}
                Err(err) => {
                    log::error!("{:?}", err);
                }
            }
        }

        while let Ok(result) = compute.result_rx.try_recv() {
            match result {
                Result::Path(ref path_data) => match path_data {
                    result::path::Data::Regional(regional_data) => {
                        if let Some(agent) = population.agent_map.get_mut(&regional_data.agent_id) {
                            agent.result_vec.push(result);
                        }
                    }
                    result::path::Data::Local(local_data) => {
                        if let Some(agent) = population.agent_map.get_mut(&local_data.agent_id) {
                            agent.result_vec.push(result);
                        }
                    }
                },
            }
        }
    }
}
