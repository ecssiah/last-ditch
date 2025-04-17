use crate::{
    interface::consts::*,
    simulation::{self},
};
use glam::{Mat4, Vec3};

pub struct Camera {
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub view_projection_buffer: wgpu::Buffer,
    pub view_projection_bind_group: wgpu::BindGroup,
}

impl Camera {
    #[rustfmt::skip]
    const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.5,
        0.0, 0.0, 0.0, 1.0,
    ]);

    pub fn new(device: &wgpu::Device) -> Camera {
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let view_projection_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("View Projection Buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("View Projection Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_projection_buffer.as_entire_binding(),
            }],
        });

        let camera = Camera {
            uniform_bind_group_layout,
            view_projection_buffer,
            view_projection_bind_group,
        };

        camera
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        render_alpha: f32,
        entity_view: &simulation::observation::view::EntityView,
    ) {
        let view_projection_matrix = Self::create_view_projection_matrix(render_alpha, entity_view);

        queue.write_buffer(
            &self.view_projection_buffer,
            0,
            bytemuck::cast_slice(&view_projection_matrix),
        );
    }

    fn create_view_projection_matrix(
        render_alpha: f32,
        entity_view: &simulation::observation::view::EntityView,
    ) -> [[f32; 4]; 4] {
        let entity_position_interpolated = entity_view
            .position
            .lerp(entity_view.next_position, render_alpha);
        let entity_orientation_interpolated = entity_view
            .orientation
            .lerp(entity_view.next_orientation, render_alpha);

        let opengl_projection =
            Mat4::perspective_rh(FOV.to_radians(), WINDOW_ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);
        let projection = Self::OPENGL_TO_WGPU_MATRIX * opengl_projection;

        let forward = entity_orientation_interpolated * Vec3::Z;
        let up = entity_orientation_interpolated * Vec3::Y;

        let eye = entity_position_interpolated + simulation::consts::USER_VIEW_OFFSET * up;
        let target = eye + forward;

        let view = Mat4::look_at_rh(eye, target, up);

        let view_projection = projection * view;

        view_projection.to_cols_array_2d()
    }
}
