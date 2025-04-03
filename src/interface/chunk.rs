use crate::{
    interface::chunk::mesh::Mesh,
    simulation::{id::chunk_id::ChunkID, time::Tick},
};

pub mod mesh;
pub mod vertex;

pub struct Chunk {
    pub id: ChunkID,
    pub tick: Tick,
    pub mesh: Mesh,
}
