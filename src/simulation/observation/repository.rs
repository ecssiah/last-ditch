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

    pub fn add(&self, entity_id: entity::ID, view: View) {
        let buffer = Buffer::new(view);
        let mut buffers = self.buffers.write().unwrap();

        buffers.insert(entity_id, buffer);
    }

    pub fn update(&self, entity_id: entity::ID, view: View) {
        let buffers = self.buffers.read().unwrap();

        if let Some(buffer) = buffers.get(&entity_id) {
            buffer.update(view);
        } else {
            log::error!("ObservationID {:?} not found for update.", entity_id);
        }
    }

    pub fn get(&self, entity_id: entity::ID) -> Option<Arc<View>> {
        let buffers = self.buffers.read().unwrap();

        buffers.get(&entity_id).map(|buffer| buffer.get())
    }

    pub fn list_entities(&self) -> Vec<entity::ID> {
        let buffers = self.buffers.read().unwrap();

        buffers.keys().cloned().collect()
    }
}
