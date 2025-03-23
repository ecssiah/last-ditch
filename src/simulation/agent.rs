use glam::{Quat, Vec3};

#[derive(Debug)]
pub struct Agent {
    pub id: u32,
    pub name: &'static str,
    pub position: Vec3,
    pub z_speed: f32,
    pub x_speed: f32,
    pub look_x_axis: f32,
    pub look_y_axis: f32,
    pub look_rotation: Quat,
}

impl Agent {
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
