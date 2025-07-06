use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct Buffer<T> {
    buffer_arc_vec: [Arc<T>; 2],
    write_index: AtomicUsize,
    read_index: AtomicUsize,
}

impl<T> Buffer<T> {
    pub fn new(value: T) -> Self
    where
        T: Clone,
    {
        let view1 = Arc::new(value.clone());
        let view2 = Arc::new(value);

        Self {
            buffer_arc_vec: [view1, view2],
            write_index: AtomicUsize::new(0),
            read_index: AtomicUsize::new(0),
        }
    }

    pub fn get(&self) -> Arc<T> {
        let index = self.read_index.load(Ordering::Acquire);
        self.buffer_arc_vec[index].clone()
    }

    pub fn update(&mut self, value: T) {
        let index = self.write_index.load(Ordering::Relaxed);
        self.buffer_arc_vec[index] = Arc::new(value);

        self.read_index.store(index, Ordering::Release);
        self.write_index.store(1 - index, Ordering::Relaxed);
    }
}
