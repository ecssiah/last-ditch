use crate::simulation::CHUNK_VOLUME;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID(pub usize);

impl From<ID> for usize {
    fn from(id: ID) -> Self {
        id.0
    }
}

impl ID {
    pub fn is_valid(block_id: ID) -> bool {
        (0..CHUNK_VOLUME).contains(&block_id.into())
    }
}
