pub mod entity;

pub use entity::Entity;
use glam::Vec3;
use rand::{Rng, SeedableRng};

use std::collections::HashMap;

use crate::simulation::{time::Tick, AGENT_INITIAL_POPULATION, DEFAULT_SEED, WORLD_BOUNDARY};

pub struct Population {
    pub tick: Tick,
    pub entities: HashMap<entity::ID, Entity>,
}

impl Population {
    pub fn new() -> Population {
        let population = Population {
            tick: Tick::ZERO,
            entities: HashMap::new(),
        };

        population
    }

    pub fn generate(&mut self) {
        self.generate_users();
        self.generate_agents();
    }

    fn generate_users(&mut self) {
        let mut user_entity1 = Entity::new(entity::ID::USER_ENTITY1);

        user_entity1.kind = entity::Kind::Leader;
        user_entity1.set_position(10.0, 2.0, 10.0);
        user_entity1.set_rotation(0.0, 0.0);

        self.entities.insert(user_entity1.id, user_entity1);
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
    
            self.entities.insert(agent.id, agent);
        }
    }

    pub fn tick(&mut self, tick: &Tick) {
        self.tick = *tick;
    }

    pub fn all(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn get(&self, entity_id: &entity::ID) -> Option<&Entity> {
        self.entities.get(entity_id)
    }

    pub fn get_mut(&mut self, entity_id: &entity::ID) -> Option<&mut Entity> {
        self.entities.get_mut(entity_id)
    }
}
