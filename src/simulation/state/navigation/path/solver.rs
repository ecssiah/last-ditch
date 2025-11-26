use crate::simulation::state::navigation::path;

pub struct Solver {}

impl Solver {
    pub const EXPANSION_BUDGET: usize = 500;

    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(path_task_vec: &mut Vec<path::Task>) {
        let mut working_budget = Self::EXPANSION_BUDGET;

        for task in path_task_vec.iter_mut() {
            if working_budget > 0 && !task.finished {
                working_budget -= 1;
            }

            if working_budget == 0 {
                break;
            }
        }

        path_task_vec.retain(|task| !task.finished);
    }
}
