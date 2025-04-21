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
use std::{sync::Arc, thread};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    dispatch: Dispatch,
    state: State,
    physics: Physics,
    observation: Arc<Observation>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let dispatch = Dispatch::new(action_rx);
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

        loop {
            self.update();
        }
    }

    pub fn get_observation(&self) -> Arc<Observation> {
        self.observation.clone()
    }

    fn update(&mut self) {
        self.state.calculate_work();

        while self.state.has_work() {
            self.dispatch.tick(&mut self.state);

            self.state.tick();
            self.physics.tick(&mut self.state);

            self.observation.tick(&self.state);
        }

        thread::sleep(SIMULATION_WAIT_DURATION);
    }
}
