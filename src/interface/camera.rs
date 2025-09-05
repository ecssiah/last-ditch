//! Translates Simulation viewpoint for Interface

pub mod camera_uniform_data;

use crate::{
    interface::{camera::camera_uniform_data::CameraUniformData, consts::*},
    simulation::observation::view::JudgeView,
};
use glam::{Mat4, Vec3};

pub struct Camera {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new(device: &wgpu::Device) -> Self {
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
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        }
    }

    pub fn apply_judge_view(&mut self, queue: &wgpu::Queue, judge_view: &JudgeView) {
        let camera_uniform_data = Self::setup_camera_uniform_data(judge_view);

        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform_data]),
        );
    }

    fn setup_camera_uniform_data(judge_view: &JudgeView) -> CameraUniformData {
        let projection =
            Mat4::perspective_lh(FOV_RADIANS, WINDOW_ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);

        let eye_offset = Vec3::Y * 0.9 * judge_view.size.y;
        let eye = judge_view.world_position + eye_offset;

        let forward = judge_view.quaternion * Vec3::Z;
        let up = judge_view.quaternion * Vec3::Y;
        let target = eye + forward;

        let view = Mat4::look_at_lh(eye, target, up);
        let view_projection = projection * view;

        CameraUniformData {
            view_projection_matrix: view_projection.to_cols_array_2d(),
            camera_position: eye.to_array(),
            _padding: 0.0,
        }
    }
}
