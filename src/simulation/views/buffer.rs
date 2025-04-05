use crate::simulation::views::view::View;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

pub struct Buffer {
    buffers: [Arc<RwLock<View>>; 2],
    write_index: AtomicUsize,
    read_index: AtomicUsize,
}

impl Buffer {
    pub fn new(view: View) -> Self {
        let view_arc1 = Arc::new(RwLock::new(view));
        let view_arc2 = view_arc1.clone();

        let buffer = Self {
            buffers: [view_arc1, view_arc2],
            write_index: AtomicUsize::new(0),
            read_index: AtomicUsize::new(0),
        };

        buffer
    }

    pub fn update(&self, view: View) {
        let index = self.swap(view);

        self.read_index.store(index, Ordering::Release);
        self.write_index.store(1 - index, Ordering::Relaxed);
    }

    fn swap(&self, view: View) -> usize {
        let index = self.write_index.load(Ordering::Relaxed);
        let mut buffer = self.buffers[index].write().unwrap();
        *buffer = view;

        index
    }

    pub fn get(&self) -> Arc<View> {
        let index = self.read_index.load(Ordering::Acquire);
        let buffer = self.buffers[index].read().unwrap();

        Arc::new(buffer.clone())
    }
}
