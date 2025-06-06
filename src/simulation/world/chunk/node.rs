use crate::simulation::world::chunk;

#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) chunk_id: chunk::ID,
    pub(crate) group_id: u32,
}
