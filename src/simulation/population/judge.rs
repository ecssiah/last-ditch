pub mod id;
pub mod jump_state;

pub use id::ID;
pub use jump_state::JumpStage;
pub use jump_state::JumpState;

use crate::simulation::dispatch::JumpAction;
use crate::simulation::dispatch::MovementAction;
use crate::simulation::time::Tick;
use glam::{Quat, Vec3};

#[derive(Clone)]
pub struct Judge {
    pub id: ID,
    pub tick: Tick,
    pub name: String,
    pub position: Vec3,
    pub velocity: Vec3,
    pub chunk_update: bool,
    pub z_speed: f32,
    pub x_speed: f32,
    pub look_x_axis: f32,
    pub look_y_axis: f32,
    pub orientation: Quat,
    pub jump_state: JumpState,
}

impl Judge {
    pub fn new(judge_id: ID) -> Judge {
        let judge = Self {
            id: judge_id,
            tick: Tick::ZERO,
            name: "TEST JUDGE NAME".to_string(),
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            chunk_update: false,
            z_speed: 0.0,
            x_speed: 0.0,
            look_x_axis: 0.0,
            look_y_axis: 0.0,
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
        let x_axis = x_axis.to_radians();
        let y_axis = y_axis.to_radians();

        let limit = 89.0_f32.to_radians();

        self.look_x_axis = x_axis.clamp(-limit, limit);
        self.look_y_axis = y_axis;

        let y_axis_quat = Quat::from_rotation_y(self.look_y_axis);
        let x_axis_quat = Quat::from_rotation_x(self.look_x_axis);

        self.orientation = y_axis_quat * x_axis_quat;
    }

    pub fn apply_movement_action(&mut self, movement_action: &MovementAction) {
        self.z_speed = movement_action.direction.z;
        self.x_speed = movement_action.direction.x;

        if movement_action.rotation.length_squared() > 1e-6 {
            self.look_x_axis -= movement_action.rotation.x;
            self.look_y_axis += movement_action.rotation.y;

            let limit = 89.0_f32.to_radians();

            self.look_x_axis = self.look_x_axis.clamp(-limit, limit);

            let y_axis_quat = Quat::from_rotation_y(self.look_y_axis);
            let x_axis_quat = Quat::from_rotation_x(self.look_x_axis);

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
