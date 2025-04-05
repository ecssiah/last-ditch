use crate::simulation::{population::Population, time::Time, world::World, DEFAULT_SEED};

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

    pub fn calculate_work(&mut self) {
        self.time.calculate_work();
    }

    pub fn has_work(&self) -> bool {
        self.time.has_work()
    }

    pub fn generate(&mut self) {
        self.population.generate();
        self.world.generate();
    }

    pub fn tick(&mut self) {
        self.population.tick();
        self.world.tick();

        self.time.tick();
    }
}
