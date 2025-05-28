//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod admin;
pub mod consts;
pub mod dispatch;
pub mod observation;
pub mod physics;
pub mod population;
pub mod state;
pub mod time;
pub mod world;

pub use consts::*;

use crate::simulation::{dispatch::Dispatch, observation::Observation};
use dispatch::Action;
use state::State;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;

pub struct Simulation {
    dispatch: Dispatch,
    state: State,
    observation: Arc<Observation>,
}

impl Simulation {
    pub fn new() -> Self {
        let dispatch = Dispatch::new();
        let state = State::new();
        let observation = Arc::new(Observation::new());

        let simulation = Self {
            dispatch,
            state,
            observation,
        };

        log::info!("Simulation Initialized");

        simulation
    }

    pub fn run(&mut self) {
        self.observation.tick(&self.state);
        self.state.setup();

        log::info!("Simulation Run");

        let mut next_instant = Instant::now();

        loop {
            let now = Instant::now();

            while now >= next_instant {
                self.dispatch.tick(&mut self.state);
                self.state.tick();
                self.observation.tick(&self.state);

                next_instant += SIMULATION_TICK_DURATION;
            }

            let now = Instant::now();

            if next_instant > now {
                let remaining_duration = next_instant - now;

                if remaining_duration > Duration::from_millis(2) {
                    std::thread::sleep(remaining_duration - Duration::from_millis(1));
                }

                while Instant::now() < next_instant {
                    std::hint::spin_loop();
                }
            }
        }
    }

    pub fn get_observation(&self) -> Arc<Observation> {
        self.observation.clone()
    }

    pub fn get_action_tx(&self) -> Arc<UnboundedSender<Action>> {
        self.dispatch.get_action_tx()
    }
}
