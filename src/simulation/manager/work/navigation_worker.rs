use crate::simulation::{
    manager::work::{navigation_task::NavigationTask, worker::Worker},
    state::State,
};
use std::collections::VecDeque;

pub struct NavigationWorker {
    task_deque: VecDeque<NavigationTask>,
}

impl Worker for NavigationWorker {
    fn active(&self) -> bool {
        false
    }

    fn budget(&self) -> u32 {
        500
    }

    fn cost(&self) -> u32 {
        1
    }

    fn work(&mut self, state: &mut State) {
        if let Some(mut navigation_task) = self.task_deque.pop_front() {
            let done = NavigationTask::step(&mut state.navigation, &mut navigation_task);

            if !done {
                self.task_deque.push_back(navigation_task)
            }
        }
    }
    
}
