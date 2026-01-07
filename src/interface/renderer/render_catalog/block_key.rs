use crate::simulation::state::world::block::block_kind::BlockKind;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BlockKey {
    block_kind: BlockKind,
}

impl BlockKey {
    pub fn new(block_kind: &BlockKind) -> Self {
        Self {
            block_kind: block_kind.clone(),
        }
    }
}
