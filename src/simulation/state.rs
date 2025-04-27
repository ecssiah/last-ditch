use crate::simulation::{
    admin::{Admin, Mode},
    consts::*,
    population::Population,
    time::Time,
    world::World,
};

pub struct State {
    pub admin: Admin,
    pub time: Time,
    pub world: World,
    pub population: Population,
}

impl State {
    pub fn new() -> State {
        let state = State {
            admin: Admin::new(),
            time: Time::new(),
            world: World::new(),
            population: Population::new(),
        };

        state
    }

    pub fn generate(&mut self) {
        self.world.generate();
        self.population.generate();
    }

    pub fn tick(&mut self) {
        let tick = &self.time.tick;

        self.world.tick(tick);
        self.population.tick(tick, &self.world);

        self.time.tick();
    }
}
