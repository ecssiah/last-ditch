pub mod entity;

pub use entity::Entity;

use std::collections::HashMap;

use crate::simulation::time::Tick;

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
        let mut user_entity1 = Entity::new(entity::ID::USER_ENTITY1);

        user_entity1.set_position(10.0, 2.0, 10.0);
        user_entity1.set_rotation(0.0, 0.0);

        self.entities.insert(user_entity1.id, user_entity1);
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
