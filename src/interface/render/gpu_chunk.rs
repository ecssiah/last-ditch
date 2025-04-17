use crate::{
    interface::render::GPUMesh,
    simulation::{self},
};

pub struct GPUChunk {
    pub chunk_id: simulation::chunk::ID,
    pub tick: simulation::time::Tick,
    pub gpu_mesh: GPUMesh,
}
