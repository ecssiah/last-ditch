use crate::simulation::state::{work::construct_task::ConstructTask, State};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct ConstructWorker {
    pub budget: u32,
    pub task_deque: VecDeque<ConstructTask>,
}

impl ConstructWorker {
    pub fn new(budget: u32) -> Self {
        Self {
            budget,
            task_deque: VecDeque::new(),
        }
    }

    pub fn enqueue(construct_task: ConstructTask, task_deque: &mut VecDeque<ConstructTask>) {
        task_deque.push_back(construct_task);
    }

    pub fn budget(construct_worker: &Self) -> u32 {
        construct_worker.budget
    }

    pub fn cost(construct_worker: &Self) -> u32 {
        if let Some(construct_task) = construct_worker.task_deque.front() {
            ConstructTask::cost(construct_task)
        } else {
            0
        }
    }

    pub fn work(state: &mut State) {
        if let Some(mut construct_task) = state.work.construct_worker.task_deque.pop_front() {
            let done = ConstructTask::step(state, &mut construct_task);

            if !done {
                state
                    .work
                    .construct_worker
                    .task_deque
                    .push_back(construct_task)
            }
        }
    }
}
