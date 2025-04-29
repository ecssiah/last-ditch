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
        self.state.generate();

        log::info!("Simulation Run");

        let mut next_tick = Instant::now();

        loop {
            let now = Instant::now();

            while now >= next_tick {
                self.dispatch.tick(&mut self.state);
                self.state.tick();
                self.observation.tick(&self.state);

                next_tick += SIMULATION_TICK_DURATION;
            }

            let now = Instant::now();

            if next_tick > now {
                let time_until_next = next_tick - now;

                if time_until_next > Duration::from_millis(2) {
                    std::thread::sleep(time_until_next - Duration::from_millis(1));
                }

                while Instant::now() < next_tick {
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
