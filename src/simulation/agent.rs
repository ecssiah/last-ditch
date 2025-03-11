use glam::{Quat, Vec3};

#[derive(Debug)]
pub struct Agent {
    pub id: u32,
    pub name: String,
    pub position: Vec3,
    pub speed: f32,
    pub strafe_speed: f32,
    pub angular_speed: f32,
    pub move_yaw: f32,
    pub look_pitch: f32,
    pub look_yaw: f32,
    pub look_rotation: Quat,
}
