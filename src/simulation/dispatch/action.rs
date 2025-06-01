use crate::simulation::dispatch::{AgentAction, WorldAction};

#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Agent(AgentAction),
}
