//! Simulation evolution

pub mod constants;
pub mod overseer;
pub mod state;
pub mod utils;

use crate::simulation::{
    overseer::{viewer::view::View, Overseer, Message},
    state::State,
};
use tracing::instrument;

pub struct Simulation {
    pub overseer: Overseer,
    pub state: State,
}

impl Simulation {
    pub fn new(
        message_rx: crossbeam::channel::Receiver<Message>,
        view_input: triple_buffer::Input<View>,
    ) -> Self {
        let overseer = Overseer::new(message_rx, view_input);
        let state = State::new();

        Self { overseer, state }
    }

    #[instrument(skip_all)]
    pub fn run(overseer: &mut Overseer, state: &mut State) {
        loop {
            Overseer::start(overseer);

            while Overseer::has_work(&overseer) {
                if Overseer::tick(state, overseer) == false {
                    return;
                }
            }

            Overseer::fix_timestep(overseer);
        }
    }
}
