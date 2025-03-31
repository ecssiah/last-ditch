use crate::simulation::{self, id::chunk_id::ChunkID, time::Tick};
use glam::Vec3;
use std::collections::HashMap;

pub struct ChunkView {
    pub mesh: simulation::chunk::mesh::Mesh,
}

pub struct View {
    pub tick: Tick,
    pub position: Vec3,

    pub chunk_views: HashMap<ChunkID, ChunkView>,
}
