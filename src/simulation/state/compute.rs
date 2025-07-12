//! Agent task processing

pub mod result;
pub mod task;

pub use result::Result;
pub use task::Task;

use crate::simulation::state::{
    compute,
    population::{
        entity::{self, Agent},
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
    pub result_tx: Sender<compute::Result>,
    pub result_rx: Receiver<compute::Result>,
    pub task_heap: BinaryHeap<compute::Task>,
    pub task_store_arc_lock: Arc<RwLock<task::Store>>,
    pub result_store_arc_lock: Arc<RwLock<result::Store>>,
}

impl Compute {
    pub fn new() -> Self {
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();
        let (result_tx, result_rx) = unbounded::<compute::Result>();

        let task_heap = BinaryHeap::new();

        let task_store_arc_lock = Arc::new(RwLock::new(task::Store::new()));
        let result_store_arc_lock = Arc::new(RwLock::new(result::Store::new()));

        Self {
            thread_pool,
            result_tx,
            result_rx,
            task_heap,
            task_store_arc_lock,
            result_store_arc_lock,
        }
    }

    pub fn tick(compute: &mut Compute, population: &mut Population) {
        Self::process_tasks(
            &compute.thread_pool,
            &compute.result_tx,
            &mut compute.task_heap,
            &compute.task_store_arc_lock,
            &compute.result_store_arc_lock,
        );

        Self::distribute_results(
            &compute.result_rx,
            &compute.result_store_arc_lock,
            &mut population.agent_map,
        );
    }

    fn process_tasks(
        thread_pool: &rayon::ThreadPool,
        result_tx: &Sender<compute::Result>,
        task_heap: &mut BinaryHeap<compute::Task>,
        task_store_arc_lock: &Arc<RwLock<task::Store>>,
        result_store_arc_lock: &Arc<RwLock<result::Store>>,
    ) {
        let mut current_tasks: Vec<_> = task_heap.drain().collect();

        while let Some(task) = current_tasks.pop() {
            let result_tx_clone = result_tx.clone();
            let task_store_arc_lock_clone = task_store_arc_lock.clone();
            let result_store_arc_lock_clone = result_store_arc_lock.clone();

            thread_pool.spawn(move || {
                let result = Self::execute_task(
                    task,
                    task_store_arc_lock_clone,
                    result_store_arc_lock_clone,
                );

                if let Err(err) = result_tx_clone.send(result) {
                    log::error!("Failed to send result: {:?}", err);
                }
            });
        }
    }

    fn execute_task(
        task: compute::Task,
        task_store_arc_lock: Arc<RwLock<task::Store>>,
        result_store_arc_lock: Arc<RwLock<result::Store>>,
    ) -> compute::Result {
        match task.kind {
            task::Kind::PathRegion => {
                Self::execute_path_region_task(task, task_store_arc_lock, result_store_arc_lock)
            }
            task::Kind::PathLocal => {
                Self::execute_path_local_task(task, task_store_arc_lock, result_store_arc_lock)
            }
        }
    }

    fn execute_path_region_task(
        task: compute::Task,
        task_store_arc_lock: Arc<RwLock<task::Store>>,
        result_store_arc_lock: Arc<RwLock<result::Store>>,
    ) -> compute::Result {
        let mut task_data = {
            let mut task_store = task_store_arc_lock.write().unwrap();

            task_store
                .path_region_data_map
                .remove(&task.id)
                .expect("Task is missing PathRegion data")
        };

        let node_vec = Graph::find_region_path(
            task_data.start_position,
            task_data.end_position,
            &task_data.level_0,
            &mut task_data.search_level,
        );

        let position_vec = node_vec.iter().map(|node| node.position).collect();

        let result_data = compute::result::data::path::Region {
            plan_id: task_data.plan_id,
            entity_id: task_data.entity_id,
            position_vec,
        };

        let result = compute::Result::new(compute::result::Kind::RegionPath);

        {
            let mut result_store = result_store_arc_lock.write().unwrap();

            result_store
                .path_region_data_map
                .insert(result.id, result_data);
        }

        result
    }

    fn execute_path_local_task(
        task: compute::Task,
        task_store_arc_lock: Arc<RwLock<task::Store>>,
        result_store_arc_lock: Arc<RwLock<result::Store>>,
    ) -> compute::Result {
        let task_data = {
            let mut task_store = task_store_arc_lock.write().unwrap();

            task_store
                .path_local_data_map
                .remove(&task.id)
                .expect("Task is missing PathLocal data")
        };

        let node_vec = Graph::find_local_path(
            task_data.start_position,
            task_data.end_position,
            &task_data.level_0,
        );

        let position_vec = node_vec.iter().map(|node| node.position).collect();

        let result = compute::Result::new(compute::result::Kind::LocalPath);

        let result_data = compute::result::data::path::Local {
            plan_id: task_data.plan_id,
            entity_id: task_data.entity_id,
            chunk_id: task_data.chunk_id,
            position_vec,
        };

        {
            let mut result_store = result_store_arc_lock.write().unwrap();

            result_store
                .path_local_data_map
                .insert(result.id, result_data);
        }

        result
    }

    fn distribute_results(
        result_rx: &Receiver<compute::Result>,
        result_store_arc_lock: &Arc<RwLock<result::Store>>,
        agent_map: &mut HashMap<entity::ID, Agent>,
    ) {
        while let Ok(result) = result_rx.try_recv() {
            match result.kind {
                compute::result::Kind::RegionPath => {
                    Self::distribute_region_path_results(result, result_store_arc_lock, agent_map)
                }
                compute::result::Kind::LocalPath => {
                    Self::distribute_local_path_results(result, result_store_arc_lock, agent_map)
                }
            }
        }
    }

    fn distribute_region_path_results(
        result: compute::Result,
        result_store_arc_lock: &Arc<RwLock<result::Store>>,
        agent_map: &mut HashMap<entity::ID, Agent>,
    ) {
        let mut result_store = result_store_arc_lock.write().unwrap();

        let result_data = result_store
            .path_region_data_map
            .remove(&result.id)
            .unwrap();

        if let Some(agent) = agent_map.get_mut(&result_data.entity_id) {
            let travel_data = agent
                .decision
                .plan_store
                .travel_data_map
                .get_mut(&result_data.plan_id)
                .unwrap();

            travel_data.path_found = true;
            travel_data.path_index = 1;
            travel_data.region_path_vec = result_data.position_vec;

            println!("Region Path: ");
            for position in &travel_data.region_path_vec {
                println!("{:?}", position);
            }
        }
    }

    fn distribute_local_path_results(
        result: compute::Result,
        result_store_arc_lock: &Arc<RwLock<result::Store>>,
        agent_map: &mut HashMap<entity::ID, Agent>,
    ) {
        let mut result_store = result_store_arc_lock.write().unwrap();

        let result_data = result_store.path_local_data_map.remove(&result.id).unwrap();

        if let Some(agent) = agent_map.get_mut(&result_data.entity_id) {
            let travel_data = agent
                .decision
                .plan_store
                .travel_data_map
                .get_mut(&result_data.plan_id)
                .unwrap();

            travel_data.local_path_found = true;
            travel_data.local_path_index = 0;
            travel_data.local_path_vec = result_data.position_vec;
        }
    }
}
