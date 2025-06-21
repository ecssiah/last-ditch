pub mod action;

use crate::simulation::state::{
    admin,
    receiver::action::{Action, AgentAction, JumpAction, MovementAction, TestAction, WorldAction},
    State,
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Receiver {
    pub action_rx: UnboundedReceiver<Action>,
}

impl Receiver {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        Self { action_rx }
    }

    pub fn tick(&mut self, state: &mut State) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::Test(test_action) => self.handle_test_action(state, &test_action),
                Action::Agent(agent_action) => self.handle_agent_action(state, &agent_action),
                Action::World(world_action) => self.handle_world_action(state, &world_action),
            }
        }
    }

    fn handle_test_action(&mut self, state: &mut State, test_action: &TestAction) {
        match test_action {
            TestAction::Test1 => {
                state.population.test_chunk_path(&state.world);
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
