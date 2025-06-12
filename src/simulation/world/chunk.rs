pub mod edge;
pub mod geometry;
pub mod graph;
pub mod id;
pub mod modified;
pub mod node;

pub use edge::Edge;
pub use edge::Key;
pub use geometry::Geometry;
pub use graph::Graph;
pub use id::ID;
pub use modified::Modified;
pub use node::Node;

use crate::simulation::{
    time::Tick,
    world::{block, chunk, grid},
};
use glam::IVec3;

pub struct Chunk {
    pub id: chunk::ID,
    pub tick: Tick,
    pub modified: Modified,
    pub position: IVec3,
    pub geometry: chunk::Geometry,
    pub block_list: Vec<block::Block>,
    pub visibility_list: Vec<Vec<grid::Direction>>,
}

impl Chunk {
    pub fn get_block(&self, block_id: block::ID) -> Option<&block::Block> {
        self.block_list.get(usize::from(block_id))
    }

    pub fn get_block_mut(&mut self, block_id: block::ID) -> Option<&mut block::Block> {
        self.block_list.get_mut(usize::from(block_id))
    }
}
