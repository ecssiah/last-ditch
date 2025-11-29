use crate::simulation::{manager::work::population_task::PopulationTask, state::State};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct PopulationWorker {
    pub task_deque: VecDeque<PopulationTask>,
}

impl PopulationWorker {
    pub fn new() -> Self {
        Self {
            task_deque: VecDeque::new(),
        }
    }

    pub fn enqueue(population_task: PopulationTask, task_deque: &mut VecDeque<PopulationTask>) {
        task_deque.push_back(population_task);
    }

    pub fn active(_population_worker: &PopulationWorker) -> bool {
        true
    }

    pub fn budget(_population_worker: &PopulationWorker) -> u32 {
        500
    }

    pub fn cost(_population_worker: &PopulationWorker) -> u32 {
        1
    }

    pub fn work(state: &mut State, task_deque: &mut VecDeque<PopulationTask>) {
        if let Some(mut population_task) = task_deque.pop_front() {
            let done =
                PopulationTask::step(&state.world, &mut state.population, &mut population_task);

            if !done {
                task_deque.push_back(population_task)
            }
        }
    }
}
