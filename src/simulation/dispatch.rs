pub mod action;
pub mod entity_action;
pub mod message;
pub mod world_action;

pub use action::Action;
pub use entity_action::EntityAction;
pub use entity_action::JumpAction;
pub use entity_action::MovementAction;
pub use world_action::WorldAction;

use crate::simulation::admin;
use crate::simulation::dispatch::message::Message;
use crate::simulation::state::State;
use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

pub struct Dispatch {
    runtime: tokio::runtime::Runtime,
    message_tx: Arc<UnboundedSender<Message>>,
    message_rx: UnboundedReceiver<Message>,
    action_tx: Arc<UnboundedSender<Action>>,
    action_rx: UnboundedReceiver<Action>,
}

impl Dispatch {
    pub fn new() -> Dispatch {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let (message_tx, message_rx) = tokio::sync::mpsc::unbounded_channel();
        let message_tx = Arc::new(message_tx);

        let (action_tx, action_rx) = unbounded_channel();
        let action_tx = Arc::new(action_tx);

        let dispatch = Dispatch {
            runtime,
            message_tx,
            message_rx,
            action_tx,
            action_rx,
        };

        dispatch
    }

    pub fn get_action_tx(&self) -> Arc<UnboundedSender<Action>> {
        self.action_tx.clone()
    }

    pub fn get_message_tx(&self) -> Arc<UnboundedSender<Message>> {
        self.message_tx.clone()
    }

    pub fn get_message_rx(&self) -> &UnboundedReceiver<Message> {
        &self.message_rx
    }

    pub fn tick(&mut self, state: &mut State) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::Agent(EntityAction::Movement(movement_actions)) => {
                    self.handle_movement_action(state, &movement_actions);
                }
                Action::Agent(EntityAction::Jump(jump_action)) => {
                    self.handle_jump_action(state, &jump_action);
                }
                Action::World(WorldAction::Exit) => {
                    self.handle_exit_action(state);
                }
            }
        }

        while let Ok(message) = self.message_rx.try_recv() {
            match message {}
        }
    }

    fn handle_exit_action(&mut self, state: &mut State) {
        state.admin.mode = admin::Mode::Exit;
    }

    fn handle_movement_action(&mut self, state: &mut State, movement_action: &MovementAction) {
        state
            .population
            .judge
            .apply_movement_action(movement_action);
    }

    fn handle_jump_action(&mut self, state: &mut State, jump_action: &JumpAction) {
        state.population.judge.apply_jump_action(jump_action);
    }
}
