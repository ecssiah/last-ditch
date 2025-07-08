use crate::simulation::state::world::chunk;

#[derive(Clone, Debug)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub geometry: chunk::Geometry,
}

impl ChunkView {
    pub fn new() -> Self {
        Self {
            id: chunk::ID::MAX,
            geometry: chunk::Geometry::new(),
        }
    }
}
