use crate::{
    include_assets,
    interface::render::{GPUEntity, GPUVertex},
};
use glam::Vec3;
use wgpu::{
    util::DeviceExt, BindGroupLayout, CommandEncoder, Device, PipelineCompilationOptions,
    TextureFormat, TextureView,
};

pub struct EntityRender {
    pub shader_module: wgpu::ShaderModule,
    pub vertex_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub entity_vertices: Vec<GPUVertex>,
    pub gpu_entities: Vec<GPUEntity>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl EntityRender {
    pub fn new(
        device: &wgpu::Device,
        surface_format: &wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> EntityRender {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Entity Shader"),
            source: wgpu::ShaderSource::Wgsl(include_assets!("shaders/entity.wgsl").into()),
        });

        let entity_sphere_vertices = Self::generate_head_vertices(8, 8);
        let entity_rectangle_vertices = Self::generate_body_vertices();
        let entity_vertices = [entity_sphere_vertices, entity_rectangle_vertices].concat();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Entity Vertex Buffer"),
            contents: bytemuck::cast_slice(&entity_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Entity Instance Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let render_pipeline = Self::create_render_pipeline(
            device,
            surface_format,
            &shader_module,
            uniform_bind_group_layout,
        );

        let gpu_entities = Vec::new();

        let entity_renderer = EntityRender {
            shader_module,
            entity_vertices,
            vertex_buffer,
            instance_buffer,
            gpu_entities,
            render_pipeline,
        };

        entity_renderer
    }

    pub fn create_render_pipeline(
        device: &Device,
        surface_format: &TextureFormat,
        shader_module: &wgpu::ShaderModule,
        uniform_bind_group_layout: &BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Entity Render Pipeline Layout"),
                bind_group_layouts: &[uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Entity Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[GPUVertex::desc(), GPUEntity::desc()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: *surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        render_pipeline
    }

    pub fn render(
        &mut self,
        encoder: &mut CommandEncoder,
        texture_view: &TextureView,
        depth_texture_view: &TextureView,
        view_projection_bind_group: &wgpu::BindGroup,
    ) {
        let render_pass_color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        });

        if self.gpu_entities.len() > 0 {
            let depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Entity Render Pass"),
                color_attachments: &[render_pass_color_attachment],
                depth_stencil_attachment,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, view_projection_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.draw(
                0..(self.entity_vertices.len() as u32),
                0..(self.gpu_entities.len() as u32),
            );

            drop(render_pass);
        }
    }

    fn generate_head_vertices(latitude_bands: u32, longitude_bands: u32) -> Vec<GPUVertex> {
        let mut vertices = Vec::new();

        for lat in 0..latitude_bands {
            let theta1 = (lat as f32) * std::f32::consts::PI / latitude_bands as f32;
            let theta2 = (lat as f32 + 1.0) * std::f32::consts::PI / latitude_bands as f32;

            for lon in 0..longitude_bands {
                let phi1 = (lon as f32) * 2.0 * std::f32::consts::PI / longitude_bands as f32;
                let phi2 = (lon as f32 + 1.0) * 2.0 * std::f32::consts::PI / longitude_bands as f32;

                let p1 = 0.26 * Self::spherical_to_cartesian(theta1, phi1);
                let p2 = 0.26 * Self::spherical_to_cartesian(theta2, phi1);
                let p3 = 0.26 * Self::spherical_to_cartesian(theta2, phi2);
                let p4 = 0.26 * Self::spherical_to_cartesian(theta1, phi2);

                let p1 = [p1.x, p1.y + 1.9, p1.z];
                let p2 = [p2.x, p2.y + 1.9, p2.z];
                let p3 = [p3.x, p3.y + 1.9, p3.z];
                let p4 = [p4.x, p4.y + 1.9, p4.z];

                vertices.push(GPUVertex {
                    position: p1,
                    normal: p1,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });

                vertices.push(GPUVertex {
                    position: p2,
                    normal: p2,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });

                vertices.push(GPUVertex {
                    position: p3,
                    normal: p3,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });

                vertices.push(GPUVertex {
                    position: p1,
                    normal: p1,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });

                vertices.push(GPUVertex {
                    position: p3,
                    normal: p3,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });

                vertices.push(GPUVertex {
                    position: p4,
                    normal: p4,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
            }
        }

        vertices
    }

    fn spherical_to_cartesian(theta: f32, phi: f32) -> Vec3 {
        let x = phi.sin() * theta.sin();
        let y = theta.cos();
        let z = phi.cos() * theta.sin();

        Vec3::new(x, y, z)
    }

    fn generate_body_vertices() -> Vec<GPUVertex> {
        let top_half_width = 0.18;
        let bottom_half_width = 0.08;
        let half_height = 0.5;

        let normals = [
            [0.0, 0.0, 1.0],  // front
            [0.0, 0.0, -1.0], // back
            [1.0, 0.0, 0.0],  // right
            [-1.0, 0.0, 0.0], // left
            [0.0, 1.0, 0.0],  // top
            [0.0, -1.0, 0.0], // bottom
        ];

        let faces = vec![
            // front face
            [
                [-bottom_half_width, -half_height + 1.0, top_half_width],
                [bottom_half_width, -half_height + 1.0, top_half_width],
                [top_half_width, half_height + 1.0, top_half_width],
                [top_half_width, half_height + 1.0, top_half_width],
                [-top_half_width, half_height + 1.0, top_half_width],
                [-bottom_half_width, -half_height + 1.0, top_half_width],
            ],
            // back face
            [
                [bottom_half_width, -half_height + 1.0, -top_half_width],
                [-bottom_half_width, -half_height + 1.0, -top_half_width],
                [-top_half_width, half_height + 1.0, -top_half_width],
                [-top_half_width, half_height + 1.0, -top_half_width],
                [top_half_width, half_height + 1.0, -top_half_width],
                [bottom_half_width, -half_height + 1.0, -top_half_width],
            ],
            // right face
            [
                [bottom_half_width, -half_height + 1.0, top_half_width],
                [bottom_half_width, -half_height + 1.0, -top_half_width],
                [top_half_width, half_height + 1.0, -top_half_width],
                [top_half_width, half_height + 1.0, -top_half_width],
                [top_half_width, half_height + 1.0, top_half_width],
                [bottom_half_width, -half_height + 1.0, top_half_width],
            ],
            // left face
            [
                [-bottom_half_width, -half_height + 1.0, -top_half_width],
                [-bottom_half_width, -half_height + 1.0, top_half_width],
                [-top_half_width, half_height + 1.0, top_half_width],
                [-top_half_width, half_height + 1.0, top_half_width],
                [-top_half_width, half_height + 1.0, -top_half_width],
                [-bottom_half_width, -half_height + 1.0, -top_half_width],
            ],
            // top face
            [
                [-top_half_width, half_height + 1.0, top_half_width],
                [top_half_width, half_height + 1.0, top_half_width],
                [top_half_width, half_height + 1.0, -top_half_width],
                [top_half_width, half_height + 1.0, -top_half_width],
                [-top_half_width, half_height + 1.0, -top_half_width],
                [-top_half_width, half_height + 1.0, top_half_width],
            ],
            // bottom face
            [
                [-bottom_half_width, -half_height + 1.0, -top_half_width],
                [bottom_half_width, -half_height + 1.0, -top_half_width],
                [bottom_half_width, -half_height + 1.0, top_half_width],
                [bottom_half_width, -half_height + 1.0, top_half_width],
                [-bottom_half_width, -half_height + 1.0, top_half_width],
                [-bottom_half_width, -half_height + 1.0, -top_half_width],
            ],
        ];

        let mut vertices = Vec::new();

        for (i, face) in faces.iter().enumerate() {
            let normal = normals[i];

            for &position in face {
                vertices.push(GPUVertex {
                    position,
                    normal,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
            }
        }

        vertices
    }
}
