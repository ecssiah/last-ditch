//! Simulation evolution

pub mod config;
pub mod constants;
pub mod constructor;
pub mod kind;
pub mod observation;
pub mod state;
pub mod timing;
pub mod utils;

pub use config::Config;
pub use kind::Kind;
use tracing::{info, info_span};

use crate::simulation::{
    observation::{view::View, Observation},
    state::{receiver::action::Action, Receiver, State},
    timing::Timing,
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub kind: Kind,
    pub timing: Timing,
    pub receiver: Receiver,
    pub observation: Observation,
    pub state: State,
    pub view_buffer_input: triple_buffer::Input<View>,
}

impl Simulation {
    pub fn new(
        action_rx: UnboundedReceiver<Action>,
        view_buffer_input: triple_buffer::Input<View>,
    ) -> Self {
        let kind = Kind::Main;
        let timing = Timing::new();
        let receiver = Receiver::new(action_rx);
        let observation = Observation::new();
        let state = State::new(kind);

        Self {
            kind,
            timing,
            receiver,
            observation,
            state,
            view_buffer_input,
        }
    }

    pub fn run(
        timing: &mut Timing,
        receiver: &mut Receiver,
        observation: &mut Observation,
        state: &mut State,
        view_buffer_input: &mut triple_buffer::Input<View>,
    ) {
        Timing::init(timing);

        loop {
            let _simulation_span = info_span!("simulation").entered();

            Timing::start_frame(timing);

            while Timing::has_work(timing) {
                match Receiver::listen(receiver) {
                    Some(action_vec) => {
                        State::tick(action_vec, state);
                        Observation::tick(state, view_buffer_input, observation);

                        Timing::update_frame(timing);
                    }
                    None => {
                        tracing::info!("Simulation Exit");
                        return;
                    }
                }
            }

            Timing::fix_timestep(timing);
        }
    }
}
