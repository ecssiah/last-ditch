//! Evolution of the simulated environment.

pub mod config;
pub mod constructor;
pub mod consts;
pub mod mode;
pub mod observation;
pub mod state;
pub mod utils;

pub use config::Config;
pub use mode::Mode;

use crate::simulation::{
    consts::*,
    observation::Observation,
    state::{receiver::action::Action, Receiver},
};
use state::State;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub mode: Mode,
    pub receiver: Receiver,
    pub state: State,
    pub observation_arc: Arc<Observation>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let mode = Mode::GraphTest;

        let receiver = Receiver::new(action_rx);
        let state = State::new(mode);
        let observation_arc = Arc::new(Observation::new());

        Self {
            mode,
            receiver,
            state,
            observation_arc,
        }
    }

    pub fn get_observation_arc(&self) -> Arc<Observation> {
        self.observation_arc.clone()
    }

    pub fn run(&mut self) {
        self.setup();
        self.execute();
    }

    fn setup(&mut self) {
        // FIXME: currently required to display "Loading World"
        self.observation_arc.tick(&self.state);

        self.state.setup();
    }

    fn execute(&mut self) {
        let mut next_instant = Instant::now() + SIMULATION_TICK_DURATION;

        loop {
            while Instant::now() >= next_instant {
                self.receiver.tick(&mut self.state);
                self.state.tick();
                self.observation_arc.tick(&self.state);

                next_instant += SIMULATION_TICK_DURATION;
            }

            self.fix_timestep(next_instant);
        }
    }

    fn fix_timestep(&self, next_instant: Instant) {
        let current_instant = Instant::now();

        if next_instant > current_instant {
            let remaining_duration = next_instant - current_instant;

            if remaining_duration > Duration::from_millis(2) {
                std::thread::sleep(remaining_duration - Duration::from_millis(1));
            }

            while Instant::now() < next_instant {
                std::hint::spin_loop();
            }
        }
    }
}
