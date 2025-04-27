pub mod agent;
pub mod decision;
pub mod judge;

pub use agent::Agent;
pub use judge::Judge;
use rand::{Rng, SeedableRng};

use crate::simulation::{
    consts::*,
    population::{self},
    time::Tick,
    world::World,
};
use std::collections::HashMap;

pub struct Population {
    pub tick: Tick,
    pub rand_pcg: rand_pcg::Pcg32,
    pub judge: Judge,
    pub agent_map: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new() -> Population {
        let population = Population {
            tick: Tick::ZERO,
            rand_pcg: rand_pcg::Pcg32::seed_from_u64(DEFAULT_SEED),
            judge: Judge::new(judge::ID::allocate()),
            agent_map: HashMap::new(),
        };

        population
    }

    pub fn generate(&mut self) {
        self.generate_judge();
        self.generate_agents();
    }

    fn generate_judge(&mut self) {
        log::info!("Generating Judge");

        self.judge.set_position(18.0, 12.0, 0.0);
        self.judge.set_rotation(0.0, 0.0 * std::f32::consts::PI);
    }

    fn generate_agents(&mut self) {
        log::info!("Generating Agents");

        for kind in population::agent::Kind::all() {
            for _ in 0..AGENT_INITIAL_POPULATION {
                let mut agent = Agent::new(agent::ID::allocate());

                let mut position = kind.home().as_vec3();
                position.x -= 2.0;

                agent.position = position;
                agent.target = position;

                let mut rng = rand_pcg::Pcg32::from_entropy();

                agent.kind = kind.clone();
                agent.height = rng.gen_range(0.7..1.3);

                self.agent_map.insert(agent.id, agent);
            }
        }
    }

    pub fn tick(&mut self, tick: &Tick, world: &World) {
        self.tick = *tick;

        self.tick_agents(world);
    }

    fn tick_agents(&mut self, world: &World) {
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
