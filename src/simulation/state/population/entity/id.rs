use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(100);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID(pub usize);

impl ID {
    pub fn allocate() -> ID {
        ID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for ID {
    fn default() -> Self {
        Self(0)
    }
}

impl From<ID> for usize {
    fn from(id: ID) -> Self {
        id.0
    }
}
