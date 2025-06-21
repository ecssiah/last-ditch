pub mod agent_action;
pub mod test_action;
pub mod world_action;

pub use agent_action::AgentAction;
pub use agent_action::JumpAction;
pub use agent_action::MovementAction;
pub use test_action::TestAction;
pub use world_action::WorldAction;

#[derive(Debug)]
pub enum Action {
    Test(TestAction),
    World(WorldAction),
    Agent(AgentAction),
}
