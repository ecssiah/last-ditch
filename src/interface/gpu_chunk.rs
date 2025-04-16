pub mod gpu_mesh;
pub mod gpu_vertex;

pub use gpu_mesh::GPUMesh;

use crate::simulation::{self};

pub struct GPUChunk {
    pub chunk_id: simulation::chunk::ID,
    pub tick: simulation::time::Tick,
    pub gpu_mesh: GPUMesh,
}
