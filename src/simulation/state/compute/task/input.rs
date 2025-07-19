pub mod data;
pub mod id;
pub mod priority;
pub mod store;

pub use id::ID;
pub use priority::Priority;
pub use store::Store;

use std::cmp::Ordering;
use crate::simulation::state::compute::task;

#[derive(Clone, Debug)]
pub struct Input {
    pub id: ID,
    pub priority: Priority,
    pub kind: task::Kind,
}

impl Input {
    pub fn new(kind: task::Kind) -> Self {
        Self {
            id: ID::allocate(),
            priority: Priority::Medium,
            kind,
        }
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Input {}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
