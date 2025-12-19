//! Current state of the simulation

pub mod action;
pub mod body;
pub mod navigation;
pub mod physics;
pub mod population;
pub mod time;
pub mod work;
pub mod world;

pub use action::Action;
pub use physics::Physics;
pub use population::Population;
pub use time::Time;
pub use world::World;

use crate::simulation::state::{
    navigation::Navigation,
    population::{person::Person, sight::Sight},
    work::Work,
    world::block,
};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};

pub struct State {
    pub rng: ChaCha8Rng,
    pub action: Action,
    pub world: World,
    pub population: Population,
    pub physics: Physics,
    pub navigation: Navigation,
    pub work: Work,
}

impl State {
    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(1);

        let action = Action::new();
        let world = World::new(rng.next_u64());
        let population = Population::new(rng.next_u64());
        let physics = Physics::new();
        let navigation = Navigation::new();
        let work = Work::new();

        Self {
            rng,
            action,
            physics,
            world,
            population,
            navigation,
            work,
        }
    }

    pub fn place_block(person: &Person, world: &mut World) {
        let range = 8.0;
        let origin = person.sight.world_position;
        let direction = Sight::get_forward(&person.sight);

        if let Some((hit_position, normal)) =
            World::raycast_to_block(origin, direction, range, world)
        {
            let placement_position = hit_position + normal;

            World::set_block(placement_position, person.selected_block_kind, world);
        }
    }

    pub fn remove_block(person: &Person, world: &mut World) {
        let range = 8.0;
        let origin = person.sight.world_position;
        let direction = Sight::get_forward(&person.sight);

        if let Some((hit_position, _)) = World::raycast_to_block(origin, direction, range, world) {
            World::set_block(hit_position, block::Kind::None, world);
        }
    }

    pub fn seed(seed: u64, state: &mut Self) {
        state.rng = ChaCha8Rng::seed_from_u64(seed);

        state.world.rng = ChaCha8Rng::seed_from_u64(state.rng.next_u64());
        state.population.rng = ChaCha8Rng::seed_from_u64(state.rng.next_u64());
    }

    pub fn tick(state: &mut Self) {
        let _ = tracing::info_span!("state_tick").entered();

        Action::tick(state);
        World::tick(&mut state.world);
        Population::tick(&mut state.population);
        Physics::tick(&state.world, &mut state.population, &mut state.physics);
        Navigation::tick(&state.world, &mut state.navigation);
        Work::tick(state);
    }
}
