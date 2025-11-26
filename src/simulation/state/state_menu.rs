use crate::simulation::state::{Action, State};

pub fn tick(state: &mut State) {
    Action::tick(state);
}