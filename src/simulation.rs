//! Simulation evolution

pub mod constants;
pub mod manager;
pub mod state;
pub mod utils;

use crate::simulation::{
    manager::{Manager, Message, status::Status, viewer::View},
    state::State,
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub manager: Manager,
    pub state: State,
}

impl Simulation {
    pub fn new(
        message_rx: UnboundedReceiver<Message>,
        view_input: triple_buffer::Input<View>,
    ) -> Self {
        let manager = Manager::new(message_rx, view_input);
        let state = State::new();

        Self { manager, state }
    }

    pub fn run(manager: &mut Manager, state: &mut State) {
        Manager::init(manager);

        loop {
            let _ = tracing::info_span!("simulation_loop").entered();

            Manager::start(manager);

            while Manager::has_work(manager) {
                if !Manager::tick(state, manager) {
                    return;
                }
            }

            Manager::fix_timestep(manager);
        }
    }
}
