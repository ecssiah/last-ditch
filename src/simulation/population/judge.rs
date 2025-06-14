pub mod id;
pub mod jump_state;

pub use id::ID;
pub use jump_state::JumpStage;
pub use jump_state::JumpState;

use crate::simulation::physics::aabb::AABB;
use crate::simulation::physics::dynamic_object::DynamicObject;
use crate::simulation::world::World;
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
    pub chunk_id: chunk::ID,
    pub chunk_update: bool,
    pub world_position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub aabb: AABB,
    pub yaw: f32,
    pub pitch: f32,
    pub orientation: Quat,
    pub jump_state: JumpState,
}

impl Judge {
    pub fn new(judge_id: ID) -> Self {
        Self {
            id: judge_id,
            tick: Tick::ZERO,
            chunk_id: chunk::ID(0),
            chunk_update: false,
            world_position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::new(0.0, -GRAVITY_ACCELERATION, 0.0),
            aabb: AABB::new(Vec3::ZERO, Vec3::new(0.6, 2.2, 0.6)),
            yaw: 0.0,
            pitch: 0.0,
            orientation: Quat::default(),
            jump_state: JumpState {
                stage: JumpStage::Ground,
                timer: 0,
            },
        }
    }

    pub fn tick(&mut self, world: &World) {
        if let Some(chunk_id) = world.grid.world_to_chunk_id(self.world_position) {
            if chunk_id != self.chunk_id {
                self.chunk_update = true;
                self.chunk_id = chunk_id;
            }
        }
    }

    pub fn apply_movement_action(&mut self, movement_action: &MovementAction) {
        if movement_action.yaw.abs() > 1e-6 || movement_action.pitch.abs() > 1e-6 {
            self.yaw += movement_action.yaw;

            self.pitch += movement_action.pitch;
            self.pitch = self.pitch.clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

            self.orientation = Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);
        }

        let input_direction = movement_action.direction.normalize_or_zero();

        if input_direction.length_squared() > 1e-6 {
            let yaw_quat = Quat::from_rotation_y(self.yaw);

            let local_velocity = input_direction * Vec3::new(JUDGE_SPEED_X, 0.0, JUDGE_SPEED_Z);
            let velocity = yaw_quat * local_velocity;

            self.velocity.x = velocity.x;
            self.velocity.z = velocity.z;
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

                self.velocity.y = JUDGE_SPEED_Y;
            }
            JumpAction::End => {
                self.jump_state.stage = JumpStage::Fall;
            }
        }
    }
}

impl DynamicObject for Judge {
    fn chunk_id(&self) -> chunk::ID {
        self.chunk_id
    }

    fn chunk_update(&self) -> bool {
        self.chunk_update
    }

    fn set_chunk_update(&mut self, chunk_update: bool) {
        self.chunk_update = chunk_update;
    }

    fn world_position(&self) -> Vec3 {
        self.world_position
    }

    fn set_world_position(&mut self, x: f32, y: f32, z: f32) {
        self.world_position = Vec3::new(x, y, z);
        self.aabb.set_bottom_center(x, y, z);
    }

    fn velocity(&self) -> Vec3 {
        self.velocity
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.velocity = Vec3::new(x, y, z);
    }

    fn acceleration(&self) -> Vec3 {
        self.acceleration
    }

    fn set_acceleration(&mut self, x: f32, y: f32, z: f32) {
        self.acceleration = Vec3::new(x, y, z);
    }

    fn height(&self) -> f32 {
        self.aabb.max.y - self.aabb.min.y
    }

    fn pitch(&self) -> f32 {
        self.pitch
    }

    fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    fn yaw(&self) -> f32 {
        self.yaw
    }

    fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
    }

    fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw.to_radians();

        self.pitch = pitch.to_radians();
        self.pitch = self.pitch.clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

        self.orientation = Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);

        let velocity_xz = Vec3::new(self.velocity.x, 0.0, self.velocity.z);
        let speed = velocity_xz.length();

        if speed > 1e-6 {
            let forward = self.orientation * Vec3::Z;
            let new_velocity_xz = forward.normalize() * speed;

            self.velocity.x = new_velocity_xz.x;
            self.velocity.z = new_velocity_xz.z;
        }
    }

    fn aabb(&self) -> AABB {
        self.aabb.clone()
    }

    fn set_aabb(&mut self, aabb: AABB) {
        self.aabb = aabb;
    }
}
