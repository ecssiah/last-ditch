use std::time::Instant;

use crate::simulation::{population::Population, time::Time, world::World, DEFAULT_SEED, FIXED_DT};

#[derive(Clone, PartialEq, Eq)]
pub enum Mode {
    Simulating,
    Exit,
}

pub struct State {
    pub seed: u64,
    pub mode: Mode,
    pub time: Time,
    pub population: Population,
    pub world: World,
}

impl State {
    pub fn new() -> State {
        let state = State {
            seed: DEFAULT_SEED,
            mode: Mode::Simulating,
            time: Time::new(),
            population: Population::new(),
            world: World::new(),
        };

        state
    }

    pub fn generate(&mut self) {
        self.population.generate();
        self.world.generate();
    }

    pub fn has_work_time(&self) -> bool {
        self.time.work_time >= FIXED_DT
    }

    pub fn calculate_work_time(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.time.previous);
        self.time.previous = now;

        self.time.work_time += frame_time;
    }

    pub fn tick(&mut self) {
        self.population.tick();
        self.world.tick();

        self.time.tick();
    }
}
