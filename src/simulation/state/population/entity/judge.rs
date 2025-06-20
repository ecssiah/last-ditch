use crate::simulation::{
    consts::*,
    dispatch::{JumpAction, MovementAction},
    observation::state_pair::StatePair,
    state::{
        population::entity::{self, Kinematics, Nation, Spatial},
        world::{chunk, World},
    },
};
use glam::{Quat, Vec3};

pub struct Judge {
    pub id: entity::ID,
    pub chunk_id: StatePair<chunk::ID>,
    pub spatial: Spatial,
    pub kinematics: Kinematics,
    pub kind: entity::Kind,
    pub nation: Nation,
}

impl Judge {
    pub fn new() -> Self {
        Self {
            id: entity::ID::allocate(),
            chunk_id: StatePair::new(chunk::ID::zero(), chunk::ID::zero()),
            spatial: Spatial::new(),
            kinematics: Kinematics::new(),
            kind: entity::Kind::Eagle,
            nation: Nation {
                kind: entity::Kind::Eagle,
            },
        }
    }

    pub fn tick(&mut self, world: &World) {
        if let Some(chunk_id) = world.grid.world_to_chunk_id(self.spatial.world_position) {
            self.chunk_id.set(chunk_id);
        }
    }

    pub fn chunk_updated(&self) -> bool {
        self.chunk_id.current != self.chunk_id.next
    }

    pub fn apply_movement_action(&mut self, movement_action: &MovementAction) {
        if movement_action.yaw.abs() > 1e-6 || movement_action.pitch.abs() > 1e-6 {
            self.spatial.yaw += movement_action.yaw;

            self.spatial.pitch += movement_action.pitch;
            self.spatial.pitch = self
                .spatial
                .pitch
                .clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

            self.spatial.quaternion =
                Quat::from_rotation_y(self.spatial.yaw) * Quat::from_rotation_x(self.spatial.pitch);
        }

        let input_direction = movement_action.direction.normalize_or_zero();

        if input_direction.length_squared() > 1e-6 {
            let yaw_quat = Quat::from_rotation_y(self.spatial.yaw);

            let local_velocity = input_direction * Vec3::new(JUDGE_SPEED_X, 0.0, JUDGE_SPEED_Z);
            let velocity = yaw_quat * local_velocity;

            self.kinematics.velocity.x = velocity.x;
            self.kinematics.velocity.z = velocity.z;
        } else {
            self.kinematics.velocity.x = 0.0;
            self.kinematics.velocity.z = 0.0;
        }
    }

    pub fn apply_jump_action(&mut self, jump_action: &JumpAction) {
        match jump_action {
            JumpAction::Start => {
                self.kinematics.velocity.y = JUDGE_SPEED_Y;
            }
            _ => (),
        }
    }

    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.spatial.yaw = yaw.to_radians();

        self.spatial.pitch = pitch.to_radians();
        self.spatial.pitch = self
            .spatial
            .pitch
            .clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

        self.spatial.quaternion =
            Quat::from_rotation_y(self.spatial.yaw) * Quat::from_rotation_x(self.spatial.pitch);

        let velocity_xz = Vec3::new(self.kinematics.velocity.x, 0.0, self.kinematics.velocity.z);
        let speed = velocity_xz.length();

        if speed > 1e-6 {
            let forward = self.spatial.quaternion * Vec3::Z;
            let new_velocity_xz = forward.normalize() * speed;

            self.kinematics.velocity.x = new_velocity_xz.x;
            self.kinematics.velocity.z = new_velocity_xz.z;
        }
    }
}
