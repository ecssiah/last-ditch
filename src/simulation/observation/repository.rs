use crate::simulation::{
    agent,
    observation::{buffer::Buffer, view::View},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Repository {
    buffers: Arc<RwLock<HashMap<agent::ID, Buffer>>>,
}

impl Repository {
    pub fn new() -> Self {
        let repository = Self {
            buffers: Arc::new(RwLock::new(HashMap::new())),
        };

        repository
    }

    pub fn add(&self, agent_id: agent::ID, view: View) {
        let buffer = Buffer::new(view);
        let mut buffers = self.buffers.write().unwrap();

        buffers.insert(agent_id, buffer);
    }

    pub fn update(&self, agent_id: agent::ID, view: View) {
        let buffers = self.buffers.read().unwrap();

        if let Some(buffer) = buffers.get(&agent_id) {
            buffer.update(view);
        } else {
            log::error!("ObservationID {:?} not found for update.", agent_id);
        }
    }

    pub fn get(&self, agent_id: agent::ID) -> Option<Arc<View>> {
        let buffers = self.buffers.read().unwrap();

        buffers.get(&agent_id).map(|buffer| buffer.get())
    }

    pub fn list_agents(&self) -> Vec<agent::ID> {
        let buffers = self.buffers.read().unwrap();

        buffers.keys().cloned().collect()
    }
}
