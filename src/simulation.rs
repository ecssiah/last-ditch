//! Simulation evolution

pub mod constants;
pub mod constructor;
pub mod manager;
pub mod state;
pub mod utils;
pub mod viewer;

use crate::simulation::{
    manager::Manager,
    state::{action::act::Act, State},
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
        act_rx: UnboundedReceiver<Act>,
        view_buffer_input: triple_buffer::Input<View>,
    ) -> Self {
        let manager = Manager::new(act_rx);
        let viewer = Viewer::new(view_buffer_input);
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

                if !manager.active {
                    return;
                }
            }

            Manager::fix_timestep(manager);
        }
    }
}
