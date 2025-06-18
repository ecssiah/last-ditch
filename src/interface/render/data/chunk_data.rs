use crate::{
    interface::render::data::MeshData,
    simulation::{self},
};

pub struct ChunkData {
    pub chunk_id: simulation::state::world::chunk::ID,
    pub mesh_data: MeshData,
}
