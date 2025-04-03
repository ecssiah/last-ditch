use crate::simulation::{id::agent_id::AgentID, time::Tick};
use glam::{Quat, Vec3};
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct JumpState {
    pub active: bool,
    pub cancel: bool,
    pub timer: Duration,
}

#[derive(Copy, Clone)]
pub struct Agent {
    pub id: AgentID,
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

impl Agent {
    pub fn new(agent_id: AgentID) -> Agent {
        let agent = Self {
            id: agent_id,
            tick: Tick::ZERO,
            name: "",
            position: Vec3::ZERO,
            z_speed: 0.0,
            x_speed: 0.0,
            look_x_axis: 0.0,
            look_y_axis: 0.0,
            orientation: Quat::default(),
            jump_state: JumpState {
                active: false,
                cancel: false,
                timer: Duration::ZERO,
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

        self.orientation = y_axis_quat * x_axis_quat;
    }
}
