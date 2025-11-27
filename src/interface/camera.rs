//! Translates Simulation viewpoint for Interface

pub mod camera_uniform_data;
pub mod frustum;
pub mod plane;

use crate::{
    interface::{
        camera::{camera_uniform_data::CameraUniformData, frustum::Frustum},
        constants::*,
    },
    simulation::viewer::JudgeView,
};
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
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        }
    }

    pub fn apply_judge_view(queue: &wgpu::Queue, judge_view: &JudgeView, camera: &mut Camera) {
        Self::update_camera(judge_view, camera);

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

        let camera_uniform_data = CameraUniformData {
            view_projection_matrix: view_projection_matrix_array,
            view_matrix: view_matrix_array,
            projection_matrix: projection_matrix_array,
            camera_position: *camera.position.as_array(),
            _padding: 0.0,
        };

        queue.write_buffer(
            &camera.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform_data]),
        );
    }

    fn update_camera(judge_view: &JudgeView, camera: &mut Camera) {
        let projection_matrix =
            Self::get_projection_matrix(FOV_RADIANS, WINDOW_ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);

        camera.right = judge_view.sight_rotor * Vec3::unit_x();
        camera.forward = judge_view.sight_rotor * Vec3::unit_y();
        camera.up = judge_view.sight_rotor * Vec3::unit_z();

        let target = judge_view.sight_world_position + camera.forward;

        let view_matrix = Self::get_view_matrix(judge_view.sight_world_position, target, camera.up);
        let view_projection_matrix = projection_matrix * view_matrix;

        camera.position = judge_view.sight_world_position;
        camera.view_matrix = view_matrix;
        camera.projection_matrix = projection_matrix;
        camera.view_projection_matrix = view_projection_matrix;
        camera.frustum = Frustum::from_matrix(&view_projection_matrix);
    }

    fn get_view_matrix(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        let y_unit = Vec3::normalized(&(target - eye));
        let x_unit = Vec3::normalized(&Vec3::cross(&y_unit, up));
        let z_unit = Vec3::cross(&x_unit, y_unit);

        Mat4::new(
            Vec4::new(x_unit.x, z_unit.x, y_unit.x, 0.0),
            Vec4::new(x_unit.y, z_unit.y, y_unit.y, 0.0),
            Vec4::new(x_unit.z, z_unit.z, y_unit.z, 0.0),
            Vec4::new(
                -Vec3::dot(&x_unit, eye),
                -Vec3::dot(&z_unit, eye),
                -Vec3::dot(&y_unit, eye),
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
        let z_scale = z_far / (z_far - z_near);
        let z_offset = (-z_near * z_far) / (z_far - z_near);

        Mat4::new(
            Vec4::new(x_scale, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y_scale, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z_scale, 1.0),
            Vec4::new(0.0, 0.0, z_offset, 0.0),
        )
    }
}
