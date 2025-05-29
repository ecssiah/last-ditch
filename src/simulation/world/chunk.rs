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
    consts::*,
    time::Tick,
    world::{block, chunk, grid},
    BLOCK_MAP,
};
use glam::IVec3;

pub struct Chunk {
    pub id: chunk::ID,
    pub tick: Tick,
    pub updated: bool,
    pub position: IVec3,
    pub graph: chunk::Graph,
    pub geometry: chunk::Geometry,
    pub kind_list: Vec<block::Kind>,
    pub block_list: Box<[usize; CHUNK_VOLUME]>,
    pub visibility_list: Box<[Vec<grid::Direction>; CHUNK_VOLUME]>,
}

impl Chunk {
    pub fn get_block(&self, block_id: block::ID) -> Option<&block::Block> {
        if block::ID::is_valid(block_id) {
            let kind_id = self.block_list.get(usize::from(block_id))?;
            let kind = self.kind_list.get(usize::from(*kind_id))?;

            let block = BLOCK_MAP.get(&kind)?;

            Some(block)
        } else {
            None
        }
    }
}
