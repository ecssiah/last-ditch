use crate::simulation::{agent::Agent, id::agent_id::AgentID, time::Time, world::World};
use std::collections::HashMap;

pub struct State {
    pub active: bool,
    pub seed: u64,
    pub time: Time,
    pub agents: HashMap<AgentID, Agent>,
    pub world: World,
}
