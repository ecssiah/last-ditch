use crate::simulation::BLOCK_ID_MAX;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID(pub usize);

impl From<ID> for usize {
    fn from(id: ID) -> Self {
        id.0
    }
}

impl ID {
    pub const MAX: Self = Self(BLOCK_ID_MAX);

    pub fn is_valid(block_id: ID) -> bool {
        (0..=BLOCK_ID_MAX).contains(&usize::from(block_id))
    }

    pub fn all() -> [Self; BLOCK_ID_MAX + 1] {
        std::array::from_fn(|i| Self(i))
    }
}
