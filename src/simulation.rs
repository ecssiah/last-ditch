//! Simulation evolution

pub mod config;
pub mod constants;
pub mod constructor;
pub mod kind;
pub mod state;
pub mod timing;
pub mod utils;
pub mod viewer;

pub use config::Config;
pub use kind::Kind;

use crate::simulation::{
    self,
    state::{receiver::Action, Receiver, State},
    timing::Timing,
    viewer::{view::View, Viewer},
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub kind: simulation::Kind,
    pub timing: Timing,
    pub receiver: Receiver,
    pub viewer: Viewer,
    pub state: State,
    pub view_buffer_input: triple_buffer::Input<View>,
}

impl Simulation {
    pub fn new(
        action_rx: UnboundedReceiver<Action>,
        view_buffer_input: triple_buffer::Input<View>,
    ) -> Self {
        let simulation_kind = simulation::Kind::Main;

        let timing = Timing::new();
        let receiver = Receiver::new(action_rx);
        let viewer = Viewer::new();
        let state = State::new(simulation_kind);

        Self {
            kind: simulation_kind,
            timing,
            receiver,
            viewer,
            state,
            view_buffer_input,
        }
    }

    pub fn run(
        timing: &mut Timing,
        receiver: &mut Receiver,
        viewer: &mut Viewer,
        state: &mut State,
        view_buffer_input: &mut triple_buffer::Input<View>,
    ) {
        Timing::init(timing);

        loop {
            let _simulation_span = tracing::info_span!("simulation").entered();

            Timing::start(timing);

            while Timing::has_work(timing) {
                State::tick(state);
                Viewer::tick(state, view_buffer_input, viewer);
                Timing::tick(timing);
                Receiver::tick(receiver, state);

                if receiver.is_off {
                    return;
                }
            }

            Timing::fix_timestep(timing);
        }
    }
}
