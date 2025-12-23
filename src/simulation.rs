//! Simulation evolution

pub mod constants;
pub mod manager;
pub mod state;
pub mod utils;

use crate::simulation::{
    manager::{viewer::view::View, Manager, Message},
    state::State,
};

pub struct Simulation {
    pub manager: Manager,
    pub state: State,
}

impl Simulation {
    pub fn new(
        message_rx: crossbeam::channel::Receiver<Message>,
        view_input: triple_buffer::Input<View>,
    ) -> Self {
        let manager = Manager::new(message_rx, view_input);
        let state = State::new();

        Self { manager, state }
    }

    pub fn run(manager: &mut Manager, state: &mut State) {
        loop {
            let _span = tracing::info_span!("simulation_loop").entered();

            Manager::start(manager);

            while Manager::has_work(&manager) {
                if !Manager::tick(state, manager) {
                    return;
                }
            }

            Manager::fix_timestep(manager);
        }
    }
}
