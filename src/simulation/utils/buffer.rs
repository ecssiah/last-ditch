use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct Buffer<T> {
    value_arc_array: [Arc<T>; 2],
    write_index: AtomicUsize,
    read_index: AtomicUsize,
}

impl<T> Buffer<T> {
    pub fn new(value: T) -> Self
    where
        T: Clone,
    {
        let value1 = Arc::new(value.clone());
        let value2 = Arc::new(value);

        let value_arc_array = [value1, value2];

        Self {
            value_arc_array,
            write_index: AtomicUsize::new(1),
            read_index: AtomicUsize::new(0),
        }
    }

    pub fn get(&self) -> Arc<T> {
        let index = self.read_index.load(Ordering::Acquire);
        self.value_arc_array[index].clone()
    }

    pub fn update(&mut self, value: T) {
        let index = self.write_index.load(Ordering::Relaxed);
        self.value_arc_array[index] = Arc::new(value);

        self.read_index.store(index, Ordering::Release);
        self.write_index.store(1 - index, Ordering::Relaxed);
    }
}
