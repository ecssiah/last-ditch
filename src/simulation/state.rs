use crate::simulation::{
    agent::{self, Agent},
    time::Time,
    world::World,
};
use std::collections::HashMap;

pub struct State {
    pub active: bool,
    pub seed: u64,
    pub time: Time,
    pub agents: HashMap<agent::ID, Agent>,
    pub world: World,
}
