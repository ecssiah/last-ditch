use crate::simulation::id::AgentID;
use std::sync::atomic::{AtomicU32, Ordering};

static NEXT: AtomicU32 = AtomicU32::new(0);

pub fn next() -> AgentID {
    AgentID(NEXT.fetch_add(1, Ordering::Relaxed))
}
