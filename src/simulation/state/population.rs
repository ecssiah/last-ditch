//! Entities acting in the simulated environment

pub mod entity;

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
        let agent_map = HashMap::default();

        Self {
            kind,
            judge,
            agent_map,
        }
    }

    pub fn setup(&mut self, world: &World) {
        match self.kind {
            simulation::Kind::Main => constructor::main::construct_population(self, world),
            simulation::Kind::Placeholder => (),
            simulation::Kind::Empty => constructor::empty::construct_population(self, world),
            simulation::Kind::WorldTest => {
                constructor::world_test::construct_population(self, world)
            }
            simulation::Kind::GraphTest => {
                constructor::graph_test::construct_population(self, world)
            }
        }
    }

    pub fn tick(&mut self, world: &World) {
        for agent in self.agent_map.values_mut() {
            agent.tick(world);
        }

        self.judge.tick(world);
    }

    pub fn get_judge(&self) -> &Judge {
        &self.judge
    }

    pub fn get_judge_mut(&mut self) -> &mut Judge {
        &mut self.judge
    }

    pub fn get_agent_map(&self) -> impl Iterator<Item = &Agent> {
        self.agent_map.values()
    }

    pub fn get_agent_map_mut(&mut self) -> impl Iterator<Item = &mut Agent> {
        self.agent_map.values_mut()
    }

    pub fn get_agent(&self, agent_id: &entity::ID) -> Option<&Agent> {
        self.agent_map.get(agent_id)
    }

    pub fn get_agent_mut(&mut self, agent_id: &entity::ID) -> Option<&mut Agent> {
        self.agent_map.get_mut(agent_id)
    }
}
