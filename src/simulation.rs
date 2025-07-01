//! Evolution of the simulated environment.

pub mod config;
pub mod constructor;
pub mod consts;
pub mod kind;
pub mod observation;
pub mod state;
pub mod utils;

pub use config::Config;
pub use kind::Kind;

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
    pub kind: Kind,
    pub receiver: Receiver,
    pub state: State,
    pub observation_arc: Arc<Observation>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let kind = Kind::Main;

        let receiver = Receiver::new(action_rx);
        let state = State::new(kind);
        let observation_arc = Arc::new(Observation::new());

        Self {
            kind,
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
        self.state.setup();
    }

    fn execute(&mut self) {
        let mut next_instant = Instant::now() + SIMULATION_TICK_DURATION;

        loop {
            while Instant::now() >= next_instant {
                match self.receiver.listen() {
                    Some(action_vec) => {
                        self.state.tick(action_vec);
                        self.observation_arc.tick(&self.state);

                        next_instant += SIMULATION_TICK_DURATION;
                    }
                    None => return,
                }
            }

            self.fix_timestep(next_instant);
        }
    }

    fn fix_timestep(&self, next_instant: Instant) {
        let current_instant = Instant::now();

        if current_instant < next_instant {
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
