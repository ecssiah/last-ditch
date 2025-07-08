//! Entities acting in the simulated environment

pub mod entity;

use crate::simulation::{
    self, constructor,
    state::{
        compute,
        population::entity::{Agent, Judge},
        world::World,
    },
};
use std::collections::HashMap;

pub struct Population {
    pub kind: simulation::Kind,
    pub judge: Judge,
    pub agent_map: HashMap<entity::ID, Agent>,
    pub task_vec: Vec<compute::task::Kind>,
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

    pub fn setup(&mut self, world: &World) {
        match self.kind {
            simulation::Kind::Main => {
                constructor::population::main::construct(self, world);
            }
            simulation::Kind::Empty => {
                constructor::population::empty::construct(self, world);
            }
            simulation::Kind::WorldTest => {
                constructor::population::world_test::construct(self, world);
            }
            simulation::Kind::GraphTest => {
                constructor::population::graph_test::construct(self, world);
            }
            simulation::Kind::Placeholder => (),
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
