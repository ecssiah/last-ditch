use crate::simulation::observation::view::View;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

pub struct Buffer {
    buffer_locks: [Arc<RwLock<View>>; 2],
    write_index: AtomicUsize,
    read_index: AtomicUsize,
}

impl Buffer {
    pub fn new(view: View) -> Self {
        let view1_lock = Arc::new(RwLock::new(view));
        let view2_lock = view1_lock.clone();

        let buffer = Self {
            buffer_locks: [view1_lock, view2_lock],
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
        let mut buffer = self.buffer_locks[index].write().unwrap();
        *buffer = view;

        index
    }

    pub fn get(&self) -> Arc<View> {
        let index = self.read_index.load(Ordering::Acquire);
        let buffer = self.buffer_locks[index].read().unwrap();

        Arc::new(buffer.clone())
    }
}
