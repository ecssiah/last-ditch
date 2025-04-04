pub mod mesh;
pub mod vertex;

pub use mesh::Mesh;
pub use vertex::Vertex;

use crate::{
    interface,
    simulation::{id::chunk_id::ChunkID, time::Tick},
};

pub struct Chunk {
    pub id: ChunkID,
    pub tick: Tick,
    pub mesh: interface::chunk::Mesh,
}
