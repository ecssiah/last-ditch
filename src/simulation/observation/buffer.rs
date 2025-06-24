use crate::simulation::observation::view::View;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct Buffer {
    buffer_arc_list: [Arc<View>; 2],
    write_index: AtomicUsize,
    read_index: AtomicUsize,
}

impl Buffer {
    pub fn new(view: View) -> Self {
        let view1 = Arc::new(view.clone());
        let view2 = Arc::new(view);

        Self {
            buffer_arc_list: [view1, view2],
            write_index: AtomicUsize::new(0),
            read_index: AtomicUsize::new(0),
        }
    }

    pub fn get(&self) -> Arc<View> {
        let index = self.read_index.load(Ordering::Acquire);
        self.buffer_arc_list[index].clone()
    }

    pub fn update(&mut self, view: View) {
        let index = self.write_index.load(Ordering::Relaxed);
        self.buffer_arc_list[index] = Arc::new(view);

        self.read_index.store(index, Ordering::Release);
        self.write_index.store(1 - index, Ordering::Relaxed);
    }
}
