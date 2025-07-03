//! Evolution of the simulated environment.

pub mod config;
pub mod constructor;
pub mod consts;
pub mod kind;
pub mod observation;
pub mod state;
pub mod timing;
pub mod utils;

pub use config::Config;
pub use kind::Kind;

use crate::simulation::{
    observation::Observation,
    state::{receiver::action::Action, Receiver, State},
    timing::Timing,
};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub kind: Kind,
    pub timing: Timing,
    pub receiver: Receiver,
    pub state: State,
    pub observation_arc: Arc<Observation>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let kind = Kind::Main;

        let timing = Timing::new();
        let receiver = Receiver::new(action_rx);
        let state = State::new(kind);
        let observation_arc = Arc::new(Observation::new());

        Self {
            kind,
            timing,
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
        self.timing.init();

        loop {
            self.timing.start_frame();

            while self.timing.has_work() {
                match self.receiver.listen() {
                    Some(action_vec) => {
                        self.state.tick(action_vec);
                        self.observation_arc.tick(&self.state);

                        self.timing.update_frame();
                    }
                    None => return,
                }
            }

            self.timing.fix_timestep();
        }
    }
}
