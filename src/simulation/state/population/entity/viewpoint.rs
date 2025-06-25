use crate::simulation::state::physics::aabb::AABB;
use glam::{Mat4, Quat, Vec3};

pub const DEFAULT_FOV_Y_DEGREES: f32 = 179.99;
pub const DEFAULT_ASPECT_RATIO: f32 = 1.0;
pub const DEFAULT_NEAR_PLANE: f32 = 0.1;
pub const DEFAULT_FAR_PLANE: f32 = 1000.0;
pub const DEFAULT_BACK_OFFSET: f32 = -5.0;

pub struct Viewpoint {
    pub origin: Vec3,
    pub orientation: Quat,
    pub fov_y_radians: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Viewpoint {
    pub fn new() -> Self {
        Self {
            origin: Vec3::ZERO,
            orientation: Quat::IDENTITY,
            fov_y_radians: DEFAULT_FOV_Y_DEGREES.to_radians(),
            aspect_ratio: DEFAULT_ASPECT_RATIO,
            near: DEFAULT_NEAR_PLANE,
            far: DEFAULT_FAR_PLANE,
        }
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin + self.forward() * DEFAULT_BACK_OFFSET;
    }

    pub fn set_orientation(&mut self, orientation: Quat) {
        self.orientation = orientation;
    }

    pub fn is_front(&self, aabb: &AABB) -> bool {
        let center = aabb.center();

        let to_center = (center - self.origin).normalize();
        let view_direction = self.forward();

        view_direction.dot(to_center) >= 0.0
    }

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

impl Default for Viewpoint {
    fn default() -> Self {
        Self::new()
    }
}
