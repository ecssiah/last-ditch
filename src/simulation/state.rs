//! Current state of the simulation

pub mod action;
pub mod config;
pub mod constructor;
pub mod navigation;
pub mod physics;
pub mod population;
pub mod template;
pub mod time;
pub mod world;

pub use action::Action;
pub use config::Config;
pub use constructor::Constructor;
pub use physics::Physics;
pub use population::Population;
pub use template::Template;
pub use time::Time;
pub use world::World;

use crate::simulation::{
    manager::{status::Status, Manager},
    state::{
        self,
        navigation::{Graph, Navigation},
        population::sight::Sight,
        world::block,
    },
};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};

pub struct State {
    pub rng: ChaCha8Rng,
    pub constructor: Constructor,
    pub time: Time,
    pub action: Action,
    pub world: World,
    pub population: Population,
    pub physics: Physics,
    pub navigation: Navigation,
}

impl State {
    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(1);

        let constructor = Constructor::new(state::Template::Main);
        let action = Action::new();
        let world = World::new(rng.next_u64());
        let population = Population::new(rng.next_u64());
        let physics = Physics::new();
        let navigation = Navigation::new();
        let time = Time::new();

        Self {
            rng,
            constructor,
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
                &mut state.world.sector_vec,
            );

            Graph::set_solid(hit_position, false, &mut state.navigation.graph);
        }
    }

    pub fn seed(seed: u64, state: &mut State) {
        state.rng = ChaCha8Rng::seed_from_u64(seed);

        state.world.rng = ChaCha8Rng::seed_from_u64(state.rng.next_u64());
        state.population.rng = ChaCha8Rng::seed_from_u64(state.rng.next_u64());
    }

    pub fn load(manager: &mut Manager, state: &mut State) {
        match state.constructor.phase {
            constructor::Phase::World => todo!(),
            constructor::Phase::Population => todo!(),
            constructor::Phase::Navigation => todo!(),
            constructor::Phase::Complete => manager.status = Status::Run,
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
