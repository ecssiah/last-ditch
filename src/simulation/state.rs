//! Current state of the simulation

pub mod action;
pub mod admin;
pub mod config;
pub mod navigation;
pub mod physics;
pub mod population;
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
pub use template::Template;
pub use time::Time;
pub use world::World;

use crate::simulation::state::{
    self,
    navigation::{Graph, Navigation},
    population::sight::Sight,
    world::block,
};

pub struct State {
    pub template: state::Template,
    pub construct_rx: Option<tokio::sync::mpsc::Receiver<(World, Population)>>,
    pub admin: Admin,
    pub time: Time,
    pub action: Action,
    pub world: World,
    pub population: Population,
    pub physics: Physics,
    pub navigation: Navigation,
}

impl State {
    pub fn new() -> Self {
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

    pub fn place_block(block_kind: block::Kind, state: &mut State) {
        let judge = &state.population.judge;

        let range = 8.0;
        let origin = judge.sight.world_position;
        let direction = Sight::get_forward(&judge.sight);

        if let Some((hit_position, normal)) =
            World::raycast_to_block(origin, direction, range, &state.world)
        {
            let placement_position = hit_position + normal;

            World::set_block(
                placement_position,
                judge.selected_block,
                &state.world.block_info_map,
                &state.world.grid,
                &mut state.world.sector_vec,
            );

            let block_info = state.world.block_info_map[&block_kind];

            Graph::set_solid(
                placement_position,
                block_info.solid,
                &mut state.navigation.graph,
            );
        }
    }

    pub fn remove_block(state: &mut State) {
        let judge = &state.population.judge;

        let range = 8.0;
        let origin = judge.sight.world_position;
        let direction = Sight::get_forward(&judge.sight);

        if let Some((hit_position, _)) =
            World::raycast_to_block(origin, direction, range, &state.world)
        {
            World::set_block(
                hit_position,
                block::Kind::None,
                &state.world.block_info_map,
                &state.world.grid,
                &mut state.world.sector_vec,
            );

            Graph::set_solid(hit_position, false, &mut state.navigation.graph);
        }
    }
}
