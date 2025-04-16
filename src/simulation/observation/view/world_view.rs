use crate::simulation::{chunk, observation::view::chunk_view::ChunkView, time::Tick};
use std::collections::HashMap;

#[derive(Clone)]
pub struct WorldView {
    pub tick: Tick,
    pub chunk_views: HashMap<chunk::ID, ChunkView>,
}
