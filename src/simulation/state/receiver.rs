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
            match action {
                Action::Admin(admin_action) => match admin_action {
                    action::AdminAction::Exit => return false,
                    action::AdminAction::Start => state.receive_action(action),
                },
                other => state.receive_action(other),
            }
        }
        
        true
    }
}
