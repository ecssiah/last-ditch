use super::block::Block;
use crate::consts::CHUNK_VOLUME;
use cgmath::Point3;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: u32,
    pub local_position: Point3<i32>,
    pub world_position: Point3<f32>,
    pub modified: bool,
    pub blocks: Box<[Block; CHUNK_VOLUME as usize]>,
}
