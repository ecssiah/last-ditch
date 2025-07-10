//! Agent task processing

pub mod result;
pub mod task;

pub use result::Result;
pub use task::Task;

use crate::simulation::state::{
    population::{
        entity::{self, Agent},
        Population,
    },
    world::World,
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;

pub struct Compute {
    pub thread_pool: rayon::ThreadPool,
    pub result_tx: Sender<Result>,
    pub result_rx: Receiver<Result>,
}

impl Compute {
    pub fn new() -> Self {
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();
        let (result_tx, result_rx) = unbounded::<Result>();

        Self {
            thread_pool,
            result_tx,
            result_rx,
        }
    }

    pub fn tick(compute: &mut Compute, population: &mut Population, _world: &World) {
        Self::process_tasks(
            &compute.thread_pool,
            &compute.result_tx,
            &mut population.task_vec,
        );

        Self::distribute_results(&compute.result_rx, &mut population.agent_map);
    }

    fn process_tasks(
        thread_pool: &rayon::ThreadPool,
        result_tx: &Sender<Result>,
        task_vec: &mut Vec<Task>,
    ) {
        for mut task in task_vec.drain(..) {
            let result_tx = result_tx.clone();

            thread_pool.spawn(move || {
                let result = Task::execute(&mut task);

                if let Err(err) = result_tx.send(result) {
                    log::error!("Failed to send result: {:?}", err);
                }
            });
        }
    }

    fn distribute_results(
        result_rx: &Receiver<Result>,
        agent_map: &mut HashMap<entity::ID, Agent>,
    ) {
        while let Ok(result) = result_rx.try_recv() {
            match result {
                Result::Path(ref path_data) => match path_data {
                    result::path::Kind::Regional(regional_data) => {
                        if let Some(agent) = agent_map.get_mut(&regional_data.agent_id) {
                            agent.result_vec.push(result);
                        }
                    }
                    result::path::Kind::Local(local_data) => {
                        if let Some(agent) = agent_map.get_mut(&local_data.agent_id) {
                            agent.result_vec.push(result);
                        }
                    }
                },
            }
        }
    }
}
