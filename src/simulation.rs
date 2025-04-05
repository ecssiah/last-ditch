//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod actions;
pub mod block;
pub mod chunk;
pub mod consts;
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

use crate::simulation::{actions::Actions, observation::Observation};
use actions::Action;
use physics::Physics;
use state::State;
use std::{
    sync::{Arc, RwLock},
    thread,
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    actions: Actions,
    state: State,
    physics: Physics,
    observation: Arc<RwLock<Observation>>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let actions = Actions::new(action_rx);
        let state = State::new();
        let physics = Physics::new();

        let observation = Arc::new(RwLock::new(Observation::new()));

        let simulation = Self {
            actions,
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

        self.setup_observation();

        loop {
            self.update();
        }
    }

    pub fn get_observation(&self) -> Arc<RwLock<Observation>> {
        Arc::clone(&self.observation)
    }

    fn setup_observation(&mut self) {
        let mut observation = self.observation.write().unwrap();

        observation.generate(&self.state);
    }

    fn update(&mut self) {
        self.state.time.calculate_work_time();

        while self.state.time.has_work_time() {
            self.actions.tick(&mut self.state);
            self.state.tick();
            self.physics.tick(&mut self.state);
            self.tick_observation();

            self.state.time.use_work_time();
        }

        thread::sleep(SIMULATION_WAIT_DURATION);
    }

    fn tick_observation(&mut self) {
        if let Ok(mut observation) = self.observation.write() {
            observation.tick(&self.state);
        } else {
            log::error!("Failed to acquire Observation write lock");
        }
    }
}
