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
    pub agent_map: HashMap<u64, Agent>,
    pub next_judge_id: u64,
    pub next_agent_id: u64,
}

impl Population {
    pub fn new(seed: u64) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let judge = Judge::new(0);
        let agent_map = HashMap::new();
        
        let next_judge_id = 1;
        let next_agent_id = 0;

        Self {
            next_judge_id,
            next_agent_id,
            rng,
            judge,
            agent_map,
        }
    }

    pub fn get_next_judge_id(population: &mut Population) -> u64 {
        let judge_id = population.next_judge_id;

        population.next_judge_id += 1;

        judge_id
    }

    pub fn get_next_agent_id(population: &mut Population) -> u64 {
        let agent_id = population.next_agent_id;

        population.next_agent_id += 1;

        agent_id
    }

    pub fn tick(world: &World, population: &mut Population) {
        let _ = tracing::info_span!("population_tick").entered();

        Judge::tick(world, &mut population.judge);

        for agent in population.agent_map.values_mut() {
            Agent::tick(world, agent);
        }
    }
}
