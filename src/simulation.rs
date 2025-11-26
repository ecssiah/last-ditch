//! Simulation evolution

pub mod constants;
pub mod constructor;
pub mod state;
pub mod timestep;
pub mod utils;
pub mod viewer;

use crate::simulation::{
    state::{Receiver, State, action::act::Act},
    timestep::Timestep,
    viewer::{View, Viewer},
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub timestep: Timestep,
    pub receiver: Receiver,
    pub viewer: Viewer,
    pub state: State,
    pub view_buffer_input: triple_buffer::Input<View>,
}

impl Simulation {
    pub fn new(
        act_rx: UnboundedReceiver<Act>,
        view_buffer_input: triple_buffer::Input<View>,
    ) -> Self {
        let timestep = Timestep::new();
        let receiver = Receiver::new(act_rx);
        let viewer = Viewer::new();
        let state = State::new();

        Self {
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
                Receiver::tick(receiver, &mut state.action);
                State::tick(state);
                Viewer::tick(state, view_buffer_input, viewer);
                Timestep::tick(timestep);

                if !state.active {
                    return;
                }
            }

            Timestep::fix(timestep);
        }
    }
}
