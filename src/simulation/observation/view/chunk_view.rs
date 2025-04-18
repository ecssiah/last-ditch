use crate::simulation::{chunk, time::Tick};
use glam::IVec3;

#[derive(Clone, Debug)]
pub struct ChunkView {
    pub id: chunk::ID,
    pub tick: (Tick, Tick),
    pub position: (IVec3, IVec3),
    pub mesh: (chunk::Mesh, chunk::Mesh),
}
