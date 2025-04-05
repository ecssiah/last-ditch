use crate::simulation::{population::Population, time::Time, world::World, DEFAULT_SEED};

pub struct State {
    pub active: bool,
    pub seed: u64,
    pub time: Time,
    pub population: Population,
    pub world: World,
}

impl State {
    pub fn new() -> State {
        let state = State {
            active: true,
            seed: DEFAULT_SEED,
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

    pub fn tick(&mut self) {
        self.time.tick();
        self.population.tick(&self.time.tick);
        self.world.tick(&self.time.tick);
    }
}
