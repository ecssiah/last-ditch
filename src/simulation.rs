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
        let mut next_instant = Instant::now();

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
