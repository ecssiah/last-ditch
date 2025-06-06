//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod agent;
pub mod decision;
pub mod judge;

pub use agent::Agent;
pub use judge::Judge;

use crate::simulation::{
    consts::*,
    physics::dynamic_object::DynamicObject,
    population::{self},
    time::Tick,
    world::World,
};
use glam::Vec3;
use rand::Rng;
use std::collections::HashMap;

pub struct Population {
    pub(crate) tick: Tick,
    pub(crate) judge: Judge,
    pub(crate) agent_map: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new() -> Population {
        let population = Population {
            tick: Tick::ZERO,
            judge: Judge::new(judge::ID::allocate()),
            agent_map: HashMap::new(),
        };

        population
    }

    pub fn setup(&mut self, world: &World) {
        self.setup_judge();
        self.setup_agents(world);
    }

    fn setup_judge(&mut self) {
        log::info!("Setup Judge");

        if TESTING {
            self.judge.set_position(0.0, -2.0, 0.0);
            self.judge.set_rotation(0.0, 0.0);
        } else {
            self.judge.set_position(0.0, 2.0, 0.0);
            self.judge.set_rotation(0.0, 0.0);
        }
    }

    fn setup_agents(&mut self, world: &World) {
        log::info!("Setup Agents");

        if TESTING {
            return;
        }

        let mut rng = rand::thread_rng();

        for kind in population::agent::Kind::all() {
            if let Some(flag_position) = world.get_flag(kind) {
                let flag_position = flag_position.as_vec3();

                for _ in 0..AGENT_INITIAL_POPULATION {
                    let offset =
                        Vec3::new(rng.gen_range(-4.0..=4.0), 0.0, rng.gen_range(-4.0..=4.0));
                    let position = flag_position + offset;

                    let mut agent = Agent::new(agent::ID::allocate());

                    agent.position = position;
                    agent.target = position;
                    agent.kind = kind;
                    agent.height = rng.gen_range(1.2..=2.2);

                    self.agent_map.insert(agent.id, agent);
                }
            }
        }
    }

    pub fn tick(&mut self, tick: &Tick, world: &World) {
        self.tick = *tick;

        self.tick_agent_map(world);
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
