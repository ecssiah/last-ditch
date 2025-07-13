pub mod data;
pub mod id;
pub mod kind;
pub mod store;

pub use id::ID;
pub use kind::Kind;
pub use store::Store;

#[derive(Clone, Debug)]
pub struct TaskOutput {
    pub id: ID,
    pub kind: Kind,
}

impl TaskOutput {
    pub fn new(kind: Kind) -> Self {
        Self {
            id: ID::allocate(),
            kind,
        }
    }
}
