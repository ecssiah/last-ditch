use glam::{Quat, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Spatial {
    pub world_position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub quaternion: Quat,
}

impl Spatial {
    pub fn new() -> Self {
        Self {
            world_position: Vec3::default(),
            yaw: 0.0,
            pitch: 0.0,
            quaternion: Quat::default(),
        }
    }

    pub fn forward(&self) -> Vec3 {
        self.quaternion * Vec3::Z
    }

    pub fn up(&self) -> Vec3 {
        self.quaternion * Vec3::Y
    }

    pub fn right(&self) -> Vec3 {
        self.quaternion * Vec3::X
    }
}
