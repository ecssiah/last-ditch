use crate::simulation::state::world::chunk;

#[derive(Clone, Default, Debug)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub geometry: chunk::Geometry,
}
