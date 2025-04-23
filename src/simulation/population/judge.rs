pub mod id;
pub mod jump_state;

pub use id::ID;
pub use jump_state::JumpStage;
pub use jump_state::JumpState;

use crate::simulation::chunk;
use crate::simulation::dispatch::JumpAction;
use crate::simulation::dispatch::MovementAction;
use crate::simulation::time::Tick;
use crate::simulation::JUDGE_VIEW_X_LIMIT;
use glam::{Quat, Vec3};

#[derive(Clone)]
pub struct Judge {
    pub id: ID,
    pub tick: Tick,
    pub name: String,
    pub chunk_id: chunk::ID,
    pub chunk_update: bool,
    pub position: Vec3,
    pub velocity: Vec3,
    pub speed: Vec3,
    pub look: Vec3,
    pub orientation: Quat,
    pub jump_state: JumpState,
}

impl Judge {
    pub fn new(judge_id: ID) -> Judge {
        let judge = Self {
            id: judge_id,
            tick: Tick::ZERO,
            name: "TEST JUDGE NAME".to_string(),
            chunk_id: chunk::ID(0),
            chunk_update: false,
            position: Vec3::ZERO,
            speed: Vec3::ZERO,
            velocity: Vec3::ZERO,
            look: Vec3::ZERO,
            orientation: Quat::default(),
            jump_state: JumpState {
                stage: JumpStage::Ground,
                timer: 0,
            },
        };

        judge
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vec3::new(x, y, z);
    }

    pub fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.velocity = Vec3::new(x, y, z);
    }

    pub fn set_rotation(&mut self, x_axis: f32, y_axis: f32) {
        self.look.x = x_axis.to_radians();
        self.look.x = self.look.x.clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

        self.look.y = y_axis.to_radians();

        let x_axis_quat = Quat::from_rotation_x(self.look.x);
        let y_axis_quat = Quat::from_rotation_y(self.look.y);

        self.orientation = y_axis_quat * x_axis_quat;
    }

    pub fn apply_movement_action(&mut self, movement_action: &MovementAction) {
        self.speed.x = movement_action.direction.x;
        self.speed.z = movement_action.direction.z;

        if movement_action.rotation.length_squared() > 1e-6 {
            self.look.x -= movement_action.rotation.x;
            self.look.y += movement_action.rotation.y;

            self.look.x = self.look.x.clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

            let x_axis_quat = Quat::from_rotation_x(self.look.x);
            let y_axis_quat = Quat::from_rotation_y(self.look.y);

            let target_rotation = y_axis_quat * x_axis_quat;

            self.orientation = self.orientation.slerp(target_rotation, 0.3);
        }
    }

    pub fn apply_jump_action(&mut self, jump_action: &JumpAction) {
        match jump_action {
            JumpAction::Start => {
                self.jump_state.stage = JumpStage::Launch;
                self.jump_state.timer = 0;
            }
            JumpAction::End => {
                self.jump_state.stage = JumpStage::Fall;
            }
        }
    }
}
