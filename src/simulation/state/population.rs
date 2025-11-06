//! Entities acting in the simulated environment

pub mod entity;
pub mod nation;

use crate::simulation::{
    self, constructor,
    state::{
        population::entity::{Agent, Judge},
        world::World,
    },
};
use std::collections::HashMap;

pub struct Population {
    pub simulation_kind: simulation::Kind,
    pub judge: Judge,
    pub agent_map: HashMap<entity::ID, Agent>,
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

    pub fn setup(simulation_kind: simulation::Kind, world: &World, population: &mut Population) {
        match simulation_kind {
            simulation::Kind::Placeholder => (),
            simulation::Kind::Empty => {
                constructor::population::empty::construct();
            }
            simulation::Kind::Main => {
                constructor::population::main::construct(world, population);
            }
            simulation::Kind::Test => {
                constructor::population::world_test::construct(population);
            }
            simulation::Kind::Graph => {
                constructor::population::graph_test::construct(population);
            }
        }
    }

    pub fn tick(world: &World, population: &mut Population) {
        for agent in population.agent_map.values_mut() {
            Agent::tick(world, agent);
        }

        Judge::tick(world, &mut population.judge);
    }
}
