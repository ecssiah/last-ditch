//! Current state of the simulation

pub mod admin;
pub mod compute;
pub mod physics;
pub mod population;
pub mod receiver;
pub mod time;
pub mod world;

pub use admin::Admin;
pub use compute::Compute;
pub use physics::Physics;
pub use population::Population;
pub use receiver::Receiver;
pub use time::Time;
pub use world::World;

use crate::simulation::{self};

pub struct State {
    pub admin: Admin,
    pub compute: Compute,
    pub time: Time,
    pub physics: Physics,
    pub world: World,
    pub population: Population,
}

impl State {
    pub fn new(mode: simulation::Mode) -> Self {
        let admin = Admin::new();
        let compute = Compute::new();
        let time = Time::new();
        let physics = Physics::new();
        let world = World::new(mode);
        let population = Population::new(mode, &compute);

        Self {
            admin,
            compute,
            time,
            physics,
            world,
            population,
        }
    }

    pub fn setup(&mut self) {
        self.admin.setup();
        self.world.setup();
        self.population.setup(&self.world);
    }

    pub fn tick(&mut self) {
        self.admin.tick();
        self.time.tick();
        self.world.tick();
        self.population.tick(&self.world);
        self.physics.tick(&self.world, &mut self.population);
        self.compute.tick(&self.world, &self.population);
    }
}
