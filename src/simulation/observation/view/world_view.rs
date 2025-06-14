use crate::simulation::{
    observation::{state_pair::StatePair, view::chunk_view::ChunkView},
    time::Tick,
    world::chunk,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorldView {
    pub tick: StatePair<Tick>,
    pub chunk_view_map: HashMap<chunk::ID, ChunkView>,
}

impl WorldView {
    pub fn new() -> Self {
        Self {
            tick: StatePair::new(Tick::ZERO, Tick::ZERO),
            chunk_view_map: HashMap::new(),
        }
    }
}
