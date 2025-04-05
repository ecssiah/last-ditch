pub mod entity;

pub use entity::Entity;

use std::collections::HashMap;

pub struct Population {
    entities: HashMap<entity::ID, Entity>,
}

impl Population {
    pub fn new() -> Population {
        let population = Population {
            entities: HashMap::new(),
        };

        population
    }

    pub fn generate(&mut self) {
        let mut user_entity = Entity::new(entity::ID::USER_ENTITY);

        user_entity.set_position(3.0, 3.0, 3.0);
        user_entity.set_rotation(0.0, 0.0);

        self.entities.insert(user_entity.id, user_entity);
    }

    pub fn tick(&mut self) {}

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
