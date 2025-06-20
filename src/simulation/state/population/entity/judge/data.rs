use crate::simulation::{
    consts::*,
    dispatch::{JumpAction, MovementAction},
    state::{population::entity::Core, World},
};
use glam::{Quat, Vec3};

pub struct Data {
    pub core: Core,
}

impl Data {
    pub fn new() -> Self {
        Self { core: Core::new() }
    }

    pub fn tick(&mut self, world: &World) {
        if let Some(chunk_id) = world
            .grid
            .world_to_chunk_id(self.core.kinematics.world_position)
        {
            self.core.chunk_id.set(chunk_id);
        }
    }

    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw.to_radians();

        self.pitch = pitch.to_radians();
        self.pitch = self.pitch.clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

        self.quaternion = Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);

        let velocity_xz = Vec3::new(self.velocity.x, 0.0, self.velocity.z);
        let speed = velocity_xz.length();

        if speed > 1e-6 {
            let forward = self.orientation * Vec3::Z;
            let new_velocity_xz = forward.normalize() * speed;

            self.velocity.x = new_velocity_xz.x;
            self.velocity.z = new_velocity_xz.z;
        }
    }

    pub fn apply_movement_action(&mut self, movement_action: &MovementAction) {
        if movement_action.yaw.abs() > 1e-6 || movement_action.pitch.abs() > 1e-6 {
            self.orientation.yaw += movement_action.yaw;

            self.orientation.pitch += movement_action.pitch;
            self.orientation.pitch = self
                .orientation
                .pitch
                .clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

            self.orientation.quaternion = Quat::from_rotation_y(self.orientation.yaw)
                * Quat::from_rotation_x(self.orientation.pitch);
        }

        let input_direction = movement_action.direction.normalize_or_zero();

        if input_direction.length_squared() > 1e-6 {
            let yaw_quat = Quat::from_rotation_y(self.orientation.yaw);

            let local_velocity = input_direction * Vec3::new(JUDGE_SPEED_X, 0.0, JUDGE_SPEED_Z);
            let velocity = yaw_quat * local_velocity;

            self.core.velocity.x = velocity.x;
            self.core.velocity.z = velocity.z;
        } else {
            self.core.velocity.x = 0.0;
            self.core.velocity.z = 0.0;
        }
    }

    pub fn apply_jump_action(&mut self, jump_action: &JumpAction) {
        match jump_action {
            JumpAction::Start => {
                self.core.velocity.y = JUDGE_SPEED_Y;
            }
            _ => (),
        }
    }
}
