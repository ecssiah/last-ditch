//! Allows Interface to send messages to Simulation

pub mod action;
pub mod agent_action;
pub mod test_action;
pub mod world_action;

pub use action::Action;
pub use agent_action::AgentAction;
pub use agent_action::JumpAction;
pub use agent_action::MovementAction;
pub use test_action::TestAction;
pub use world_action::WorldAction;

use crate::simulation::admin;
use crate::simulation::state::State;
use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

pub struct Dispatch {
    action_tx: Arc<UnboundedSender<Action>>,
    action_rx: UnboundedReceiver<Action>,
}

impl Dispatch {
    pub fn new() -> Self {
        let (action_tx, action_rx) = unbounded_channel();
        let action_tx = Arc::new(action_tx);

        Self {
            action_tx,
            action_rx,
        }
    }

    pub fn get_action_tx(&self) -> Arc<UnboundedSender<Action>> {
        self.action_tx.clone()
    }

    pub fn tick(&mut self, state: &mut State) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::Test(test_action) => {
                    self.handle_test_action(state, &test_action);
                }
                Action::Agent(agent_action) => {
                    self.handle_agent_action(state, &agent_action);
                }
                Action::World(world_action) => {
                    self.handle_world_action(state, &world_action);
                }
            }
        }
    }

    fn handle_test_action(&mut self, state: &mut State, test_action: &TestAction) {
        match test_action {
            TestAction::Test1 => {
                println!("Test Action 1");

                state.population.test_pathfinding_action(&state.world);
            }
            TestAction::Test2 => println!("Test Action 2"),
            TestAction::Test3 => println!("Test Action 3"),
            TestAction::Test4 => println!("Test Action 4"),
        }
    }

    fn handle_agent_action(&mut self, state: &mut State, agent_action: &AgentAction) {
        match agent_action {
            AgentAction::Jump(jump_action) => self.handle_jump_action(state, jump_action),
            AgentAction::Movement(movement_action) => {
                self.handle_movement_action(state, movement_action)
            }
        }
    }

    fn handle_world_action(&mut self, state: &mut State, world_action: &WorldAction) {
        match world_action {
            WorldAction::Exit => self.handle_exit_action(state),
        }
    }

    fn handle_jump_action(&mut self, state: &mut State, jump_action: &JumpAction) {
        state.population.judge.apply_jump_action(jump_action);
    }

    fn handle_movement_action(&mut self, state: &mut State, movement_action: &MovementAction) {
        state
            .population
            .judge
            .apply_movement_action(movement_action);
    }

    fn handle_exit_action(&mut self, state: &mut State) {
        state.admin.mode = admin::Mode::Exit;
    }
}
