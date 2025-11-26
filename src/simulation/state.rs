//! Current state of the simulation

pub mod action;
pub mod admin;
pub mod config;
pub mod navigation;
pub mod physics;
pub mod population;
pub mod receiver;
pub mod state_loading;
pub mod state_menu;
pub mod state_shutdown;
pub mod state_simulate;
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

use crate::simulation::state::{self, navigation::Navigation};

pub struct State {
    pub active: bool,
    pub template: state::Template,
    pub construct_rx: Option<tokio::sync::mpsc::Receiver<(World, Population)>>,
    pub admin: Admin,
    pub action: Action,
    pub world: World,
    pub population: Population,
    pub physics: Physics,
    pub navigation: Navigation,
    pub time: Time,
}

impl State {
    pub fn new() -> Self {
        let active = true;
        let template = state::Template::Main;
        let construct_rx = None;

        let admin = Admin::new();
        let action = Action::new();
        let world = World::new(template);
        let population = Population::new(template);
        let physics = Physics::new();
        let navigation = Navigation::new(&world.grid);
        let time = Time::new();

        Self {
            active,
            template,
            construct_rx,
            admin,
            action,
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
            admin::Mode::Menu => state_menu::tick(state),
            admin::Mode::Loading => state_loading::tick(state),
            admin::Mode::Simulate => state_simulate::tick(state),
            admin::Mode::Shutdown => state_shutdown::tick(state),
        }
    }
}
