use crate::{
    interface::render::data::MeshData,
    simulation::{self},
};

pub struct ChunkData {
    pub chunk_id: simulation::world::chunk::ID,
    pub tick: simulation::time::Tick,
    pub mesh_data: MeshData,
}
