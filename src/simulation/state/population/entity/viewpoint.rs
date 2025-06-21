use crate::simulation::state::physics::aabb::AABB;
use glam::{Mat4, Quat, Vec3};

pub const DEFAULT_FOV_Y_DEGREES: f32 = 179.99;
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
        self.origin = origin + self.forward() * -6.0;
    }

    pub fn set_orientation(&mut self, orientation: Quat) {
        self.orientation = orientation;
    }

    pub fn intersects(&self, aabb: &AABB) -> bool {
        if aabb.contains_point(self.origin) {
            return true;
        }

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

    pub fn bounding_aabb(&self) -> AABB {
        let forward = self.forward();
        let up = self.up();
        let right = self.right();

        let near_center = self.origin + forward * self.near;
        let far_center = self.origin + forward * self.far;

        let half_fov_y = self.fov_y_radians * 0.5;
        let near_height = (half_fov_y.tan()) * self.near;
        let near_width = near_height * self.aspect_ratio;

        let far_height = (half_fov_y.tan()) * self.far;
        let far_width = far_height * self.aspect_ratio;

        let corners = [
            near_center + up * near_height + right * near_width,
            near_center + up * near_height - right * near_width,
            near_center - up * near_height + right * near_width,
            near_center - up * near_height - right * near_width,
            far_center + up * far_height + right * far_width,
            far_center + up * far_height - right * far_width,
            far_center - up * far_height + right * far_width,
            far_center - up * far_height - right * far_width,
        ];

        let mut min = corners[0];
        let mut max = corners[0];

        for &corner in &corners[1..] {
            min = min.min(corner);
            max = max.max(corner);
        }

        AABB { min, max }
    }
}
