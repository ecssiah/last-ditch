use crate::simulation::WORLD_VOLUME;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID(pub usize);

impl From<ID> for usize {
    fn from(id: ID) -> Self {
        id.0
    }
}

impl ID {
    pub fn valid(chunk_id: ID) -> bool {
        (0..WORLD_VOLUME).contains(&chunk_id.into())
    }
}
