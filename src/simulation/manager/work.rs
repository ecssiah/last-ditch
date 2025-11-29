use crate::simulation::{
    manager::work::{
        navigation_worker::NavigationWorker, population_worker::PopulationWorker,
        world_worker::WorldWorker,
    },
    state::State,
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

    pub fn tick(state: &mut State, work: &mut Work) {
        Self::perform_world_work(state, &mut work.world_worker);
        Self::perform_population_work(state, &mut work.population_worker);
        Self::perform_navigation_work(state, &mut work.navigation_worker);
    }

    fn perform_world_work(state: &mut State, world_worker: &mut WorldWorker) {
        let mut current_budget = WorldWorker::budget(world_worker);

        while current_budget > 0 {
            let cost = WorldWorker::cost(world_worker);

            if cost == 0 || current_budget < cost {
                break;
            }

            WorldWorker::work(state, &mut world_worker.task_deque);
            current_budget -= cost;
        }
    }

    fn perform_population_work(state: &mut State, population_worker: &mut PopulationWorker) {
        let mut current_budget = PopulationWorker::budget(&population_worker);

        while current_budget > 0 {
            let cost = PopulationWorker::cost(&population_worker);

            if cost == 0 || current_budget < cost {
                break;
            }

            PopulationWorker::work(state, &mut population_worker.task_deque);
            current_budget -= cost;
        }
    }

    fn perform_navigation_work(state: &mut State, navigation_worker: &mut NavigationWorker) {
        let mut current_budget = NavigationWorker::budget(&navigation_worker);

        while current_budget > 0 {
            let cost = NavigationWorker::cost(&navigation_worker);

            if cost == 0 || current_budget < cost {
                break;
            }

            NavigationWorker::work(state, &mut navigation_worker.task_deque);
            current_budget -= cost;
        }
    }
}
