use tracing::instrument;

use crate::simulation::state::{
    work::{construct_worker::ConstructWorker, navigation_worker::NavigationWorker},
    State,
};

pub mod construct_task;
pub mod construct_worker;
pub mod navigation_task;
pub mod navigation_worker;

pub struct Work {
    pub construct_worker: ConstructWorker,
    pub navigation_worker: NavigationWorker,
}

impl Work {
    pub fn new() -> Self {
        let work = Self {
            construct_worker: ConstructWorker::new(100),
            navigation_worker: NavigationWorker::new(100),
        };

        work
    }

    #[instrument(skip_all, name = "tick")]
    pub fn tick(state: &mut State) {
        Self::perform_world_work(state);
        Self::perform_navigation_work(state);
    }

    #[instrument(skip_all, name = "perform_world_work")]
    fn perform_world_work(state: &mut State) {
        let mut current_budget = ConstructWorker::budget(&state.work.construct_worker);

        while current_budget > 0 {
            let cost = ConstructWorker::cost(&state.work.construct_worker);

            if cost == 0 || current_budget < cost {
                break;
            }

            ConstructWorker::work(state);
            current_budget -= cost;
        }
    }

    #[instrument(skip_all, name = "perform_navigation_work")]
    fn perform_navigation_work(state: &mut State) {
        let mut current_budget = NavigationWorker::budget(&state.work.navigation_worker);

        while current_budget > 0 {
            let cost = NavigationWorker::cost(&state.work.navigation_worker);

            if cost == 0 || current_budget < cost {
                break;
            }

            NavigationWorker::work(state);
            current_budget -= cost;
        }
    }
}
