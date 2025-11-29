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
    population::{agent::Agent, judge::Judge},
    world::World,
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;

pub struct Population {
    pub rng: ChaCha8Rng,
    pub judge: Judge,
    pub agent_map: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new(seed: u64) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let judge = Judge::new();
        let agent_map = HashMap::new();

        Self {
            rng,
            judge,
            agent_map,
        }
    }

    pub fn tick(world: &World, population: &mut Population) {
        let _ = tracing::info_span!("population_tick").entered();

        Judge::tick(world, &mut population.judge);

        for agent in population.agent_map.values_mut() {
            Agent::tick(world, agent);
        }
    }
}
