pub mod data;
pub mod id;
pub mod kind;
pub mod priority;
pub mod state;
pub mod store;

pub use id::ID;
pub use kind::Kind;
pub use priority::Priority;
pub use state::State;
pub use store::Store;

use std::cmp::Ordering;

pub struct Plan {
    pub id: ID,
    pub kind: Kind,
    pub priority: Priority,
}

impl Plan {
    pub fn new(kind: Kind, priority: Priority) -> Self {
        Self {
            id: ID::allocate(),
            kind,
            priority,
        }
    }

    pub fn create_idle_plan() -> Self {
        Self {
            id: ID::allocate(),
            kind: Kind::Idle,
            priority: Priority::Medium,
        }
    }

    pub fn create_travel_plan() -> Self {
        Self {
            id: ID::allocate(),
            kind: Kind::Travel,
            priority: Priority::Medium,
        }
    }
}

impl PartialEq for Plan {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Plan {}

impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Plan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
