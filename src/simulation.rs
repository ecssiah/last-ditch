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
        let kind = Kind::GraphTest;

        let timing = Timing::new();
        let receiver = Receiver::new(action_rx);
        let observation_arc = Arc::new(Observation::new());
        let state = State::new(kind);

        Self {
            kind,
            timing,
            receiver,
            observation_arc,
            state,
        }
    }

    pub fn run(
        timing: &mut Timing,
        receiver: &mut Receiver,
        observation_arc: Arc<Observation>,
        state: &mut State,
    ) {
        Self::execute(timing, receiver, observation_arc, state);
    }

    fn execute(
        timing: &mut Timing,
        receiver: &mut Receiver,
        observation_arc: Arc<Observation>,
        state: &mut State,
    ) {
        Timing::init(timing);

        loop {
            Timing::start_frame(timing);

            while Timing::has_work(timing) {
                match Receiver::listen(receiver) {
                    Some(action_vec) => {
                        State::tick(state, action_vec);
                        Observation::tick(observation_arc.clone(), state);

                        Timing::update_frame(timing);
                    }
                    None => {
                        log::info!("Simulation Exit");
                        return;
                    }
                }
            }

            Timing::fix_timestep(timing);
        }
    }
}
