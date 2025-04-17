pub mod entity;

pub use entity::Entity;
use glam::Vec3;
use rand::{Rng, SeedableRng};

use crate::simulation::{time::Tick, AGENT_INITIAL_POPULATION, DEFAULT_SEED, WORLD_BOUNDARY};
use std::collections::HashMap;

pub struct Population {
    pub tick: Tick,
    pub judge: Option<Entity>,
    pub agents: HashMap<entity::ID, Entity>,
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
        let mut judge = Entity::new(entity::ID::USER_ENTITY1);

        judge.kind = entity::Kind::Judge;
        judge.set_position(10.0, 2.0, 10.0);
        judge.set_rotation(0.0, 0.0);

        self.judge = Some(judge);
    }

    fn generate_agents(&mut self) {
        let mut rng = rand_pcg::Pcg32::seed_from_u64(DEFAULT_SEED);

        for _ in 0..AGENT_INITIAL_POPULATION {
            let mut agent = Entity::new(entity::ID::allocate());

            let position = Vec3::new(
                rng.gen_range(-(WORLD_BOUNDARY as f32)..=(WORLD_BOUNDARY as f32)),
                2.0,
                rng.gen_range(-(WORLD_BOUNDARY as f32)..=(WORLD_BOUNDARY as f32)),
            );

            agent.kind = entity::Kind::Agent;
            agent.set_position(position.x, position.y, position.z);
            agent.set_rotation(0.0, 0.0);

            self.agents.insert(agent.id, agent);
        }
    }

    pub fn tick(&mut self, tick: &Tick) {
        self.tick = *tick;
    }

    pub fn get_judge(&self) -> Option<&Entity> {
        self.judge.as_ref()
    }

    pub fn get_judge_mut(&mut self) -> Option<&mut Entity> {
        self.judge.as_mut()
    }

    pub fn all_agents(&self) -> impl Iterator<Item = &Entity> {
        self.agents.values()
    }

    pub fn get_agent(&self, entity_id: &entity::ID) -> Option<&Entity> {
        self.agents.get(entity_id)
    }

    pub fn get_agent_mut(&mut self, entity_id: &entity::ID) -> Option<&mut Entity> {
        self.agents.get_mut(entity_id)
    }
}
