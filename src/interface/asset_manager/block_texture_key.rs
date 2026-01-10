use crate::simulation::state::world::block::block_kind::BlockKind;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BlockTextureKey {
    pub block_kind: BlockKind,
}

impl BlockTextureKey {
    pub fn from_block_kind(block_kind: &BlockKind) -> Self {
        Self {
            block_kind: block_kind.clone(),
        }
    }
}
