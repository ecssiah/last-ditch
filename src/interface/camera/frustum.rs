use crate::interface::camera::plane::Plane;
use ultraviolet::{Mat4, Vec3};

pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_matrix(view_projection_matrix: &Mat4) -> Self {
        let view_projection_matrix_transposed = view_projection_matrix.transposed();

        let left = Self::extract(
            Vec3::new(
                view_projection_matrix_transposed.cols[3].x
                    + view_projection_matrix_transposed.cols[0].x,
                view_projection_matrix_transposed.cols[3].y
                    + view_projection_matrix_transposed.cols[0].y,
                view_projection_matrix_transposed.cols[3].z
                    + view_projection_matrix_transposed.cols[0].z,
            ),
            view_projection_matrix_transposed.cols[3].w
                + view_projection_matrix_transposed.cols[0].w,
        );

        let right = Self::extract(
            Vec3::new(
                view_projection_matrix_transposed.cols[3].x
                    - view_projection_matrix_transposed.cols[0].x,
                view_projection_matrix_transposed.cols[3].y
                    - view_projection_matrix_transposed.cols[0].y,
                view_projection_matrix_transposed.cols[3].z
                    - view_projection_matrix_transposed.cols[0].z,
            ),
            view_projection_matrix_transposed.cols[3].w
                - view_projection_matrix_transposed.cols[0].w,
        );

        let bottom = Self::extract(
            Vec3::new(
                view_projection_matrix_transposed.cols[3].x
                    + view_projection_matrix_transposed.cols[1].x,
                view_projection_matrix_transposed.cols[3].y
                    + view_projection_matrix_transposed.cols[1].y,
                view_projection_matrix_transposed.cols[3].z
                    + view_projection_matrix_transposed.cols[1].z,
            ),
            view_projection_matrix_transposed.cols[3].w
                + view_projection_matrix_transposed.cols[1].w,
        );

        let top = Self::extract(
            Vec3::new(
                view_projection_matrix_transposed.cols[3].x
                    - view_projection_matrix_transposed.cols[1].x,
                view_projection_matrix_transposed.cols[3].y
                    - view_projection_matrix_transposed.cols[1].y,
                view_projection_matrix_transposed.cols[3].z
                    - view_projection_matrix_transposed.cols[1].z,
            ),
            view_projection_matrix_transposed.cols[3].w
                - view_projection_matrix_transposed.cols[1].w,
        );

        let near = Self::extract(
            Vec3::new(
                view_projection_matrix_transposed.cols[3].x
                    + view_projection_matrix_transposed.cols[2].x,
                view_projection_matrix_transposed.cols[3].y
                    + view_projection_matrix_transposed.cols[2].y,
                view_projection_matrix_transposed.cols[3].z
                    + view_projection_matrix_transposed.cols[2].z,
            ),
            view_projection_matrix_transposed.cols[3].w
                + view_projection_matrix_transposed.cols[2].w,
        );

        let far = Self::extract(
            Vec3::new(
                view_projection_matrix_transposed.cols[3].x
                    - view_projection_matrix_transposed.cols[2].x,
                view_projection_matrix_transposed.cols[3].y
                    - view_projection_matrix_transposed.cols[2].y,
                view_projection_matrix_transposed.cols[3].z
                    - view_projection_matrix_transposed.cols[2].z,
            ),
            view_projection_matrix_transposed.cols[3].w
                - view_projection_matrix_transposed.cols[2].w,
        );

        Self {
            planes: [left, right, bottom, top, near, far],
        }
    }

    pub fn point_in_frustum(&self, p: Vec3) -> bool {
        for plane in &self.planes {
            if plane.normal.dot(p) + plane.d < 0.0 {
                return false;
            }
        }

        true
    }

    pub fn sphere_in_frustum(&self, center: Vec3, radius: f32) -> bool {
        for p in &self.planes {
            if p.normal.dot(center) + p.d <= -radius {
                return false;
            }
        }

        true
    }

    fn extract(normal: Vec3, d: f32) -> Plane {
        let len = normal.mag();

        Plane {
            normal: normal / len,
            d: d / len,
        }
    }
}
