use crate::simulation::{
    constants::PROJECT_TITLE,
    constructor,
    state::{admin, navigation::Navigation, Population, State, World},
};

pub fn init(state: &mut State) {
    let state_template = state.template;

    let world = std::mem::replace(&mut state.world, World::placeholder());
    let population = std::mem::replace(&mut state.population, Population::placeholder());

    let (construct_tx, construct_rx) = tokio::sync::mpsc::channel(1);

    tokio::task::spawn_blocking(move || {
        let mut world = world;
        let mut population = population;

        constructor::world_template::construct(state_template, &mut world);
        constructor::population_template::construct(state_template, &world, &mut population);

        let _ = construct_tx.blocking_send((world, population));
    });

    state.construct_rx = Some(construct_rx);

    state.admin.mode = admin::Mode::Loading;
    state.admin.message = "Construction in Progress...".to_string();
}

pub fn tick(state: &mut State) {
    if let Some(construct_rx) = &mut state.construct_rx {
        if let Ok((world, population)) = construct_rx.try_recv() {
            state.world = world;
            state.population = population;

            Navigation::init_graph(&state.world, &mut state.navigation.graph);

            state.admin.mode = admin::Mode::Simulate;
            state.admin.message = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
        }
    }
}
