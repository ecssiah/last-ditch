//! Evolution of the simulated environment.

pub mod consts;
pub mod observation;
pub mod state;
pub mod utils;

use crate::simulation::{
    consts::SIMULATION_TICK_DURATION,
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
    pub receiver: Receiver,
    pub state: State,
    pub observation_arc: Arc<Observation>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let receiver = Receiver::new(action_rx);
        let state = State::new();
        let observation_arc = Arc::new(Observation::new());

        Self {
            receiver,
            state,
            observation_arc,
        }
    }

    pub fn run(&mut self) {
        self.setup();

        let mut next_instant = Instant::now();

        loop {
            let current_instant = Instant::now();

            while current_instant >= next_instant {
                self.receiver.tick(&mut self.state);
                self.state.tick();
                self.observation_arc.tick(&self.state);

                next_instant += SIMULATION_TICK_DURATION;
            }

            let current_instant = Instant::now();

            self.fix_timestep(current_instant, next_instant);
        }
    }

    fn setup(&mut self) {
        // self.observation_arc.tick(&self.state);

        self.state.setup();
    }

    fn fix_timestep(&self, current_instant: Instant, next_instant: Instant) {
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

    pub fn get_observation_arc(&self) -> Arc<Observation> {
        self.observation_arc.clone()
    }
}
