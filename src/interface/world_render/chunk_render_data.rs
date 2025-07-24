use crate::{interface::mesh_data::MeshData, simulation::state::world::chunk};

pub struct ChunkRenderData {
    pub chunk_id: chunk::ID,
    pub mesh_data: MeshData,
}
