use crate::simulation::observation::{buffer::Buffer, view::View};
use std::sync::{Arc, RwLock};

pub struct Repository {
    buffer_lock: RwLock<Buffer>,
}

impl Repository {
    pub fn new() -> Self {
        let view = View::default();

        Self {
            buffer_lock: RwLock::new(Buffer::new(view)),
        }
    }

    pub fn get(&self) -> Arc<View> {
        let buffer = self.buffer_lock.read().unwrap();

        buffer.get()
    }

    pub fn set(&self, view: View) {
        let mut buffer = self.buffer_lock.write().unwrap();

        buffer.update(view.clone())
    }
}
