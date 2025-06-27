use crate::simulation::{
    consts::*,
    observation::state_pair::StatePair,
    state::{
        physics::aabb::AABB,
        population::entity::{self, Detection, Kinematic, Nation, Spatial},
        receiver::action::{JumpAction, MovementAction},
        world::{chunk, World},
    },
};
use glam::{Quat, Vec3};

pub struct Judge {
    pub id: entity::ID,
    pub chunk_id: StatePair<chunk::ID>,
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
            chunk_id: StatePair::default(),
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
            .clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

        self.spatial.quaternion =
            Quat::from_rotation_y(self.spatial.yaw) * Quat::from_rotation_x(self.spatial.pitch);

        let velocity_xz = Vec3::new(self.kinematic.velocity.x, 0.0, self.kinematic.velocity.z);
        let speed = velocity_xz.length();

        if speed > 1e-6 {
            let forward = self.spatial.quaternion * Vec3::Z;
            let new_velocity_xz = forward.normalize() * speed;

            self.kinematic.velocity.x = new_velocity_xz.x;
            self.kinematic.velocity.z = new_velocity_xz.z;
        }
    }

    pub fn eye(&self) -> Vec3 {
        self.spatial.world_position + self.spatial.up() * 0.9 * self.size().y
    }
}

impl Default for Judge {
    fn default() -> Self {
        Self::new()
    }
}
