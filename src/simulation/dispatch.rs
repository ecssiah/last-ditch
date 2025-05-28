//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod action;
pub mod entity_action;
pub mod world_action;

pub use action::Action;
pub use entity_action::EntityAction;
pub use entity_action::JumpAction;
pub use entity_action::MovementAction;
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
    pub fn new() -> Dispatch {
        let (action_tx, action_rx) = unbounded_channel();
        let action_tx = Arc::new(action_tx);

        let dispatch = Dispatch {
            action_tx,
            action_rx,
        };

        dispatch
    }

    pub fn get_action_tx(&self) -> Arc<UnboundedSender<Action>> {
        self.action_tx.clone()
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
