use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_ID: AtomicU32 = AtomicU32::new(100);

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ID(pub u32);

impl ID {
    pub const MAX: Self = Self(u32::MAX);

    pub fn allocate() -> ID {
        ID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

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
