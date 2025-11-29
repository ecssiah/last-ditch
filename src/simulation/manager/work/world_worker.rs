use crate::simulation::{manager::work::world_task::WorldTask, state::State};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct WorldWorker {
    pub budget: u32,
    pub task_deque: VecDeque<WorldTask>,
}

impl WorldWorker {
    pub fn new(budget: u32) -> Self {
        Self {
            budget,
            task_deque: VecDeque::new(),
        }
    }

    pub fn enqueue(world_task: WorldTask, task_deque: &mut VecDeque<WorldTask>) {
        task_deque.push_back(world_task);
    }

    pub fn budget(world_worker: &WorldWorker) -> u32 {
        world_worker.budget
    }

    pub fn cost(world_worker: &WorldWorker) -> u32 {
        if let Some(world_task) = world_worker.task_deque.front() {
            WorldTask::cost(world_task)
        } else {
            0
        }
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
