pub mod block_kind;
pub mod block_shape;
pub mod block_state;

use crate::simulation::state::world::block::{block_kind::BlockKind, block_state::BlockState};

#[derive(Clone)]
pub struct Block {
    pub block_kind: BlockKind,
    pub block_state: BlockState,
}

impl Block {
    pub fn new(block_kind: &BlockKind, block_state: &BlockState) -> Self {
        Self {
            block_kind: block_kind.clone(),
            block_state: block_state.clone(),
        }
    }
}
