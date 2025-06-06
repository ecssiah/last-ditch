pub mod connection;
pub mod edge;
pub mod geometry;
pub mod graph;
pub mod id;
pub mod node;

pub use connection::Connection;
pub use edge::Edge;
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
    pub id: chunk::ID,
    pub tick: Tick,
    pub updated: bool,
    pub boundary_updated: bool,
    pub position: IVec3,
    pub geometry: chunk::Geometry,
    pub kind_list: Vec<block::Kind>,
    pub block_list: Vec<usize>,
    pub visibility_list: Vec<Vec<grid::Direction>>,
}
