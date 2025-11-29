use crate::simulation::{
    manager::work::{worker::Worker, world_task::WorldTask},
    state::State,
};
use std::collections::VecDeque;

pub struct WorldWorker {
    task_deque: VecDeque<WorldTask>,
}

impl Worker for WorldWorker {
    fn active(&self) -> bool {
        false
    }

    fn cost(&self) -> u32 {
        1
    }

    fn work(&mut self, state: &mut State) {
        if let Some(mut world_task) = self.task_deque.pop_front() {
            let done = WorldTask::step(&mut state.world, &mut world_task);

            if !done {
                self.task_deque.push_back(world_task)
            }
        }
    }
}
