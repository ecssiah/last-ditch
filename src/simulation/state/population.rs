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
    pub kind: simulation::Kind,
    pub judge: Judge,
    pub agent_map: HashMap<entity::ID, Agent>,
}

impl Population {
    pub fn new(kind: simulation::Kind) -> Self {
        let judge = Judge::new();
        let agent_map = HashMap::new();

        Self {
            kind,
            judge,
            agent_map,
        }
    }

    pub fn placeholder() -> Self {
        let kind = simulation::Kind::Placeholder;
        let judge = Judge::new();
        let agent_map = HashMap::new();

        Self {
            kind,
            judge,
            agent_map,
        }
    }

    pub fn setup(kind: simulation::Kind, world: &World, population: &mut Population) {
        match kind {
            simulation::Kind::Placeholder => (),
            simulation::Kind::EmptyWorld => {
                constructor::population::empty::construct();
            }
            simulation::Kind::MainWorld => {
                constructor::population::main::construct(world, population);
            }
            simulation::Kind::TestWorld => {
                constructor::population::world_test::construct(population);
            }
            simulation::Kind::GraphWorld => {
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
