//! Entities acting in the simulated environment

pub mod entity;
pub mod nation;

use crate::simulation::{
    self, constructor,
    state::{
        compute::task,
        population::entity::{Agent, Judge},
        world::World,
        Compute,
    },
};
use std::collections::HashMap;

pub struct Population {
    pub kind: simulation::Kind,
    pub judge: Judge,
    pub agent_map: HashMap<entity::ID, Agent>,
    pub task_input_vec: Vec<task::Input>,
}

impl Population {
    pub fn new(kind: simulation::Kind) -> Self {
        let judge = Judge::new();
        let agent_map = HashMap::new();
        let task_input_vec = Vec::new();

        Self {
            kind,
            judge,
            agent_map,
            task_input_vec,
        }
    }

    pub fn placeholder() -> Self {
        let kind = simulation::Kind::Placeholder;
        let judge = Judge::new();
        let agent_map = HashMap::new();
        let task_input_vec = Vec::new();

        Self {
            kind,
            judge,
            agent_map,
            task_input_vec,
        }
    }

    pub fn setup(kind: simulation::Kind, world: &World, population: &mut Population) {
        match kind {
            simulation::Kind::Main => {
                constructor::population::main::construct(world, population);
            }
            simulation::Kind::Empty => {
                constructor::population::empty::construct();
            }
            simulation::Kind::WorldTest => {
                constructor::population::world_test::construct(population);
            }
            simulation::Kind::GraphTest => {
                constructor::population::graph_test::construct(population);
            }
            simulation::Kind::Placeholder => (),
        }
    }

    pub fn tick(world: &World, population: &mut Population, compute: &mut Compute) {
        for agent in population.agent_map.values_mut() {
            Agent::tick(world, agent, compute);
        }

        Judge::tick(world, &mut population.judge);
    }
}
