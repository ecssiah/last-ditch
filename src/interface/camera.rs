//! Translates Simulation viewpoint for Interface

pub mod camera_uniform_data;
pub mod frustum;
pub mod plane;

use crate::{
    interface::{
        camera::{camera_uniform_data::CameraUniformData, frustum::Frustum},
        constants::*,
    },
    simulation::{
        constants::ID_JUDGE_1,
        manager::viewer::view::{PersonView, View},
    },
};
use tracing::instrument;
use ultraviolet::{Mat4, Vec3, Vec4};

pub struct Camera {
    pub position: Vec3,
    pub right: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub view_projection_matrix: Mat4,
    pub frustum: Frustum,
    pub uniform_data: CameraUniformData,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new(device: &wgpu::Device) -> Self {
        let position = Vec3::zero();
        let right = Vec3::unit_x();
        let forward = Vec3::unit_y();
        let up = Vec3::unit_z();
        let view_matrix = Mat4::identity();
        let projection_matrix = Mat4::identity();
        let view_projection_matrix = Mat4::identity();
        let frustum = Frustum::from_matrix(&Mat4::identity());
        let uniform_data = CameraUniformData::new();

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera View Projection Buffer"),
            size: std::mem::size_of::<CameraUniformData>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("View Projection Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            position,
            right,
            forward,
            up,
            view_matrix,
            projection_matrix,
            view_projection_matrix,
            frustum,
            uniform_data,
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        }
    }

    #[instrument(skip_all)]
    pub fn apply_view(view: &View, camera: &mut Self) {
        if let Some(person_view) = view.population_view.person_view_map.get(&ID_JUDGE_1) {
            Self::update_camera(&person_view, camera);
        }

        let view_matrix_array = [
            *camera.view_matrix.cols[0].as_array(),
            *camera.view_matrix.cols[1].as_array(),
            *camera.view_matrix.cols[2].as_array(),
            *camera.view_matrix.cols[3].as_array(),
        ];

        let projection_matrix_array = [
            *camera.projection_matrix.cols[0].as_array(),
            *camera.projection_matrix.cols[1].as_array(),
            *camera.projection_matrix.cols[2].as_array(),
            *camera.projection_matrix.cols[3].as_array(),
        ];

        let view_projection_matrix_array = [
            *camera.view_projection_matrix.cols[0].as_array(),
            *camera.view_projection_matrix.cols[1].as_array(),
            *camera.view_projection_matrix.cols[2].as_array(),
            *camera.view_projection_matrix.cols[3].as_array(),
        ];

        camera.uniform_data = CameraUniformData {
            view_projection_matrix: view_projection_matrix_array,
            view_matrix: view_matrix_array,
            projection_matrix: projection_matrix_array,
            camera_position: *camera.position.as_array(),
            _padding: 0.0,
        };
    }

    fn update_camera(person_view: &PersonView, camera: &mut Self) {
        let projection_matrix =
            Self::get_projection_matrix(FOV_RADIANS, WINDOW_ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);

        camera.right = person_view.sight.rotor * Vec3::unit_x();
        camera.forward = person_view.sight.rotor * Vec3::unit_y();
        camera.up = person_view.sight.rotor * Vec3::unit_z();

        let target = person_view.sight.world_position + camera.forward;

        let view_matrix =
            Self::get_view_matrix(person_view.sight.world_position, target, camera.up);

        let view_projection_matrix = projection_matrix * view_matrix;

        camera.position = person_view.sight.world_position;
        camera.view_matrix = view_matrix;
        camera.projection_matrix = projection_matrix;
        camera.view_projection_matrix = view_projection_matrix;
        camera.frustum = Frustum::from_matrix(&view_projection_matrix);
    }

    fn get_view_matrix(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        let forward = Vec3::normalized(&(target - eye));
        let right = Vec3::normalized(&Vec3::cross(&forward, up));
        let up = Vec3::cross(&right, forward);

        Mat4::new(
            Vec4::new(right.x, up.x, -forward.x, 0.0),
            Vec4::new(right.y, up.y, -forward.y, 0.0),
            Vec4::new(right.z, up.z, -forward.z, 0.0),
            Vec4::new(
                -Vec3::dot(&right, eye),
                -Vec3::dot(&up, eye),
                -Vec3::dot(&-forward, eye),
                1.0,
            ),
        )
    }

    fn get_projection_matrix(
        vertical_fov: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> Mat4 {
        let y_scale = 1.0 / f32::tan(vertical_fov / 2.0);
        let x_scale = y_scale / aspect_ratio;
        let z_scale = z_far / (z_near - z_far);
        let z_offset = (z_near * z_far) / (z_near - z_far);

        Mat4::new(
            Vec4::new(x_scale, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y_scale, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z_scale, -1.0),
            Vec4::new(0.0, 0.0, z_offset, 0.0),
        )
    }
}
