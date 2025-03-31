use crate::simulation::{
    id::observation_id::ObservationID,
    observation::{buffer::Buffer, view::View},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Repository {
    buffers: Arc<RwLock<HashMap<ObservationID, Buffer>>>,
}

impl Repository {
    pub fn new() -> Self {
        let repository = Self {
            buffers: Arc::new(RwLock::new(HashMap::new())),
        };

        repository
    }

    pub fn add(&self, id: ObservationID, view: View) {
        let buffer = Buffer::new(view);
        let mut buffers = self.buffers.write().unwrap();

        buffers.insert(id, buffer);
    }

    pub fn update(&self, id: ObservationID, view: View) {
        let buffers = self.buffers.read().unwrap();

        if let Some(buffer) = buffers.get(&id) {
            buffer.update(view);
        } else {
            log::error!("ObservationID {:?} not found for update.", id);
        }
    }

    pub fn get(&self, id: ObservationID) -> Option<Arc<View>> {
        let buffers = self.buffers.read().unwrap();

        buffers.get(&id).and_then(|buffer| buffer.get())
    }
}
