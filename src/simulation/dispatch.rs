pub mod action;
pub mod entity_action;
pub mod world_action;

pub use action::Action;
pub use entity_action::EntityAction;
pub use entity_action::JumpAction;
pub use entity_action::MovementAction;
pub use world_action::WorldAction;

use crate::simulation::admin;
use crate::simulation::population::judge::JumpStage;
use crate::simulation::state::State;
use glam::Quat;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Dispatch {
    action_rx: UnboundedReceiver<Action>,
}

impl Dispatch {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Dispatch {
        let actions = Dispatch { action_rx };

        actions
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

    fn handle_movement_action(&mut self, state: &mut State, movement_actions: &MovementAction) {
        let judge = state.population.get_judge_mut();

        judge.z_speed = movement_actions.direction.z;
        judge.x_speed = movement_actions.direction.x;

        if movement_actions.rotation.length_squared() > 1e-6 {
            judge.look_x_axis -= movement_actions.rotation.x;
            judge.look_y_axis += movement_actions.rotation.y;

            let limit = 89.0_f32.to_radians();

            judge.look_x_axis = judge.look_x_axis.clamp(-limit, limit);

            let y_axis_quat = Quat::from_rotation_y(judge.look_y_axis);
            let x_axis_quat = Quat::from_rotation_x(judge.look_x_axis);

            let target_rotation = y_axis_quat * x_axis_quat;

            judge.orientation = judge.orientation.slerp(target_rotation, 0.3);
        }
    }

    fn handle_jump_action(&mut self, state: &mut State, jump_action: &JumpAction) {
        match jump_action {
            JumpAction::Start => {
                let judge = state.population.get_judge_mut();
                judge.jump_state.stage = JumpStage::Launch;
                judge.jump_state.timer = 0;
            }
            JumpAction::End => {
                let judge = state.population.get_judge_mut();
                judge.jump_state.stage = JumpStage::Fall;
            }
        }
    }
}
