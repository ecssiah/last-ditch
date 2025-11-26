//! Simulation evolution

pub mod config;
pub mod constants;
pub mod constructor;
pub mod kind;
pub mod state;
pub mod timestep;
pub mod utils;
pub mod viewer;

pub use config::Config;
pub use kind::Kind;

use crate::simulation::{
    self,
    state::{receiver::Action, Receiver, State},
    timestep::Timestep,
    viewer::{view::View, Viewer},
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub kind: simulation::Kind,
    pub timestep: Timestep,
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

        let timestep = Timestep::new();
        let receiver = Receiver::new(action_rx);
        let viewer = Viewer::new();
        let state = State::new(simulation_kind);

        Self {
            kind: simulation_kind,
            timestep,
            receiver,
            viewer,
            state,
            view_buffer_input,
        }
    }

    pub fn run(
        timestep: &mut Timestep,
        receiver: &mut Receiver,
        viewer: &mut Viewer,
        state: &mut State,
        view_buffer_input: &mut triple_buffer::Input<View>,
    ) {
        Timestep::init(timestep);

        loop {
            let _simulation_span = tracing::info_span!("simulation").entered();

            Timestep::start(timestep);

            while Timestep::has_work(timestep) {
                State::tick(state);
                Viewer::tick(state, view_buffer_input, viewer);
                Receiver::tick(receiver, state);
                Timestep::tick(timestep);

                if receiver.is_off {
                    return;
                }
            }

            Timestep::fix(timestep);
        }
    }
}
