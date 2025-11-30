use crate::simulation::state::{
    work::{
        navigation_worker::NavigationWorker, population_worker::PopulationWorker,
        world_worker::WorldWorker,
    },
    State,
};

pub mod navigation_task;
pub mod navigation_worker;
pub mod population_task;
pub mod population_worker;
pub mod world_task;
pub mod world_worker;

pub struct Work {
    pub world_worker: WorldWorker,
    pub population_worker: PopulationWorker,
    pub navigation_worker: NavigationWorker,
}

impl Work {
    pub fn new() -> Self {
        let work = Self {
            world_worker: WorldWorker::new(100),
            population_worker: PopulationWorker::new(100),
            navigation_worker: NavigationWorker::new(100),
        };

        work
    }

    pub fn tick(state: &mut State) {
        Self::perform_world_work(state);
        Self::perform_population_work(state);
        Self::perform_navigation_work(state);
    }

    fn perform_world_work(state: &mut State) {
        let mut current_budget = WorldWorker::budget(&state.work.world_worker);

        while current_budget > 0 {
            let cost = WorldWorker::cost(&state.work.world_worker);

            if cost == 0 || current_budget < cost {
                break;
            }

            WorldWorker::work(state);
            current_budget -= cost;
        }
    }

    fn perform_population_work(state: &mut State) {
        let mut current_budget = PopulationWorker::budget(&state.work.population_worker);

        while current_budget > 0 {
            let cost = PopulationWorker::cost(&state.work.population_worker);

            if cost == 0 || current_budget < cost {
                break;
            }

            PopulationWorker::work(state);
            current_budget -= cost;
        }
    }

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
