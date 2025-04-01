use crate::simulation::{
    id::agent_id::AgentID,
    observation::{buffer::Buffer, view::View},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Repository {
    buffers: Arc<RwLock<HashMap<AgentID, Buffer>>>,
}

impl Repository {
    pub fn new() -> Self {
        let repository = Self {
            buffers: Arc::new(RwLock::new(HashMap::new())),
        };

        repository
    }

    pub fn add(&self, agent_id: AgentID, view: View) {
        let buffer = Buffer::new(view);
        let mut buffers = self.buffers.write().unwrap();

        buffers.insert(agent_id, buffer);
    }

    pub fn update(&self, agent_id: AgentID, view: View) {
        let buffers = self.buffers.read().unwrap();

        if let Some(buffer) = buffers.get(&agent_id) {
            buffer.update(view);
        } else {
            log::error!("ObservationID {:?} not found for update.", agent_id);
        }
    }

    pub fn get(&self, agent_id: AgentID) -> Option<Arc<View>> {
        let buffers = self.buffers.read().unwrap();

        buffers.get(&agent_id).and_then(|buffer| buffer.get())
    }

    pub fn list_agents(&self) -> Vec<AgentID> {
        let buffers = self.buffers.read().unwrap();

        buffers.keys().cloned().collect()
    }
}
