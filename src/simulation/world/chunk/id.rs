use crate::simulation::CHUNK_ID_MAX;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID(pub usize);

impl From<ID> for usize {
    fn from(id: ID) -> Self {
        id.0
    }
}

impl ID {
    pub const MAX: Self = Self(CHUNK_ID_MAX);

    pub fn is_valid(chunk_id: ID) -> bool {
        (0..=CHUNK_ID_MAX).contains(&chunk_id.into())
    }
}
