use crate::simulation::state::world::block::block_kind::BlockKind;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BlockKey<'a> {
    block_kind: &'a BlockKind,
}

impl<'a> BlockKey<'a> {
    pub fn new(block_kind: &'a BlockKind) -> Self {
        Self { block_kind }
    }
}
