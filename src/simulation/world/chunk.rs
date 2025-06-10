pub mod edge;
pub mod geometry;
pub mod graph;
pub mod id;
pub mod node;

pub use edge::Edge;
pub use edge::EdgeKey;
pub use geometry::Geometry;
pub use graph::Graph;
pub use id::ID;
pub use node::Node;

use crate::simulation::{
    time::Tick,
    world::{block, chunk, grid},
};
use glam::IVec3;

pub struct Chunk {
    pub(crate) id: chunk::ID,
    pub(crate) tick: Tick,
    pub(crate) block_updated: bool,
    pub(crate) boundary_updated: bool,
    pub(crate) position: IVec3,
    pub(crate) geometry: chunk::Geometry,
    pub(crate) kind_list: Vec<block::Kind>,
    pub(crate) block_list: Vec<usize>,
    pub(crate) visibility_list: Vec<Vec<grid::Direction>>,
}
