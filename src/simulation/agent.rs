use crate::simulation::id::{self, AgentID};
use glam::{Quat, Vec3};

#[derive(Clone)]
pub struct JumpState {
    pub active: bool,
    pub cancel: bool,
    pub timer: f32,
}

#[derive(Clone)]
pub struct Agent {
    pub id: AgentID,
    pub name: String,
    pub position: Vec3,
    pub z_speed: f32,
    pub x_speed: f32,
    pub look_x_axis: f32,
    pub look_y_axis: f32,
    pub look_rotation: Quat,
    pub jump_state: JumpState,
}

impl Agent {
    pub fn new() -> Agent {
        let agent = Self {
            id: id::agent_id(),
            name: String::from(""),
            position: Vec3::ZERO,
            z_speed: 0.0,
            x_speed: 0.0,
            look_x_axis: 0.0,
            look_y_axis: 0.0,
            look_rotation: Quat::default(),
            jump_state: JumpState {
                active: false,
                cancel: false,
                timer: 0.0,
            },
        };

        agent
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

        self.look_rotation = y_axis_quat * x_axis_quat;
    }
}
