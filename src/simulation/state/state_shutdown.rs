use crate::simulation::state::{admin, Action, State};

pub fn init(state: &mut State) {
    tracing::info!("Simulation Shutdown");

    state.admin.mode = admin::Mode::Shutdown;
}

pub fn tick(state: &mut State) {
    Action::tick(state);

    state.admin.message = "Shutting down...".to_string();
}
