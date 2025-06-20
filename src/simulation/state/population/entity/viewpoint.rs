use glam::{Mat4, Quat, Vec3};

pub struct Viewpoint {
    pub origin: Vec3,
    pub orientation: Quat,
    pub fov_y_radians: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Viewpoint {
    pub fn forward(&self) -> Vec3 {
        self.orientation * Vec3::Z
    }

    pub fn up(&self) -> Vec3 {
        self.orientation * Vec3::Y
    }

    pub fn right(&self) -> Vec3 {
        self.orientation * Vec3::X
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_to_rh(self.origin, self.forward(), self.up())
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov_y_radians, self.aspect_ratio, self.near, self.far)
    }

    pub fn view_proj_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}
