use crate::simulation::{manager::work::world_task::WorldTask, state::State};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct WorldWorker {
    pub task_deque: VecDeque<WorldTask>,
}

impl WorldWorker {
    pub fn new() -> Self {
        Self {
            task_deque: VecDeque::new(),
        }
    }

    pub fn enqueue(world_task: WorldTask, task_deque: &mut VecDeque<WorldTask>) {
        task_deque.push_back(world_task);
    }

    pub fn active(_world_worker: &WorldWorker) -> bool {
        true
    }

    pub fn budget(_world_worker: &WorldWorker) -> u32 {
        500
    }

    pub fn cost(_world_worker: &WorldWorker) -> u32 {
        1
    }

    pub fn work(state: &mut State, task_deque: &mut VecDeque<WorldTask>) {
        if let Some(mut world_task) = task_deque.pop_front() {
            let done = WorldTask::step(&mut state.world, &mut world_task);

            if !done {
                task_deque.push_back(world_task)
            }
        }
    }
}
