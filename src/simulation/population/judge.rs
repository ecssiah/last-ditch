pub mod id;
pub mod jump_state;

pub use id::ID;
pub use jump_state::JumpStage;
pub use jump_state::JumpState;

use crate::simulation::physics::aabb::AABB;
use crate::simulation::{
    consts::*,
    dispatch::{JumpAction, MovementAction},
    time::Tick,
    world::chunk,
};
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
    pub size: Vec3,
    pub aabb: AABB,
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
            velocity: Vec3::ZERO,
            size: Vec3::new(0.6, 2.1, 0.6),
            aabb: AABB::new(Vec3::ZERO, Vec3::new(0.6, 2.1, 0.6)),
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
        self.aabb = AABB::new(self.position, self.size);
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

        let flat_velocity = Vec3::new(self.velocity.x, 0.0, self.velocity.z);
        let speed = flat_velocity.length();

        if speed > 1e-6 {
            let forward = self.orientation * Vec3::Z;
            let new_flat_velocity = forward.normalize() * speed;
            self.velocity.x = new_flat_velocity.x;
            self.velocity.z = new_flat_velocity.z;
        }
    }

    pub fn apply_movement_action(&mut self, movement_action: &MovementAction) {
        if movement_action.rotation.length_squared() > 1e-6 {
            self.look.x -= movement_action.rotation.x;
            self.look.x = self.look.x.clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);
            self.look.y += movement_action.rotation.y;

            let x_axis_quat = Quat::from_rotation_x(self.look.x);
            let y_axis_quat = Quat::from_rotation_y(self.look.y);

            let target_rotation = y_axis_quat * x_axis_quat;

            self.orientation = self.orientation.slerp(target_rotation, 0.3);
        }

        let input_direction = movement_action.direction.normalize_or_zero();

        if input_direction.length_squared() > 1e-6 {
            let y_axis_quat = Quat::from_rotation_y(self.look.y);
            let local_velocity = y_axis_quat
                * Vec3::new(
                    DEFAULT_X_SPEED * input_direction.x,
                    0.0,
                    DEFAULT_Z_SPEED * input_direction.z,
                );

            self.velocity.x = local_velocity.x;
            self.velocity.z = local_velocity.z;
        } else {
            self.velocity.x = 0.0;
            self.velocity.z = 0.0;
        }
    }

    pub fn apply_jump_action(&mut self, jump_action: &JumpAction) {
        match jump_action {
            JumpAction::Start => {
                self.jump_state.stage = JumpStage::Launch;
                self.jump_state.timer = 0;

                self.velocity.y = JUMP_LAUNCH_VELOCITY;
            }
            JumpAction::End => {
                self.jump_state.stage = JumpStage::Fall;
            }
        }
    }
}
