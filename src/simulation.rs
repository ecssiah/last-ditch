//! Simulation evolution

pub mod constants;
pub mod state;
pub mod supervisor;
pub mod utils;

use crate::simulation::{
    state::State,
    supervisor::{viewer::view::View, Message, Supervisor},
};
use tracing::instrument;

pub struct Simulation {
    pub supervisor: Supervisor,
    pub state: State,
}

impl Simulation {
    pub fn new(
        message_rx: crossbeam::channel::Receiver<Message>,
        view_input: triple_buffer::Input<View>,
    ) -> Self {
        let supervisor = Supervisor::new(message_rx, view_input);
        let state = State::new();

        Self { supervisor, state }
    }

    #[instrument(skip_all)]
    pub fn run(supervisor: &mut Supervisor, state: &mut State) {
        loop {
            Supervisor::start_timestep(supervisor);

            while Supervisor::has_work(&supervisor) {
                if Supervisor::tick(state, supervisor) == false {
                    return;
                }
            }

            Supervisor::fix_timestep(supervisor);
        }
    }
}
