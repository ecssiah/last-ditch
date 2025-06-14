//! Entities acting in the simulated environment

pub mod agent;
pub mod builder;
pub mod decision;
pub mod judge;

pub use agent::Agent;
pub use judge::Judge;

use crate::simulation::{consts::*, time::Tick, world::World};
use std::collections::HashMap;

pub struct Population {
    pub tick: Tick,
    pub judge: Judge,
    pub agent_map: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new() -> Self {
        Self {
            tick: Tick::ZERO,
            judge: Judge::new(judge::ID::allocate()),
            agent_map: HashMap::new(),
        }
    }

    pub fn setup(&mut self, world: &World) {
        if TESTING {
            builder::TestPopulation::build(self, &world);
        } else {
            builder::MainPopulation::build(self, &world);
        }
    }

    pub fn tick(&mut self, tick: &Tick, world: &World) {
        self.tick = *tick;

        self.tick_agent_map(world);
        self.judge.tick(world);
    }

    fn tick_agent_map(&mut self, world: &World) {
        for agent in self.agent_map.values_mut() {
            agent.tick(world);
        }
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

    pub fn get_agent(&self, agent_id: &agent::ID) -> Option<&Agent> {
        self.agent_map.get(agent_id)
    }

    pub fn get_agent_mut(&mut self, agent_id: &agent::ID) -> Option<&mut Agent> {
        self.agent_map.get_mut(agent_id)
    }
}
