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
    self,
    population::{agent::Agent, judge::Judge},
    world::World,
};
use std::collections::HashMap;

pub struct Population {
    pub state_template: state::Template,
    pub judge: Judge,
    pub agent_map: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new(state_template: state::Template) -> Self {
        let judge = Judge::new();
        let agent_map = HashMap::new();

        Self {
            state_template,
            judge,
            agent_map,
        }
    }

    pub fn placeholder() -> Self {
        let state_template = state::Template::Placeholder;
        let judge = Judge::new();
        let agent_map = HashMap::new();

        Self {
            state_template,
            judge,
            agent_map,
        }
    }

    pub fn tick(world: &World, population: &mut Population) {
        let _ = tracing::info_span!("population_tick").entered();

        for agent in population.agent_map.values_mut() {
            Agent::tick(world, agent);
        }

        Judge::tick(world, &mut population.judge);
    }
}
