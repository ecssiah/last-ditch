pub mod id;
pub mod jump_state;

pub use id::ID;
pub use jump_state::JumpStage;
pub use jump_state::JumpState;

use crate::simulation::physics::aabb::AABB;
use crate::simulation::physics::dynamic_object::DynamicObject;
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
    pub acceleration: Vec3,
    pub size: Vec3,
    pub aabb: AABB,
    pub look: Vec3,
    pub quat: Quat,
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
            acceleration: Vec3::new(0.0, -GRAVITY_ACCELERATION, 0.0),
            size: Vec3::new(0.8, 2.1, 0.8),
            aabb: AABB::new(Vec3::ZERO, Vec3::new(0.8, 2.1, 0.8)),
            look: Vec3::ZERO,
            quat: Quat::default(),
            jump_state: JumpState {
                stage: JumpStage::Ground,
                timer: 0,
            },
        };

        judge
    }

    pub fn set_rotation(&mut self, x_axis: f32, y_axis: f32) {
        self.look.x = x_axis.to_radians();
        self.look.x = self.look.x.clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);

        self.look.y = y_axis.to_radians();

        let x_axis_quat = Quat::from_rotation_x(self.look.x);
        let y_axis_quat = Quat::from_rotation_y(self.look.y);

        self.quat = y_axis_quat * x_axis_quat;

        let flat_velocity = Vec3::new(self.velocity.x, 0.0, self.velocity.z);
        let speed = flat_velocity.length();

        if speed > 1e-6 {
            let forward = self.quat * Vec3::Z;
            let new_flat_velocity = forward.normalize() * speed;

            self.velocity.x = new_flat_velocity.x;
            self.velocity.z = new_flat_velocity.z;
        }
    }

    pub fn apply_movement_action(&mut self, movement_action: &MovementAction) {
        if movement_action.rotation.length_squared() > 1e-6 {
            self.look.x += movement_action.rotation.x;
            self.look.x = self.look.x.clamp(-JUDGE_VIEW_X_LIMIT, JUDGE_VIEW_X_LIMIT);
            self.look.y += movement_action.rotation.y;

            let x_axis_quat = Quat::from_rotation_x(self.look.x);
            let y_axis_quat = Quat::from_rotation_y(self.look.y);

            let rotation_target = y_axis_quat * x_axis_quat;

            self.quat = self.quat.slerp(rotation_target, 0.3);
        }

        let input_direction = movement_action.direction.normalize_or_zero();

        if input_direction.length_squared() > 1e-6 {
            let y_axis_quat = Quat::from_rotation_y(self.look.y);
            let local_velocity = input_direction * Vec3::new(DEFAULT_X_SPEED, 0.0, DEFAULT_Z_SPEED);
            let velocity = y_axis_quat * local_velocity;

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

                self.velocity.y = JUMP_LAUNCH_VELOCITY;
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

    fn position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.aabb.set_bottom_center(position);
    }

    fn velocity(&self) -> Vec3 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    fn acceleration(&self) -> Vec3 {
        self.acceleration
    }

    fn set_acceleration(&mut self, acceleration: Vec3) {
        self.acceleration = acceleration;
    }

    fn size(&self) -> Vec3 {
        self.size
    }

    fn aabb(&self) -> AABB {
        self.aabb.clone()
    }

    fn set_aabb(&mut self, aabb: AABB) {
        self.aabb = aabb;
    }
}
