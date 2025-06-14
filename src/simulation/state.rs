//! Current state of the simulation

use crate::simulation::{
    admin::{self, Admin},
    compute::Compute,
    consts::*,
    physics::Physics,
    population::Population,
    time::Time,
    world::World,
};

pub struct State {
    pub admin: Admin,
    pub compute: Compute,
    pub time: Time,
    pub physics: Physics,
    pub world: World,
    pub population: Population,
}

impl State {
    pub fn new() -> Self {
        let admin = Admin::new();
        let compute = Compute::new();
        let time = Time::new();
        let physics = Physics::new();
        let world = Self::setup_world();
        let population = Population::new(compute.task_tx.clone(), compute.result_rx.clone());

        Self {
            admin,
            compute,
            time,
            physics,
            world,
            population,
        }
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
        self.population.tick(&self.time, &self.world);

        self.compute.tick(&self.world);
    }
}
