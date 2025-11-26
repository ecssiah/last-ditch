//! Current state of the simulation

pub mod admin;
pub mod navigation;
pub mod physics;
pub mod population;
pub mod receiver;
pub mod time;
pub mod world;

pub use admin::Admin;
pub use physics::Physics;
pub use population::Population;
pub use receiver::Receiver;
pub use time::Time;
pub use world::World;

use crate::simulation::{
    self, constants::PROJECT_TITLE, constructor, state::navigation::Navigation,
};

pub struct State {
    pub simulation_kind: simulation::Kind,
    pub construct_rx: Option<tokio::sync::mpsc::Receiver<(World, Population)>>,
    pub admin: Admin,
    pub time: Time,
    pub physics: Physics,
    pub world: World,
    pub population: Population,
    pub navigation: Navigation,
}

impl State {
    pub fn new(simulation_kind: simulation::Kind) -> Self {
        let construct_rx = None;

        let admin = Admin::new();
        let time = Time::new();
        let physics = Physics::new();
        let world = World::new(simulation_kind);
        let population = Population::new(simulation_kind);
        let navigation = Navigation::new();

        Self {
            simulation_kind,
            construct_rx,
            admin,
            time,
            physics,
            world,
            population,
            navigation,
        }
    }

    pub fn tick(state: &mut State) {
        let _state_span = tracing::info_span!("state_tick").entered();

        match state.admin.mode {
            admin::Mode::Menu => (),
            admin::Mode::Loading => Self::tick_loading(state),
            admin::Mode::Simulate => Self::tick_simulate(state),
            admin::Mode::Shutdown => Self::tick_shutdown(state),
        }
    }

    fn init_load(state: &mut State) {
        let simulation_kind = state.simulation_kind;

        let world = std::mem::replace(&mut state.world, World::placeholder());
        let population = std::mem::replace(&mut state.population, Population::placeholder());

        let (construct_tx, construct_rx) = tokio::sync::mpsc::channel(1);

        tokio::task::spawn_blocking(move || {
            let mut world = world;
            let mut population = population;

            constructor::world::run(simulation_kind, &mut world);
            constructor::population::run(simulation_kind, &world, &mut population);

            let _ = construct_tx.blocking_send((world, population));
        });

        state.construct_rx = Some(construct_rx);

        state.admin.mode = admin::Mode::Loading;
        state.admin.message = "Construction in Progress...".to_string();
    }

    fn init_shutdown(state: &mut State) {
        tracing::info!("Simulation Shutdown");

        state.admin.mode = admin::Mode::Shutdown;
    }

    fn tick_loading(state: &mut State) {
        if let Some(construct_rx) = &mut state.construct_rx {
            if let Ok((world, population)) = construct_rx.try_recv() {
                state.world = world;
                state.population = population;

                state.admin.mode = admin::Mode::Simulate;
                state.admin.message = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));
            }
        }
    }

    fn tick_simulate(state: &mut State) {
        let _simulate_span = tracing::info_span!("simulate_tick").entered();

        Time::tick(&mut state.time);
        Population::tick(&state.world, &mut state.population);
        Physics::tick(&state.world, &state.physics, &mut state.population);
        Navigation::tick(&state.world, &mut state.navigation);
    }

    fn tick_shutdown(state: &mut State) {
        state.admin.message = "Shutting down...".to_string();
    }
}
