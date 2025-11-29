use crate::simulation::{
    manager::work::{population_task::PopulationTask, worker::Worker},
    state::State,
};
use std::collections::VecDeque;

pub struct PopulationWorker {
    task_deque: VecDeque<PopulationTask>,
}

impl Worker for PopulationWorker {
    fn active(&self) -> bool {
        false
    }

    fn cost(&self) -> u32 {
        1
    }

    fn work(&mut self, state: &mut State) {
        if let Some(mut population_task) = self.task_deque.pop_front() {
            let done = PopulationTask::step(&mut state.population, &mut population_task);

            if !done {
                self.task_deque.push_back(population_task)
            }
        }
    }
}
