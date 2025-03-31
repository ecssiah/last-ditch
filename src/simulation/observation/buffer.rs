use crate::simulation::observation::view::View;
use std::sync::{
    atomic::{AtomicPtr, AtomicUsize, Ordering},
    Arc,
};

pub struct Buffer {
    buffers: [AtomicPtr<View>; 2],
    current_write: AtomicUsize,
    current_read: AtomicUsize,
}

impl Buffer {
    pub fn new(view: View) -> Self {
        let view_ptr = Box::into_raw(Box::new(view));

        let buffer = Self {
            buffers: [AtomicPtr::new(view_ptr), AtomicPtr::new(view_ptr)],
            current_write: AtomicUsize::new(0),
            current_read: AtomicUsize::new(0),
        };

        buffer
    }

    pub fn update(&self, new_view: View) {
        let write_index = self.current_write.load(Ordering::Relaxed);
        let new_ptr = Box::into_raw(Box::new(new_view));

        let old_ptr = self.buffers[write_index].swap(new_ptr, Ordering::Release);

        unsafe {
            if !old_ptr.is_null() {
                drop(Box::from_raw(old_ptr));
            }
        }

        self.current_read.store(write_index, Ordering::Release);
        self.current_write.store(1 - write_index, Ordering::Relaxed);
    }

    pub fn get(&self) -> Option<Arc<View>> {
        let read_index = self.current_read.load(Ordering::Acquire);
        let view_ptr = self.buffers[read_index].load(Ordering::Acquire);

        unsafe {
            if !view_ptr.is_null() {
                Some(Arc::from_raw(view_ptr))
            } else {
                None
            }
        }
    }
}
