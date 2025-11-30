use std::collections::VecDeque;
use crate::simulation::state::{work::population_task::PopulationTask, State};

#[derive(Clone)]
pub struct PopulationWorker {
    pub budget: u32,
    pub task_deque: VecDeque<PopulationTask>,
}

impl PopulationWorker {
    pub fn new(budget: u32) -> Self {
        Self {
            budget,
            task_deque: VecDeque::new(),
        }
    }

    pub fn enqueue(population_task: PopulationTask, task_deque: &mut VecDeque<PopulationTask>) {
        task_deque.push_back(population_task);
    }

    pub fn budget(population_worker: &PopulationWorker) -> u32 {
        population_worker.budget
    }

    pub fn cost(population_worker: &PopulationWorker) -> u32 {
        if let Some(population_task) = population_worker.task_deque.front() {
            PopulationTask::cost(population_task)
        } else {
            0
        }
    }

    pub fn work(state: &mut State) {
        if let Some(mut population_task) = state.work.population_worker.task_deque.pop_front() {
            let done =
                PopulationTask::step(&state.world, &mut state.population, &mut population_task);

            if !done {
                state
                    .work
                    .population_worker
                    .task_deque
                    .push_back(population_task)
            }
        }
    }
}
