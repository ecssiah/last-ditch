//! Entities acting in the simulated environment

pub mod agent;
pub mod entity;
pub mod judge;

use crate::simulation::{
    self,
    state::{
        population::{agent::Agent, judge::Judge},
        world::World,
    },
};
use std::collections::HashMap;

pub struct Population {
    pub simulation_kind: simulation::Kind,
    pub judge: Judge,
    pub agent_map: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new(simulation_kind: simulation::Kind) -> Self {
        let judge = Judge::new();
        let agent_map = HashMap::new();

        Self {
            simulation_kind,
            judge,
            agent_map,
        }
    }

    pub fn placeholder() -> Self {
        let simulation_kind = simulation::Kind::Placeholder;
        let judge = Judge::new();
        let agent_map = HashMap::new();

        Self {
            simulation_kind,
            judge,
            agent_map,
        }
    }

    pub fn tick(world: &World, population: &mut Population) {
        let _population_span = tracing::info_span!("population_tick").entered();

        for agent in population.agent_map.values_mut() {
            Agent::tick(world, agent);
        }

        Judge::tick(world, &mut population.judge);
    }
}
