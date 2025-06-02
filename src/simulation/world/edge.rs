use crate::simulation::world::{chunk, grid};

#[derive(Debug)]
pub struct Edge {
    pub chunk_id: chunk::ID,
    pub direction: grid::Direction,
    pub cost: f32,
    pub clearance: u32,
    pub group_id: u32,
}
