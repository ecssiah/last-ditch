#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ChunkID(pub usize);

impl From<ChunkID> for usize {
    fn from(chunk_id: ChunkID) -> Self {
        chunk_id.0
    }
}
