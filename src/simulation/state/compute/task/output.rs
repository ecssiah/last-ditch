pub mod data;
pub mod id;
pub mod store;

pub use id::ID;
pub use store::Store;

use crate::simulation::state::compute::task;

#[derive(Clone, Debug)]
pub struct Output {
    pub id: ID,
    pub kind: task::Kind,
}

impl Output {
    pub fn new(kind: task::Kind) -> Self {
        Self {
            id: ID::allocate(),
            kind,
        }
    }
}
