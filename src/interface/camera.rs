use crate::{
    interface::{consts::*, render::data::CameraUniformData},
    simulation::{self},
};
use glam::{Mat4, Vec3};

pub struct Camera {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new(device: &wgpu::Device) -> Camera {
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("View Projection Buffer"),
            size: std::mem::size_of::<CameraUniformData>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
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

        let camera = Camera {
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        };

        camera
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        alpha: f32,
        judge_view: &simulation::observation::view::JudgeView,
    ) {
        let camera_uniform_data = Self::generate_camera_uniform_data(alpha, judge_view);

        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform_data]),
        );
    }

    fn generate_camera_uniform_data(
        alpha: f32,
        judge_view: &simulation::observation::view::JudgeView,
    ) -> CameraUniformData {
        let judge_position = judge_view
            .position
            .current
            .lerp(judge_view.position.next, alpha);

        let projection =
            Mat4::perspective_lh(FOV_RADIANS, WINDOW_ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);

        let eye = judge_position + Vec3::Y * judge_view.size.current.y;

        let forward = judge_view.orientation.current * Vec3::Z;
        let up = judge_view.orientation.current * Vec3::Y;
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
