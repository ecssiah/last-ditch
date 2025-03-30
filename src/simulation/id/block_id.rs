#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockID(pub usize);

impl From<BlockID> for usize {
    fn from(block_id: BlockID) -> Self {
        block_id.0
    }
}
