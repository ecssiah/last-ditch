use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(100);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ID(pub usize);

impl ID {
    pub const SYSTEM_ENTITY: ID = ID(0);
    
    pub const USER_ENTITY1: ID = ID(1);
    pub const USER_ENTITY2: ID = ID(2);
    pub const USER_ENTITY3: ID = ID(3);
    pub const USER_ENTITY4: ID = ID(4);
    pub const USER_ENTITY5: ID = ID(5);
    pub const USER_ENTITY6: ID = ID(6);
    pub const USER_ENTITY7: ID = ID(7);
    pub const USER_ENTITY8: ID = ID(8);
    pub const USER_ENTITY9: ID = ID(9);
    pub const USER_ENTITY10: ID = ID(10);

    pub fn allocate() -> ID {
        ID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl From<ID> for usize {
    fn from(id: ID) -> Self {
        id.0
    }
}
