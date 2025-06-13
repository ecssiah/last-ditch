//! Current state of the simulation

use crate::simulation::{
    admin::{self, Admin},
    consts::*,
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
            world: Self::setup_world(),
            population: Population::new(),
        };

        state
    }

    fn setup_world() -> World {
        if TESTING {
            World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32)
        } else {
            World::new(MAIN_CHUNK_RADIUS as u32, MAIN_WORLD_RADIUS as u32)
        }
    }

    pub fn setup(&mut self) {
        self.world.setup();
        self.population.setup(&self.world);

        self.admin.mode = admin::Mode::Simulate;
        self.admin.message = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
    }

    pub fn tick(&mut self) {
        self.time.tick();

        self.world.tick(&self.time.tick);
        self.physics.tick(&self.world, &mut self.population);
        self.population.tick(&self.time.tick, &self.world);
    }
}
