use crate::simulation::dispatch::{AgentAction, TestAction, WorldAction};

#[derive(Debug)]
pub enum Action {
    Test(TestAction),
    World(WorldAction),
    Agent(AgentAction),
}
