use crate::simulation::world::block;

#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) block_id: block::ID,
    pub(crate) group_id: u32,
}
