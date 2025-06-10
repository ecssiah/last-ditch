use crate::simulation::world::chunk;

#[derive(Clone, Debug)]
pub struct Node {
    pub chunk_id: chunk::ID,
    pub group_id: u32,
}
