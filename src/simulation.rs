//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod admin;
pub mod block;
pub mod chunk;
pub mod consts;
pub mod dispatch;
pub mod observation;
pub mod physics;
pub mod population;
pub mod state;
pub mod structure;
pub mod time;
pub mod world;

pub use block::Block;
pub use chunk::Chunk;
pub use consts::*;

use crate::simulation::{dispatch::Dispatch, observation::Observation};
use dispatch::Action;
use physics::Physics;
use state::State;
use std::{sync::Arc, thread, time::Instant};
use tokio::sync::mpsc::UnboundedSender;

pub struct Simulation {
    dispatch: Dispatch,
    state: State,
    physics: Physics,
    observation: Arc<Observation>,
}

impl Simulation {
    pub fn new() -> Self {
        let dispatch = Dispatch::new();
        let state = State::new();
        let physics = Physics::new();

        let observation = Arc::new(Observation::new());

        let simulation = Self {
            dispatch,
            state,
            physics,
            observation,
        };

        log::info!("Simulation Initialized");

        simulation
    }

    pub fn run(&mut self) {
        self.state.generate();
        self.physics.generate(&self.state);

        self.state.admin.mode = admin::Mode::Simulate;

        log::info!("Simulation Beginning...");

        loop {
            let start = Instant::now();

            self.dispatch.tick(&mut self.state);

            self.state.tick();
            self.physics.tick(&mut self.state);

            self.observation.tick(&self.state);

            let elapsed = start.elapsed();

            if elapsed < SIMULATION_TICK_RATE {
                std::thread::sleep(SIMULATION_TICK_RATE - elapsed);
            } else {
                log::warn!("Simulation tick overran: {:?}", elapsed);
            }
        }
    }

    pub fn get_observation(&self) -> Arc<Observation> {
        self.observation.clone()
    }

    pub fn get_action_tx(&self) -> Arc<UnboundedSender<Action>> {
        self.dispatch.get_action_tx()
    }

    fn update(&mut self) {
        self.state.calculate_work();

        while self.state.has_work() {}

        thread::sleep(SIMULATION_WAIT_DURATION);
    }
}
