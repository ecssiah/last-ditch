//! Processes Actions

pub mod action;

use crate::simulation::state::receiver::action::Action;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Receiver {
    pub action_rx: UnboundedReceiver<Action>,
}

impl Receiver {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        Self { action_rx }
    }

    pub fn listen(receiver: &mut Receiver) -> Option<Vec<Action>> {
        let mut action_vec = Vec::new();

        while let Ok(action) = receiver.action_rx.try_recv() {
            if matches!(action, Action::Admin(action::AdminAction::Shutdown)) {
                return None;
            }

            action_vec.push(action);
        }

        Some(action_vec)
    }
}
