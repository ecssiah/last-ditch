pub mod agent;
pub mod decision;
pub mod judge;

pub use agent::Agent;
pub use judge::Judge;
use rand::{Rng, SeedableRng};

use crate::simulation::{consts::*, population::agent::Kind, time::Tick, world::World};
use glam::Vec3;
use std::collections::HashMap;

pub struct Population {
    pub tick: Tick,
    pub rand_pcg: rand_pcg::Pcg32,
    pub judge: Judge,
    pub agents: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new() -> Population {
        let population = Population {
            tick: Tick::ZERO,
            rand_pcg: rand_pcg::Pcg32::seed_from_u64(DEFAULT_SEED),
            judge: Judge::new(judge::ID::allocate()),
            agents: HashMap::new(),
        };

        population
    }

    pub fn generate(&mut self) {
        self.generate_judge();
        self.generate_agents();
    }

    fn generate_judge(&mut self) {
        println!("Generating Judge");

        self.judge.set_position(18.0, 12.0, 0.0);
        self.judge.set_rotation(0.0, 0.0 * std::f32::consts::PI);
    }

    fn generate_agents(&mut self) {
        println!("Generating Agents");

        for _ in 0..AGENT_INITIAL_POPULATION {
            let mut agent = Agent::new(agent::ID::allocate());

            let position = Vec3::new(0.0, 2.0, 0.0);

            agent.position = position;
            agent.target = position;

            let mut rng = rand_pcg::Pcg32::from_entropy();
            let choice = rng.gen_range(0..4);

            agent.kind = Kind::all()[choice].clone();
            agent.height = rng.gen_range(0.4..1.0);

            self.agents.insert(agent.id, agent);
        }
    }

    pub fn tick(&mut self, tick: &Tick, world: &World) {
        self.tick = *tick;

        self.tick_agents(world);
    }

    fn tick_agents(&mut self, world: &World) {
        for agent in self.agents.values_mut() {
            agent.tick(world);
        }
    }

    pub fn get_judge(&self) -> &Judge {
        &self.judge
    }

    pub fn get_judge_mut(&mut self) -> &mut Judge {
        &mut self.judge
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
