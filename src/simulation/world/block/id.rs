#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID(pub u32);

impl From<ID> for usize {
    fn from(id: ID) -> Self {
        id.0 as usize
    }
}

impl From<ID> for u32 {
    fn from(id: ID) -> Self {
        id.0
    }
}

impl From<ID> for i32 {
    fn from(id: ID) -> Self {
        id.0 as i32
    }
}
