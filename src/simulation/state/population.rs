//! Entities acting in the simulated environment

pub mod agent;
pub mod identity;
pub mod judge;
pub mod kinematic;
pub mod nation;
pub mod role;
pub mod sight;
pub mod spatial;

pub use role::Role;

use crate::simulation::state::{
    navigation::Navigation,
    population::{agent::Agent, judge::Judge, nation::Nation},
    world::World,
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;

pub struct Population {
    pub active: bool,
    pub rng: ChaCha8Rng,
    pub judge: Judge,
    pub agent_map: HashMap<u64, Agent>,
    pub nation_map: HashMap<nation::Kind, Nation>,
    pub next_entity_id: u64,
}

impl Population {
    pub fn new(seed: u64) -> Self {
        let active = false;
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let judge = Judge::new(0);
        let agent_map = HashMap::new();
        let nation_map = HashMap::new();
        let next_entity_id = 1;

        Self {
            active,
            next_entity_id,
            rng,
            judge,
            agent_map,
            nation_map,
        }
    }

    pub fn get_next_entity_id(population: &mut Population) -> u64 {
        let entity_id = population.next_entity_id;

        population.next_entity_id += 1;

        entity_id
    }

    pub fn tick(world: &World, navigation: &mut Navigation, population: &mut Population) {
        let _ = tracing::info_span!("population_tick").entered();

        if !population.active {
            return;
        }

        Judge::tick(world, &mut population.judge);

        for agent in population.agent_map.values_mut() {
            Agent::tick(navigation, agent);
        }
    }
}
