use crate::simulation::state::{navigation::Navigation, Action, Physics, Population, State, Time};

pub fn tick(state: &mut State) {
    let _ = tracing::info_span!("simulate_tick").entered();

    Action::tick(state);
    Population::tick(&state.world, &mut state.population);
    Physics::tick(&state.world, &state.physics, &mut state.population);
    Navigation::tick(&state.world, &mut state.navigation);
    Time::tick(&mut state.time);
}
