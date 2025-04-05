pub mod action;
pub mod entity_action;
pub mod world_action;

pub use action::Action;
pub use entity_action::EntityAction;
pub use entity_action::JumpAction;
pub use entity_action::MovementAction;
pub use world_action::WorldAction;

use crate::simulation::population::entity;
use crate::simulation::population::entity::JumpStage;
use crate::simulation::state;
use crate::simulation::state::State;
use glam::Quat;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Actions {
    action_rx: UnboundedReceiver<Action>,
}

impl Actions {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Actions {
        let actions = Actions { action_rx };

        actions
    }

    pub fn tick(&mut self, state: &mut State) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::World(WorldAction::Quit) => {
                    self.handle_quit_action(state);
                }
                Action::Agent(EntityAction::Movement(movement_actions)) => {
                    self.handle_movement_action(state, &movement_actions);
                }
                Action::Agent(EntityAction::Jump(jump_action)) => {
                    self.handle_jump_action(state, &jump_action);
                }
            }
        }
    }

    fn handle_quit_action(&mut self, state: &mut State) {
        state.mode = state::Mode::Exit;
    }

    fn handle_movement_action(&mut self, state: &mut State, movement_actions: &MovementAction) {
        if let Some(entity) = state.population.get_mut(&entity::ID::USER_ENTITY) {
            entity.z_speed = movement_actions.direction.z;
            entity.x_speed = movement_actions.direction.x;

            if movement_actions.rotation.length_squared() > 1e-6 {
                entity.look_x_axis -= movement_actions.rotation.x;
                entity.look_y_axis += movement_actions.rotation.y;

                let limit = 89.0_f32.to_radians();

                entity.look_x_axis = entity.look_x_axis.clamp(-limit, limit);

                let y_axis_quat = Quat::from_rotation_y(entity.look_y_axis);
                let x_axis_quat = Quat::from_rotation_x(entity.look_x_axis);

                let target_rotation = y_axis_quat * x_axis_quat;

                entity.orientation = entity.orientation.slerp(target_rotation, 0.3);
            }
        }
    }

    fn handle_jump_action(&mut self, state: &mut State, jump_action: &JumpAction) {
        match jump_action {
            JumpAction::Start => {
                if let Some(entity) = state.population.get_mut(&entity::ID::USER_ENTITY) {
                    entity.jump_state.stage = JumpStage::Launch;
                    entity.jump_state.timer = 0;
                }
            }
            JumpAction::End => {
                if let Some(entity) = state.population.get_mut(&entity::ID::USER_ENTITY) {
                    entity.jump_state.stage = JumpStage::Fall;
                }
            }
        }
    }
}
