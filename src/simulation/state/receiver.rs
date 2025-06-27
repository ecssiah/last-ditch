//! Processes Actions

pub mod action;

use crate::simulation::state::{
    receiver::action::{Action, AdminAction, JudgeAction, TestAction},
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
                Action::Admin(admin_action) => self.handle_admin_action(state, &admin_action),
                Action::Test(test_action) => self.handle_test_action(state, &test_action),
                Action::Judge(judge_action) => self.handle_judge_action(state, &judge_action),
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

    fn handle_admin_action(&mut self, state: &mut State, admin_action: &AdminAction) {
        state.admin.receive_action(admin_action);
    }

    fn handle_judge_action(&mut self, state: &mut State, judge_action: &JudgeAction) {
        let judge = state.population.get_judge_mut();

        judge.receive_action(judge_action);
    }
}
