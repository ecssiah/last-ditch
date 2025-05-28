//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

use crate::simulation::{
    admin::{self, Admin},
    physics::Physics,
    population::Population,
    time::Time,
    world::World,
};

pub struct State {
    pub admin: Admin,
    pub time: Time,
    pub physics: Physics,
    pub world: World,
    pub population: Population,
}

impl State {
    pub fn new() -> State {
        let state = State {
            admin: Admin::new(),
            time: Time::new(),
            physics: Physics::new(),
            world: World::new(),
            population: Population::new(),
        };

        state
    }

    pub fn setup(&mut self) {
        self.world.setup();
        self.population.setup(&self.world);

        self.admin.mode = admin::Mode::Simulate;
        self.admin.message = String::from("Last Ditch 1.0");
    }

    pub fn tick(&mut self) {
        let tick = &self.time.tick;

        self.world.tick(tick);
        self.population.tick(tick, &self.world);
        self.physics.tick(&self.world, &mut self.population);
        
        self.time.tick();
    }
}
