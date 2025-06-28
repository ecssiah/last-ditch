//! Processes Actions

pub mod action;

use crate::simulation::state::{receiver::action::Action, State};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Receiver {
    pub action_rx: UnboundedReceiver<Action>,
}

impl Receiver {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        Self { action_rx }
    }

    pub fn tick(&mut self, state: &mut State) -> bool {
        while let Ok(action) = self.action_rx.try_recv() {
            if matches!(action, Action::Admin(action::AdminAction::Shutdown)) {
                return false;
            }

            state.receive_action(action);
        }

        true
    }
}
