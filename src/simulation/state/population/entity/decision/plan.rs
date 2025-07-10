pub mod kind;
pub mod priority;
pub mod state;

pub use kind::Kind;
pub use priority::Priority;
pub use state::State;

pub struct Plan {
    pub priority: Priority,
    pub kind: Kind,
    pub state: State,
}


