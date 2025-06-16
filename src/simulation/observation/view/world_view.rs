use crate::simulation::{observation::view::chunk_view::ChunkView, world::chunk};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorldView {
    pub chunk_view_map: HashMap<chunk::ID, ChunkView>,
}

impl WorldView {
    pub fn new() -> Self {
        Self {
            chunk_view_map: HashMap::new(),
        }
    }
}
