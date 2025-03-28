pub mod agent;

pub use agent::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AgentID(pub u32);
