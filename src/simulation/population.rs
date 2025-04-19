pub mod agent;
pub mod judge;

pub use agent::Agent;
pub use judge::Judge;

use crate::simulation::{time::Tick, world::World};
use glam::Vec3;
use std::collections::HashMap;

pub struct Population {
    pub tick: Tick,
    pub judge: Option<Judge>,
    pub agents: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new() -> Population {
        let population = Population {
            tick: Tick::ZERO,
            judge: None,
            agents: HashMap::new(),
        };

        population
    }

    pub fn generate(&mut self) {
        self.generate_judge();
        self.generate_agents();
    }

    fn generate_judge(&mut self) {
        let mut judge = Judge::new(judge::ID::allocate());

        judge.set_position(10.0, 2.0, 0.0);
        judge.set_rotation(0.0, 0.0);

        self.judge = Some(judge);
    }

    fn generate_agents(&mut self) {
        for x in -2..=2 {
            let mut agent = Agent::new(agent::ID::allocate());

            let position = Vec3::new((6 * x) as f32, 1.0, -18.0);

            agent.set_position(position.x, position.y, position.z);

            self.agents.insert(agent.id, agent);

            let mut agent = Agent::new(agent::ID::allocate());

            let position = Vec3::new((6 * x) as f32, 1.0, 18.0);

            agent.set_position(position.x, position.y, position.z);

            self.agents.insert(agent.id, agent);

            let mut agent = Agent::new(agent::ID::allocate());

            let position = Vec3::new(-18.0, 1.0, (6 * x) as f32);

            agent.set_position(position.x, position.y, position.z);

            self.agents.insert(agent.id, agent);

            let mut agent = Agent::new(agent::ID::allocate());

            let position = Vec3::new(18.0, 1.0, (6 * x) as f32);

            agent.set_position(position.x, position.y, position.z);

            self.agents.insert(agent.id, agent);
        }
    }

    pub fn tick(&mut self, tick: &Tick, _world: &World) {
        self.tick = *tick;
    }

    pub fn get_judge(&self) -> Option<&Judge> {
        self.judge.as_ref()
    }

    pub fn get_judge_mut(&mut self) -> Option<&mut Judge> {
        self.judge.as_mut()
    }

    pub fn all_agents(&self) -> impl Iterator<Item = &Agent> {
        self.agents.values()
    }

    pub fn all_agents_mut(&mut self) -> impl Iterator<Item = &mut Agent> {
        self.agents.values_mut()
    }

    pub fn get_agent(&self, agent_id: &agent::ID) -> Option<&Agent> {
        self.agents.get(agent_id)
    }

    pub fn get_agent_mut(&mut self, agent_id: &agent::ID) -> Option<&mut Agent> {
        self.agents.get_mut(agent_id)
    }
}
