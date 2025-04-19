pub mod agent;
pub mod judge;

pub use agent::Agent;
pub use judge::Judge;
use rand::Rng;

use crate::simulation::{
    block::Direction, time::Tick, world::World, consts::*,
};
use glam::{IVec3, Vec3};
use std::collections::HashMap;

pub struct Population {
    pub tick: Tick,
    pub judge: Judge,
    pub agents: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new() -> Population {
        let population = Population {
            tick: Tick::ZERO,
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
        self.judge.set_position(18.0, 20.0, 18.0);
        self.judge.set_rotation(0.0, 0.0 * std::f32::consts::PI);
    }

    fn generate_agents(&mut self) {
        for _ in 0..AGENT_INITIAL_POPULATION {
            let mut agent = Agent::new(agent::ID::allocate());

            let position = Vec3::new(0.0, 2.0, 0.0);

            agent.position = position;
            agent.target = position;

            self.agents.insert(agent.id, agent);
        }
    }

    pub fn tick(&mut self, tick: &Tick, world: &World) {
        self.tick = *tick;

        self.tick_agents(world);
    }

    fn tick_agents(&mut self, world: &World) {
        for agent in self.agents.values_mut() {
            let path = agent.target - agent.position;

            if path.length_squared() > 1e-3 {
                agent.position += agent.speed * FIXED_DT.as_secs_f32() * path.normalize();
            } else {
                Self::find_target(agent, world);
            }
        }
    }

    fn find_target(agent: &mut Agent, world: &World) {
        let seed = agent.id.0 as u64;
        let mut rng = rand_pcg::Pcg32::new(seed, u64::from(world.tick));

        let direction_index = rng.gen_range(0..4);
        let direction = Direction::cardinal()[direction_index];

        let dy = rng.gen_range(-1..=1);

        let delta = match direction {
            Direction::XpYoZo => IVec3::new(1, dy, 0),
            Direction::XnYoZo => IVec3::new(-1, dy, 0),
            Direction::XoYoZp => IVec3::new(0, dy, 1),
            Direction::XoYoZn => IVec3::new(0, dy, -1),
            _ => IVec3::ZERO,
        };

        if let Some(grid_position) = World::grid_position_at(agent.position) {
            let target_position = grid_position + delta;

            if let Some(block) = world.get_block(target_position) {
                if block.solid {
                    if let Some(air_block1) = world.get_block(target_position + IVec3::new(0, 1, 0))
                    {
                        if !air_block1.solid {
                            if let Some(air_block2) =
                                world.get_block(target_position + IVec3::new(0, 2, 0))
                            {
                                if !air_block2.solid {
                                    agent.target = target_position.as_vec3();
                                }
                            }
                        }
                    }
                }
            }
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
