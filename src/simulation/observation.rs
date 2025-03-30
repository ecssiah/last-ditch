use crate::simulation::{
    id::{chunk_id::ChunkID, observation_id::ObservationID, palette_id::PaletteID}, state::State, time::Tick, Chunk, CHUNK_VOLUME
};
use glam::{IVec3, Vec3};
use std::{
    collections::HashMap,
    sync::{atomic::{AtomicPtr, AtomicUsize, Ordering}, Arc, RwLock},
};

struct View {
    pub tick: Tick,
    pub position: Vec3,

    pub chunk_views: HashMap<ChunkID, Chunk>,
}

pub struct Buffer {
    buffers: [AtomicPtr<View>; 2],
    current_write: AtomicUsize,
    current_read: AtomicUsize,
}

impl Buffer {
    fn new(initial: View) -> Self {
        let ptr = Box::into_raw(Box::new(initial));

        let buffer = Self {
            buffers: [AtomicPtr::new(ptr), AtomicPtr::new(ptr)],
            current_write: AtomicUsize::new(0),
            current_read: AtomicUsize::new(0),
        };

        buffer
    }

    fn update(&self, new_view: View) {
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

    fn get(&self) -> Option<Arc<View>> {
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

struct Repository {
    buffers: Arc<RwLock<HashMap<ObservationID, Buffer>>>,
}

impl Repository {
    pub fn new() -> Self {
        Self {
            buffers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_observation(&self, id: ObservationID, initial_view: View) {
        let buffer = Buffer::new(initial_view);
        let mut buffers = self.buffers.write().unwrap();

        buffers.insert(id, buffer);
    }

    pub fn update_observation(&self, id: ObservationID, new_view: View) {
        let buffers = self.buffers.read().unwrap();

        if let Some(buffer) = buffers.get(&id) {
            buffer.update(new_view);
        } else {
            log::error!("ObservationID {:?} not found for update.", id);
        }
    }

    pub fn get_observation(&self, id: ObservationID) -> Option<Arc<View>> {
        let buffers = self.buffers.read().unwrap();

        buffers.get(&id).and_then(|buffer| buffer.get())
    }
}

pub struct Observation {
    repository: Repository,
}

impl Observation {
    pub fn new() -> Self {
        let repository = Repository::new();

        let observation = Self { repository };

        observation
    }

    pub fn update(&mut self, state: &State) {}
}
