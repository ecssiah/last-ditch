//! Act listener

use crate::simulation::state::{action::act::Act, Action};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Receiver {
    pub act_rx: UnboundedReceiver<Act>,
}

impl Receiver {
    pub fn new(act_rx: UnboundedReceiver<Act>) -> Self {
        Self {
            act_rx,
        }
    }

    pub fn tick(receiver: &mut Receiver, action: &mut Action) {
        while let Ok(act) = receiver.act_rx.try_recv() {
            action.act_deque.push_back(act);
        }
    }
}
