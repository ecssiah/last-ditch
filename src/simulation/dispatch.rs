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
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Dispatch {
    action_rx: UnboundedReceiver<Action>,
}

impl Dispatch {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Dispatch {
        let dispatch = Dispatch { action_rx };

        dispatch
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
