pub mod mesh;
pub mod vertex;

pub use mesh::Mesh;
pub use vertex::Vertex;

use crate::{
    interface,
    simulation::{self, time::Tick},
};

pub struct Chunk {
    pub id: simulation::chunk::ID,
    pub tick: Tick,
    pub mesh: interface::chunk::Mesh,
}
