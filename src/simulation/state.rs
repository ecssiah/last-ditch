//! Current state of the simulation

pub mod action;
pub mod config;
pub mod navigation;
pub mod physics;
pub mod population;
pub mod template;
pub mod time;
pub mod world;

pub use action::Action;
pub use config::Config;
pub use physics::Physics;
pub use population::Population;
pub use template::Template;
pub use time::Time;
pub use world::World;

use crate::simulation::{
    constructor,
    manager::{status::Status, Manager},
    state::{
        self,
        navigation::{Graph, Navigation},
        population::sight::Sight,
        world::block,
    },
};

pub struct State {
    pub template: state::Template,
    pub construct_rx: Option<tokio::sync::mpsc::Receiver<(World, Population)>>,
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

        let action = Action::new();
        let world = World::new(template);
        let population = Population::new(template);
        let physics = Physics::new();
        let navigation = Navigation::new(&world.grid);
        let time = Time::new();

        Self {
            template,
            construct_rx,
            action,
            time,
            physics,
            world,
            population,
            navigation,
        }
    }

    pub fn place_block(state: &mut State) {
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
                judge.selected_block_kind,
                &state.world.block_info_map,
                &state.world.grid,
                &mut state.world.sector_vec,
            );

            let block_info = state.world.block_info_map[&judge.selected_block_kind];

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
    }

    pub fn load(state: &mut State, manager: &mut Manager) {
        if let Some(construct_rx) = &mut state.construct_rx {
            if let Ok((world, population)) = construct_rx.try_recv() {
                state.world = world;
                state.population = population;

                Navigation::init_graph(&state.world, &mut state.navigation.graph);

                manager.status = Status::Run;
            }
        }
    }

    pub fn tick(state: &mut State) {
        let _ = tracing::info_span!("state_tick").entered();

        Action::tick(state);
        Population::tick(&state.world, &mut state.population);
        Physics::tick(&state.world, &state.physics, &mut state.population);
        Navigation::tick(&state.world, &mut state.navigation);
        Time::tick(&mut state.time);
    }
}
