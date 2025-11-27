//! Simulation evolution

pub mod constants;
pub mod constructor;
pub mod manager;
pub mod state;
pub mod utils;
pub mod viewer;

use crate::simulation::{
    manager::{status::Status, Manager, Message},
    state::State,
    viewer::{View, Viewer},
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    pub manager: Manager,
    pub viewer: Viewer,
    pub state: State,
}

impl Simulation {
    pub fn new(
        message_rx: UnboundedReceiver<Message>,
        view_input: triple_buffer::Input<View>,
    ) -> Self {
        let manager = Manager::new(message_rx);
        let viewer = Viewer::new(view_input);
        let state = State::new();

        Self {
            manager,
            viewer,
            state,
        }
    }

    pub fn run(manager: &mut Manager, state: &mut State, viewer: &mut Viewer) {
        Manager::init(manager);

        loop {
            let _ = tracing::info_span!("simulation").entered();

            Manager::start(manager);

            while Manager::has_work(manager) {
                Manager::tick(state, manager);
                State::tick(state);
                Viewer::tick(state, viewer);

                if manager.status == Status::Done {
                    return;
                }
            }

            Manager::fix_timestep(manager);
        }
    }
}
