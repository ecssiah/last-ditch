//! Translates Simulation viewpoint for Interface

pub mod camera_uniform_data;
pub mod frustum;
pub mod plane;

use crate::{
    interface::{camera::{camera_uniform_data::CameraUniformData, frustum::Frustum}, consts::*},
    simulation::observation::view::JudgeView,
};
use ultraviolet::{projection, Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
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
            projection::perspective_gl(FOV_RADIANS, WINDOW_ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);

        let eye_offset = Vec3::unit_y() * 0.9 * judge_view.size.y;
        let eye = judge_view.world_position + eye_offset;

        let forward = judge_view.rotor * Vec3::unit_z();
        let up = judge_view.rotor * Vec3::unit_y();

        let target = eye + forward;

        let view_matrix = Mat4::look_at_lh(eye, target, up);
        let view_projection_matrix = projection_matrix * view_matrix;

        camera.position = eye;
        camera.view_matrix = view_matrix;
        camera.projection_matrix = projection_matrix;
        camera.view_projection_matrix = view_projection_matrix;
        camera.frustum = Frustum::from_matrix(&view_projection_matrix);
    }
}
