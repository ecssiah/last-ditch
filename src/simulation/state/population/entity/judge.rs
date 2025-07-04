use crate::simulation::{
    consts::*,
    state::{
        physics::aabb::AABB,
        population::entity::{self, Detection, Kinematic, Nation, Spatial},
        receiver::action::{JudgeAction, JumpAction, MovementData},
        world::{chunk, World},
    },
};
use glam::{Quat, Vec3};

pub struct Judge {
    pub id: entity::ID,
    pub chunk_id: chunk::ID,
    pub chunk_updated: bool,
    pub action_vec: Vec<JudgeAction>,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
    pub kind: entity::Kind,
    pub nation: Nation,
}

impl Judge {
    pub fn new() -> Self {
        Self {
            id: entity::ID::allocate(),
            chunk_id: chunk::ID(0),
            chunk_updated: false,
            action_vec: Vec::default(),
            spatial: Spatial::default(),
            kinematic: Kinematic::default(),
            detection: Detection::default(),
            kind: entity::Kind::Eagle,
            nation: Nation {
                kind: entity::Kind::Eagle,
            },
        }
    }

    pub fn tick(&mut self, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(self.spatial.world_position);

        if chunk_id != self.chunk_id {
            self.chunk_updated = true;
            self.chunk_id = chunk_id;
        }

        let action_vec = std::mem::take(&mut self.action_vec);

        for action in action_vec {
            match action {
                JudgeAction::Movement(movement_data) => self.apply_movement_data(&movement_data),
                JudgeAction::Jump(jump_action) => self.apply_jump_action(&jump_action),
            }
        }
    }

    pub fn set_world_position(&mut self, world_position: Vec3) {
        self.spatial.world_position = world_position;
        self.detection.set_world_position(world_position);
    }

    pub fn size(&self) -> Vec3 {
        self.detection.body.size()
    }

    pub fn set_size(&mut self, size: Vec3) {
        self.detection.body = AABB::new(self.detection.body.center(), size);
    }

    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.spatial.yaw = yaw.to_radians();
        self.spatial.pitch = pitch.to_radians();

        self.spatial.pitch = self
            .spatial
            .pitch
            .clamp(-JUDGE_PITCH_LIMIT, JUDGE_PITCH_LIMIT);

        self.spatial.quaternion =
            Quat::from_rotation_y(self.spatial.yaw) * Quat::from_rotation_x(self.spatial.pitch);

        let velocity_xz = Vec3::new(self.kinematic.velocity.x, 0.0, self.kinematic.velocity.z);
        let speed = velocity_xz.length_squared();

        if speed > 1e-12 {
            let forward = self.spatial.quaternion * Vec3::Z;
            let new_velocity_xz = forward.normalize() * speed;

            self.kinematic.velocity.x = new_velocity_xz.x;
            self.kinematic.velocity.z = new_velocity_xz.z;
        }
    }

    pub fn eye(&self) -> Vec3 {
        self.spatial.world_position + self.spatial.up() * 0.9 * self.size().y
    }

    pub fn receive_action(&mut self, judge_action: &JudgeAction) {
        self.action_vec.push(*judge_action);
    }

    pub fn apply_movement_data(&mut self, movement_data: &MovementData) {
        if movement_data.rotation.y.abs() > 1e-6 || movement_data.rotation.z.abs() > 1e-6 {
            self.spatial.yaw += movement_data.rotation.y;
            self.spatial.pitch += movement_data.rotation.z;

            self.spatial.pitch = self
                .spatial
                .pitch
                .clamp(-JUDGE_PITCH_LIMIT, JUDGE_PITCH_LIMIT);

            self.spatial.quaternion =
                Quat::from_rotation_y(self.spatial.yaw) * Quat::from_rotation_x(self.spatial.pitch);
        }

        let input_direction = movement_data.direction.normalize_or_zero();

        if input_direction.length_squared() > 1e-6 {
            let yaw_quat = Quat::from_rotation_y(self.spatial.yaw);

            let local_velocity = input_direction * Vec3::new(JUDGE_SPEED_X, 0.0, JUDGE_SPEED_Z);
            let velocity = yaw_quat * local_velocity;

            self.kinematic.velocity.x = velocity.x;
            self.kinematic.velocity.z = velocity.z;
        } else {
            self.kinematic.velocity.x = 0.0;
            self.kinematic.velocity.z = 0.0;
        }
    }

    pub fn apply_jump_action(&mut self, jump_action: &JumpAction) {
        if let JumpAction::Start = jump_action {
            self.kinematic.velocity.y = JUDGE_SPEED_Y;
        }
    }
}

impl Default for Judge {
    fn default() -> Self {
        Self::new()
    }
}
