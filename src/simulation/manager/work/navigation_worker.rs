use crate::simulation::{manager::work::navigation_task::NavigationTask, state::State};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct NavigationWorker {
    pub budget: u32,
    pub task_deque: VecDeque<NavigationTask>,
}

impl NavigationWorker {
    pub fn new(budget: u32) -> Self {
        Self {
            budget,
            task_deque: VecDeque::new(),
        }
    }

    pub fn enqueue(navigation_task: NavigationTask, task_deque: &mut VecDeque<NavigationTask>) {
        task_deque.push_back(navigation_task);
    }

    pub fn budget(navigation_worker: &NavigationWorker) -> u32 {
        navigation_worker.budget
    }

    pub fn cost(_navigation_worker: &NavigationWorker) -> u32 {
        1
    }

    pub fn work(state: &mut State, task_deque: &mut VecDeque<NavigationTask>) {
        if let Some(mut navigation_task) = task_deque.pop_front() {
            let done = NavigationTask::step(&mut state.navigation, &mut navigation_task);

            if !done {
                task_deque.push_back(navigation_task)
            }
        }
    }
}
