use crate::simulation::{
    agent::Agent,
    chunk::Chunk,
    id::agent_id::AgentID,
    time::{Tick, Time},
    CHUNK_VOLUME,
};
use std::collections::HashMap;

pub struct LastUpdate {
    pub agents: Tick,
    pub chunks: Tick,
}

pub struct State {
    pub active: bool,
    pub seed: u64,
    pub last_update: LastUpdate,
    pub time: Time,
    pub agents: HashMap<AgentID, Agent>,
    pub chunks: [Chunk; CHUNK_VOLUME],
}
