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
use std::{
    sync::{Arc, RwLock},
    thread,
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    actions: Dispatch,
    state: State,
    physics: Physics,
    observation_lock: Arc<RwLock<Observation>>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let actions = Dispatch::new(action_rx);
        let state = State::new();
        let physics = Physics::new();

        let observation_lock = Arc::new(RwLock::new(Observation::new()));

        let simulation = Self {
            actions,
            state,
            physics,
            observation_lock,
        };

        log::info!("Simulation Initialized");

        simulation
    }

    pub fn run(&mut self) {
        self.state.generate();
        self.physics.generate(&self.state);

        self.generate_view();

        loop {
            self.update();
        }
    }

    fn generate_view(&self) {
        let observation = self.observation_lock.read().unwrap();

        observation.generate_view(&self.state);
    }

    pub fn get_observation_arc(&self) -> Arc<RwLock<Observation>> {
        Arc::clone(&self.observation_lock)
    }

    fn update(&mut self) {
        self.state.calculate_work();

        while self.state.has_work() {
            self.actions.tick(&mut self.state);

            self.state.tick();
            self.physics.tick(&mut self.state);

            self.tick_observation();
        }

        thread::sleep(SIMULATION_WAIT_DURATION);
    }

    fn tick_observation(&mut self) {
        let mut observation = self.observation_lock.write().unwrap();

        observation.tick(&self.state);
    }
}
