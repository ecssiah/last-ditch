//! Current state of the simulation

pub mod action;
pub mod admin;
pub mod config;
pub mod navigation;
pub mod physics;
pub mod population;
pub mod receiver;
pub mod template;
pub mod time;
pub mod world;

pub use action::Action;
pub use admin::Admin;
pub use config::Config;
pub use physics::Physics;
pub use population::Population;
pub use receiver::Receiver;
pub use template::Template;
pub use time::Time;
pub use world::World;

use crate::simulation::{
    constants::PROJECT_TITLE,
    constructor,
    state::{self, navigation::Navigation},
};
use std::collections::VecDeque;

pub struct State {
    pub template: state::Template,
    pub construct_rx: Option<tokio::sync::mpsc::Receiver<(World, Population)>>,
    pub action_deque: VecDeque<state::Action>,
    pub admin: Admin,
    pub world: World,
    pub population: Population,
    pub physics: Physics,
    pub navigation: Navigation,
    pub time: Time,
}

impl State {
    pub fn new() -> Self {
        let template = state::Template::Main;
        let construct_rx = None;
        let action_deque = VecDeque::new();

        let admin = Admin::new();
        let world = World::new(template);
        let population = Population::new(template);
        let physics = Physics::new();
        let navigation = Navigation::new(&world.grid);
        let time = Time::new();

        Self {
            template,
            construct_rx,
            action_deque,
            admin,
            time,
            physics,
            world,
            population,
            navigation,
        }
    }

    pub fn tick(state: &mut State) {
        let _ = tracing::info_span!("state_tick").entered();

        match state.admin.mode {
            admin::Mode::Menu => (),
            admin::Mode::Loading => Self::tick_loading(state),
            admin::Mode::Simulate => Self::tick_simulate(state),
            admin::Mode::Shutdown => Self::tick_shutdown(state),
        }
    }

    fn init_load(state: &mut State) {
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

    fn init_shutdown(state: &mut State) {
        tracing::info!("Simulation Shutdown");

        state.admin.mode = admin::Mode::Shutdown;
    }

    fn tick_loading(state: &mut State) {
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

    fn tick_simulate(state: &mut State) {
        let _ = tracing::info_span!("simulate_tick").entered();

        Population::tick(&state.world, &mut state.population);
        Physics::tick(&state.world, &state.physics, &mut state.population);
        Navigation::tick(&state.world, &mut state.navigation);
        Time::tick(&mut state.time);
    }

    fn tick_shutdown(state: &mut State) {
        state.admin.message = "Shutting down...".to_string();
    }
}
