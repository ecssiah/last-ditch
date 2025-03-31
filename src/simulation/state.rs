use crate::simulation::{
    agent::Agent,
    id::agent_id::AgentID,
    time::{Tick, Time},
    world::World,
};
use std::collections::HashMap;

pub struct LastUpdate {
    pub agents: Tick,
    pub world: Tick,
}

pub struct State {
    pub active: bool,
    pub seed: u64,
    pub last_update: LastUpdate,
    pub time: Time,
    pub agents: HashMap<AgentID, Agent>,
    pub world: World,
}
