//! Agent task processing

pub mod task_input;
pub mod task_output;

pub use task_input::TaskInput;
pub use task_output::TaskOutput;

use crate::simulation::state::{
    compute,
    population::{
        entity::{self, decision::plan, Agent},
        Population,
    },
    world::graph::Graph,
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;
use std::{
    collections::{BinaryHeap, HashMap},
    sync::{Arc, RwLock},
};

pub struct Compute {
    pub thread_pool: rayon::ThreadPool,
    pub task_output_tx: Sender<TaskOutput>,
    pub task_output_rx: Receiver<TaskOutput>,
    pub task_input_heap: BinaryHeap<TaskInput>,
    pub task_input_store_arc_lock: Arc<RwLock<task_input::Store>>,
    pub task_output_store_arc_lock: Arc<RwLock<task_output::Store>>,
}

impl Compute {
    pub fn new() -> Self {
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();
        let (task_output_tx, task_output_rx) = unbounded::<TaskOutput>();

        let task_input_heap = BinaryHeap::new();
        let task_input_store_arc_lock = Arc::new(RwLock::new(task_input::Store::new()));
        let task_output_store_arc_lock = Arc::new(RwLock::new(task_output::Store::new()));

        Self {
            thread_pool,
            task_output_tx,
            task_output_rx,
            task_input_heap,
            task_input_store_arc_lock,
            task_output_store_arc_lock,
        }
    }

    pub fn tick(compute: &mut Compute, population: &mut Population) {
        Self::process_tasks(
            &compute.thread_pool,
            &compute.task_output_tx,
            &mut compute.task_input_heap,
            &compute.task_input_store_arc_lock,
            &compute.task_output_store_arc_lock,
        );

        Self::distribute_task_outputs(
            &compute.task_output_rx,
            &compute.task_output_store_arc_lock,
            &mut population.agent_map,
        );
    }

    fn process_tasks(
        thread_pool: &rayon::ThreadPool,
        task_output_tx: &Sender<TaskOutput>,
        task_input_heap: &mut BinaryHeap<TaskInput>,
        task_input_store_arc_lock: &Arc<RwLock<task_input::Store>>,
        task_output_store_arc_lock: &Arc<RwLock<task_output::Store>>,
    ) {
        let mut task_input_vec: Vec<_> = task_input_heap.drain().collect();

        while let Some(task_input) = task_input_vec.pop() {
            let task_output_tx_clone = task_output_tx.clone();
            let task_input_store_arc_lock_clone = task_input_store_arc_lock.clone();
            let task_output_store_arc_lock_clone = task_output_store_arc_lock.clone();

            thread_pool.spawn(move || {
                let task_output = Self::execute_task_input(
                    task_input,
                    task_input_store_arc_lock_clone,
                    task_output_store_arc_lock_clone,
                );

                if let Err(err) = task_output_tx_clone.send(task_output) {
                    log::error!("Failed to send TaskOutput: {:?}", err);
                }
            });
        }
    }

    fn execute_task_input(
        task_input: TaskInput,
        task_input_store_arc_lock: Arc<RwLock<task_input::Store>>,
        task_output_store_arc_lock: Arc<RwLock<task_output::Store>>,
    ) -> TaskOutput {
        match task_input.kind {
            task_input::Kind::PathRegion => Self::execute_path_region_task_input(
                task_input,
                task_input_store_arc_lock,
                task_output_store_arc_lock,
            ),
            task_input::Kind::PathLocal => Self::execute_path_local_task_input(
                task_input,
                task_input_store_arc_lock,
                task_output_store_arc_lock,
            ),
        }
    }

    fn execute_path_region_task_input(
        task_input: TaskInput,
        task_input_store_arc_lock: Arc<RwLock<task_input::Store>>,
        task_output_store_arc_lock: Arc<RwLock<task_output::Store>>,
    ) -> TaskOutput {
        let mut task_data = {
            let mut task_input_store = task_input_store_arc_lock.write().unwrap();

            task_input_store
                .path_region_data_map
                .remove(&task_input.id)
                .expect("Task is missing PathRegion data")
        };

        let path = Graph::find_region_path(
            task_data.start_position,
            task_data.end_position,
            &task_data.level_0,
            &mut task_data.search_level,
        );

        let task_output = TaskOutput::new(compute::task_output::Kind::RegionPath);

        let task_output_data = compute::task_output::data::path::Region {
            plan_id: task_data.plan_id,
            entity_id: task_data.entity_id,
            path,
        };

        {
            let mut task_output_store = task_output_store_arc_lock.write().unwrap();

            task_output_store
                .path_region_data_map
                .insert(task_output.id, task_output_data);
        }

        task_output
    }

    fn execute_path_local_task_input(
        task_input: TaskInput,
        task_input_store_arc_lock: Arc<RwLock<task_input::Store>>,
        task_output_store_arc_lock: Arc<RwLock<task_output::Store>>,
    ) -> TaskOutput {
        let task_data = {
            let mut task_input_store = task_input_store_arc_lock.write().unwrap();

            task_input_store
                .path_local_data_map
                .remove(&task_input.id)
                .expect("Task is missing PathLocal data")
        };

        let path = Graph::find_local_path(
            task_data.start_position,
            task_data.end_position,
            &task_data.level_0,
        );

        let task_output = TaskOutput::new(task_output::Kind::LocalPath);

        let task_output_data = task_output::data::path::Local {
            plan_id: task_data.plan_id,
            entity_id: task_data.entity_id,
            chunk_id: task_data.chunk_id,
            path,
        };

        {
            let mut task_output_store = task_output_store_arc_lock.write().unwrap();

            task_output_store
                .path_local_data_map
                .insert(task_output.id, task_output_data);
        }

        task_output
    }

    fn distribute_task_outputs(
        task_output_rx: &Receiver<TaskOutput>,
        task_output_store_arc_lock: &Arc<RwLock<task_output::Store>>,
        agent_map: &mut HashMap<entity::ID, Agent>,
    ) {
        while let Ok(task_output) = task_output_rx.try_recv() {
            match task_output.kind {
                task_output::Kind::RegionPath => Self::distribute_task_outputs_region_path(
                    task_output,
                    task_output_store_arc_lock,
                    agent_map,
                ),
                task_output::Kind::LocalPath => Self::distribute_task_outputs_local_path(
                    task_output,
                    task_output_store_arc_lock,
                    agent_map,
                ),
            }
        }
    }

    fn distribute_task_outputs_region_path(
        task_output: TaskOutput,
        task_output_store_arc_lock: &Arc<RwLock<task_output::Store>>,
        agent_map: &mut HashMap<entity::ID, Agent>,
    ) {
        let mut task_output_store = task_output_store_arc_lock.write().unwrap();

        let task_output_data = task_output_store
            .path_region_data_map
            .remove(&task_output.id)
            .unwrap();

        if let Some(agent) = agent_map.get_mut(&task_output_data.entity_id) {
            let travel_data = agent
                .decision
                .plan_store
                .travel_data_map
                .get_mut(&task_output_data.plan_id)
                .unwrap();

            if task_output_data.path.valid {
                travel_data.region_path_found = true;
                travel_data.region_path_index = 1;
                travel_data.region_path = task_output_data.path;
            } else {
                travel_data.region_path_found = false;
                travel_data.stage = plan::Stage::Fail;
            }
        }
    }

    fn distribute_task_outputs_local_path(
        task_output: TaskOutput,
        task_output_store_arc_lock: &Arc<RwLock<task_output::Store>>,
        agent_map: &mut HashMap<entity::ID, Agent>,
    ) {
        let mut task_output_store = task_output_store_arc_lock.write().unwrap();

        let task_output_data = task_output_store
            .path_local_data_map
            .remove(&task_output.id)
            .unwrap();

        if let Some(agent) = agent_map.get_mut(&task_output_data.entity_id) {
            let travel_data = agent
                .decision
                .plan_store
                .travel_data_map
                .get_mut(&task_output_data.plan_id)
                .unwrap();

            if task_output_data.path.valid {
                travel_data.local_path_found = true;
                travel_data.local_path_index = 0;
                travel_data.local_path = task_output_data.path;
            } else {
                travel_data.local_path_found = false;
                travel_data.stage = plan::Stage::Fail;
            }
        }
    }
}
