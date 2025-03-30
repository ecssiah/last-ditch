use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_AGENT_ID: AtomicUsize = AtomicUsize::new(100);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AgentID(pub usize);

impl AgentID {
    pub const USER_AGENT_ID: AgentID = AgentID(0);

    pub fn allocate() -> AgentID {
        AgentID(NEXT_AGENT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl From<AgentID> for usize {
    fn from(agent_id: AgentID) -> Self {
        agent_id.0
    }
}
