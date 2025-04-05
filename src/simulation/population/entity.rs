pub mod id;

pub use id::ID;

use crate::simulation::{population::entity, time::Tick};
use glam::{Quat, Vec3};

#[derive(Clone)]
pub enum JumpStage {
    Ground,
    Launch,
    Rise,
    Fall,
}

#[derive(Clone)]
pub struct JumpState {
    pub stage: JumpStage,
    pub timer: u32,
}

#[derive(Clone)]
pub struct Entity {
    pub id: entity::ID,
    pub tick: Tick,
    pub name: &'static str,
    pub position: Vec3,
    pub z_speed: f32,
    pub x_speed: f32,
    pub look_x_axis: f32,
    pub look_y_axis: f32,
    pub orientation: Quat,
    pub jump_state: JumpState,
}

impl Entity {
    pub fn new(entity_id: id::ID) -> Entity {
        let entity = Self {
            id: entity_id,
            tick: Tick::ZERO,
            name: "",
            position: Vec3::ZERO,
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

        entity
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vec3::new(x, y, z);
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
}
