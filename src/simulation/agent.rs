use glam::{Quat, Vec3};

#[derive(Debug)]
pub struct Agent {
    pub id: u32,
    pub name: String,
    pub position: Vec3,
    pub z_speed: f32,
    pub x_speed: f32,
    pub move_y_axis: f32,
    pub look_x_axis: f32,
    pub look_y_axis: f32,
    pub look_rotation: Quat,
}
