//! Entities acting in the simulated environment

pub mod entity;

use crate::simulation::{
    self, constructor,
    state::{
        compute::Task,
        population::entity::{Agent, Judge},
        world::World,
    },
};
use std::collections::HashMap;

pub struct Population {
    pub kind: simulation::Kind,
    pub judge: Judge,
    pub agent_map: HashMap<entity::ID, Agent>,
    pub task_vec: Vec<Task>,
}

impl Population {
    pub fn new(kind: simulation::Kind) -> Self {
        let judge = Judge::new();
        let agent_map = HashMap::new();
        let task_vec = Vec::new();

        Self {
            kind,
            judge,
            agent_map,
            task_vec,
        }
    }

    pub fn placeholder() -> Self {
        let kind = simulation::Kind::Placeholder;

        let judge = Judge::new();
        let agent_map = HashMap::new();
        let task_vec = Vec::new();

        Self {
            kind,
            judge,
            agent_map,
            task_vec,
        }
    }

    pub fn setup(kind: simulation::Kind, population: &mut Population, world: &World) {
        match kind {
            simulation::Kind::Main => {
                constructor::population::main::construct(population, world);
            }
            simulation::Kind::Empty => {
                constructor::population::empty::construct(population, world);
            }
            simulation::Kind::WorldTest => {
                constructor::population::world_test::construct(population, world);
            }
            simulation::Kind::GraphTest => {
                constructor::population::graph_test::construct(population, world);
            }
            simulation::Kind::Placeholder => (),
        }
    }

    pub fn tick(population: &mut Population, world: &World) {
        for agent in population.agent_map.values_mut() {
            Agent::tick(agent, world);
        }

        Judge::tick(&mut population.judge, world);
    }
}
