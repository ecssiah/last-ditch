use crate::simulation::{
    population::entity,
    observation::{buffer::Buffer, view::View},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Repository {
    buffers: Arc<RwLock<HashMap<entity::ID, Buffer>>>,
}

impl Repository {
    pub fn new() -> Self {
        let repository = Self {
            buffers: Arc::new(RwLock::new(HashMap::new())),
        };

        repository
    }

    pub fn set(&self, entity_id: &entity::ID, view: View) {
        let mut buffers = self.buffers.write().unwrap();

        buffers
            .entry(*entity_id)
            .and_modify(|buffer| buffer.update(view.clone()))
            .or_insert_with(|| Buffer::new(view));
    }

    pub fn get(&self, entity_id: &entity::ID) -> Option<Arc<View>> {
        let buffers = self.buffers.read().unwrap();

        buffers.get(entity_id).map(|buffer| buffer.get())
    }

    pub fn entity_ids(&self) -> Vec<entity::ID> {
        let buffers = self.buffers.read().unwrap();

        buffers.keys().cloned().collect()
    }
}
