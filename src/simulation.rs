//! Evolution of the simulated environment.

pub mod admin;
pub mod compute;
pub mod consts;
pub mod dispatch;
pub mod observation;
pub mod physics;
pub mod population;
pub mod state;
pub mod time;
pub mod world;

use crate::simulation::{
    consts::SIMULATION_TICK_DURATION, dispatch::Dispatch, observation::Observation,
};
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

        Self {
            dispatch,
            state,
            observation,
        }
    }

    pub fn run(&mut self) {
        self.setup();

        let mut next_instant = Instant::now();

        loop {
            let current_instant = Instant::now();

            while current_instant >= next_instant {
                self.dispatch.tick(&mut self.state);
                self.state.tick();
                self.observation.tick(&self.state);

                next_instant += SIMULATION_TICK_DURATION;
            }

            let current_instant = Instant::now();

            self.fix_timestep(current_instant, next_instant);
        }
    }

    fn setup(&mut self) {
        self.observation.tick(&self.state);

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

    pub fn get_observation(&self) -> Arc<Observation> {
        self.observation.clone()
    }

    pub fn get_action_tx(&self) -> Arc<UnboundedSender<Action>> {
        self.dispatch.get_action_tx()
    }
}
