use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_ID: AtomicU32 = AtomicU32::new(100);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID(pub u32);

impl ID {
    pub const MAX: Self = Self(u32::MAX);

    pub fn allocate() -> ID {
        ID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl From<ID> for u32 {
    fn from(id: ID) -> Self {
        id.0
    }
}
