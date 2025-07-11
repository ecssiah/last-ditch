pub mod data;
pub mod id;
pub mod kind;
pub mod priority;
pub mod store;

pub use id::ID;
pub use kind::Kind;
pub use priority::Priority;
pub use store::Store;

use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Task {
    pub id: ID,
    pub priority: Priority,
    pub kind: Kind,
}

impl Task {
    pub fn new(priority: Priority, kind: Kind) -> Self {
        Self {
            id: ID::allocate(),
            priority,
            kind,
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
