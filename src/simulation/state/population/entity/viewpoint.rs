use glam::{Mat4, Quat, Vec3};

use crate::simulation::state::physics::aabb::AABB;

pub const DEFAULT_FOV_Y_DEGREES: f32 = 160.0;
pub const DEFAULT_ASPECT_RATIO: f32 = 16.0 / 16.0;
pub const DEFAULT_NEAR_PLANE: f32 = 0.1;
pub const DEFAULT_FAR_PLANE: f32 = 1000.0;

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
        self.origin = origin;
    }

    pub fn set_orientation(&mut self, orientation: Quat) {
        self.orientation = orientation;
    }

    pub fn intersects(&self, aabb: &AABB) -> bool {
        if aabb.contains_point(self.origin) {
            return true;
        }

        let center = aabb.center();
        let radius = aabb.size().length() * 0.5;

        let to_center = center - self.origin;
        let distance = to_center.length();

        if distance + radius < self.near || distance - radius > self.far {
            return false;
        }

        let view_dir = self.orientation * Vec3::Z;
        let angle = view_dir.angle_between(to_center);

        let half_fov_y = self.fov_y_radians * 0.5;
        let vertical_ok = angle <= half_fov_y;

        let fov_x = 2.0 * (self.aspect_ratio * (half_fov_y).tan()).atan();
        let horizontal_ok = angle <= fov_x * 0.5;

        vertical_ok && horizontal_ok
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
