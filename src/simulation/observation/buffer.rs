use crate::simulation::observation::view::View;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

pub struct Buffer {
    buffers: [Arc<RwLock<View>>; 2],
    current_write: AtomicUsize,
    current_read: AtomicUsize,
}

impl Buffer {
    pub fn new(view: View) -> Self {
        let view_arc = Arc::new(RwLock::new(view));

        let buffer = Self {
            buffers: [view_arc.clone(), view_arc],
            current_write: AtomicUsize::new(0),
            current_read: AtomicUsize::new(0),
        };

        buffer
    }

    pub fn update(&self, new_view: View) {
        let write_index = self.current_write.load(Ordering::Relaxed);

        {
            let mut buffer = self.buffers[write_index].write().unwrap();
            *buffer = new_view;
        }

        self.current_read.store(write_index, Ordering::Release);
        self.current_write.store(1 - write_index, Ordering::Relaxed);
    }

    pub fn get(&self) -> Arc<View> {
        let read_index = self.current_read.load(Ordering::Acquire);

        {
            let buffer = self.buffers[read_index].read().unwrap();

            Arc::new(buffer.clone())
        }
    }
}
