use crate::simulation::state::world::block::block_shape::BlockShape;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BlockModelKey {
    pub block_shape: BlockShape,
}

impl BlockModelKey {
    pub fn from_block_shape(block_shape: &BlockShape) -> Self {
        Self {
            block_shape: block_shape.clone(),
        }
    }
}
