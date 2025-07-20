//! Agent task processing

pub mod task;

use crate::simulation::state::{
    population::{
        entity::{self, decision::plan, Agent},
        Population,
    },
    world::graph::{path, Graph, Path},
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;
use std::{
    collections::{BinaryHeap, HashMap},
    sync::{Arc, RwLock},
};

pub struct Compute {
    pub thread_pool: rayon::ThreadPool,
    pub output_tx: Sender<task::Output>,
    pub output_rx: Receiver<task::Output>,
    pub input_heap: BinaryHeap<task::Input>,
    pub input_store_arc_lock: Arc<RwLock<task::input::Store>>,
    pub output_store_arc_lock: Arc<RwLock<task::output::Store>>,
}

impl Compute {
    pub fn new() -> Self {
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();
        let (output_tx, output_rx) = unbounded::<task::Output>();

        let input_heap = BinaryHeap::new();
        let input_store_arc_lock = Arc::new(RwLock::new(task::input::Store::new()));
        let output_store_arc_lock = Arc::new(RwLock::new(task::output::Store::new()));

        Self {
            thread_pool,
            output_tx,
            output_rx,
            input_heap,
            input_store_arc_lock,
            output_store_arc_lock,
        }
    }

    pub fn tick(compute: &mut Compute, population: &mut Population) {
        Self::process_tasks(
            &compute.thread_pool,
            &compute.output_tx,
            &mut compute.input_heap,
            &compute.input_store_arc_lock,
            &compute.output_store_arc_lock,
        );

        Self::distribute_task_outputs(
            &compute.output_rx,
            &compute.output_store_arc_lock,
            &mut population.agent_map,
        );
    }

    fn process_tasks(
        thread_pool: &rayon::ThreadPool,
        task_output_tx: &Sender<task::Output>,
        task_input_heap: &mut BinaryHeap<task::Input>,
        task_input_store_arc_lock: &Arc<RwLock<task::input::Store>>,
        task_output_store_arc_lock: &Arc<RwLock<task::output::Store>>,
    ) {
        let mut task_input_vec: Vec<_> = task_input_heap.drain().collect();

        while let Some(task_input) = task_input_vec.pop() {
            let task_output_tx_clone = task_output_tx.clone();
            let task_input_store_arc_lock_clone = task_input_store_arc_lock.clone();
            let task_output_store_arc_lock_clone = task_output_store_arc_lock.clone();

            thread_pool.spawn(move || {
                let task_output = Self::execute_task(
                    task_input,
                    task_input_store_arc_lock_clone,
                    task_output_store_arc_lock_clone,
                );

                if let Err(err) = task_output_tx_clone.send(task_output) {
                    log::error!("Failed to send Task output: {:?}", err);
                }
            });
        }
    }

    fn execute_task(
        task_input: task::Input,
        task_input_store_arc_lock: Arc<RwLock<task::input::Store>>,
        task_output_store_arc_lock: Arc<RwLock<task::output::Store>>,
    ) -> task::Output {
        match task_input.kind {
            task::Kind::PathRegion => Self::execute_path_region_task(
                task_input,
                task_input_store_arc_lock,
                task_output_store_arc_lock,
            ),
            task::Kind::PathLocal => Self::execute_path_local_task(
                task_input,
                task_input_store_arc_lock,
                task_output_store_arc_lock,
            ),
        }
    }

    fn execute_path_region_task(
        task_input: task::Input,
        task_input_store_arc_lock: Arc<RwLock<task::input::Store>>,
        task_output_store_arc_lock: Arc<RwLock<task::output::Store>>,
    ) -> task::Output {
        let mut task_input_data = {
            let mut task_input_store = task_input_store_arc_lock.write().unwrap();

            task_input_store
                .path_region_data_map
                .remove(&task_input.id)
                .expect("Task is missing PathRegion data")
        };

        let edge_vec = Graph::find_region_path(
            task_input_data.start_position,
            task_input_data.end_position,
            &task_input_data.level_0,
            &mut task_input_data.search_level,
        );

        let task_output = task::Output::new(task::Kind::PathRegion);

        let task_output_data = task::output::data::path::Region {
            plan_id: task_input_data.plan_id,
            entity_id: task_input_data.entity_id,
            edge_vec,
        };

        {
            let mut task_output_store = task_output_store_arc_lock.write().unwrap();

            task_output_store
                .path_region_data_map
                .insert(task_output.id, task_output_data);
        }

        task_output
    }

    fn execute_path_local_task(
        task_input: task::Input,
        task_input_store_arc_lock: Arc<RwLock<task::input::Store>>,
        task_output_store_arc_lock: Arc<RwLock<task::output::Store>>,
    ) -> task::Output {
        let task_input_data = {
            let mut task_input_store = task_input_store_arc_lock.write().unwrap();

            task_input_store
                .path_local_data_map
                .remove(&task_input.id)
                .expect("Task is missing PathLocal data")
        };

        let edge_vec = Graph::find_local_path(
            task_input_data.start_position,
            task_input_data.end_position,
            &task_input_data.level_0,
        );

        let task_output = task::Output::new(task::Kind::PathLocal);

        let task_output_data = task::output::data::path::Local {
            plan_id: task_input_data.plan_id,
            entity_id: task_input_data.entity_id,
            step_index: task_input_data.step_index,
            edge_vec,
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
        task_output_rx: &Receiver<task::Output>,
        task_output_store_arc_lock: &Arc<RwLock<task::output::Store>>,
        agent_map: &mut HashMap<entity::ID, Agent>,
    ) {
        while let Ok(task_output) = task_output_rx.try_recv() {
            match task_output.kind {
                task::Kind::PathRegion => Self::distribute_task_outputs_path_region(
                    task_output,
                    task_output_store_arc_lock,
                    agent_map,
                ),
                task::Kind::PathLocal => Self::distribute_task_outputs_path_local(
                    task_output,
                    task_output_store_arc_lock,
                    agent_map,
                ),
            }
        }
    }

    fn distribute_task_outputs_path_region(
        task_output: task::Output,
        task_output_store_arc_lock: &Arc<RwLock<task::output::Store>>,
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

            if task_output_data.edge_vec.is_empty() {
                travel_data.stage = plan::Stage::Fail;
            } else {
                let mut path = Path::new();

                for edge in task_output_data.edge_vec {
                    let step = path::Step::new(edge);

                    path.step_vec.push(step);
                }

                travel_data.path = Some(path);
            }
        }
    }

    fn distribute_task_outputs_path_local(
        task_output: task::Output,
        task_output_store_arc_lock: &Arc<RwLock<task::output::Store>>,
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

            if let Some(path) = &mut travel_data.path {
                if let Some(step) = path.step_vec.get_mut(task_output_data.step_index) {
                    let position_vec = if task_output_data.edge_vec.is_empty() {
                        Vec::new()
                    } else {
                        std::iter::once(task_output_data.edge_vec[0].node1.position)
                            .chain(
                                task_output_data
                                    .edge_vec
                                    .iter()
                                    .map(|edge| edge.node2.position),
                            )
                            .collect()
                    };

                    step.pending = false;
                    step.position_vec = Some(position_vec);
                }
            }
        }
    }
}
