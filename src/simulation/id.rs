use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AgentID(pub u32);

static NEXT_AGENT_ID: AtomicU32 = AtomicU32::new(0);

pub fn agent_id() -> AgentID {
    AgentID(NEXT_AGENT_ID.fetch_add(1, Ordering::Relaxed))
}
