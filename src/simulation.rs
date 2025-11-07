//! Simulation evolution

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
    observation::{view::View, Observation},
    state::{receiver::action::Action, Receiver, State},
    timing::Timing,
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub kind: Kind,
    pub timing: Timing,
    pub receiver: Receiver,
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
        let state = State::new(kind);

        Self {
            kind,
            timing,
            receiver,
            state,
            view_buffer_input,
        }
    }

    pub fn run(
        timing: &mut Timing,
        receiver: &mut Receiver,
        state: &mut State,
        view_buffer_input: &mut triple_buffer::Input<View>,
    ) {
        Timing::init(timing);

        loop {
            Timing::start_frame(timing);

            while Timing::has_work(timing) {
                match Receiver::listen(receiver) {
                    Some(action_vec) => {
                        State::tick(action_vec, state);
                        Observation::tick(state, view_buffer_input);

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
