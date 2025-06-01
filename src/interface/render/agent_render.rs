use crate::{
    include_assets,
    interface::render::data::{AgentInstanceData, VertexData},
};
use glam::Vec3;
use wgpu::{
    util::DeviceExt, BindGroupLayout, CommandEncoder, Device, PipelineCompilationOptions,
    TextureFormat, TextureView,
};

pub struct AgentRender {
    pub shader_module: wgpu::ShaderModule,
    pub vertex_data_list: Vec<VertexData>,
    pub instance_data_list: Vec<AgentInstanceData>,
    pub vertex_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl AgentRender {
    pub fn new(
        device: &wgpu::Device,
        surface_format: &wgpu::TextureFormat,
        camera_uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> AgentRender {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Agent Shader"),
            source: wgpu::ShaderSource::Wgsl(include_assets!("shaders/agent.wgsl").into()),
        });

        let vertex_data_list = Self::setup_agent_vertex_datas();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Agent Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data_list),
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
            camera_uniform_bind_group_layout,
        );

        let instance_data_list = Vec::new();

        let agent_render = AgentRender {
            shader_module,
            vertex_data_list,
            instance_data_list,
            vertex_buffer,
            instance_buffer,
            render_pipeline,
        };

        agent_render
    }

    pub fn create_render_pipeline(
        device: &Device,
        surface_format: &TextureFormat,
        shader_module: &wgpu::ShaderModule,
        camera_uniform_bind_group_layout: &BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Entity Render Pipeline Layout"),
                bind_group_layouts: &[camera_uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Entity Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[VertexData::desc(), AgentInstanceData::desc()],
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
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 1.0,
                    clamp: 0.0,
                },
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
        camera_data_bind_group: &wgpu::BindGroup,
    ) {
        let render_pass_color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        });

        if self.instance_data_list.len() > 0 {
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

            render_pass.set_bind_group(0, camera_data_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            render_pass.draw(
                0..(self.vertex_data_list.len() as u32),
                0..(self.instance_data_list.len() as u32),
            );

            drop(render_pass);
        }
    }

    fn setup_agent_vertex_datas() -> Vec<VertexData> {
        let head_bands = 6;
        let head_radius = 0.16;
        let head_height = 0.84;

        let body_height = 0.62;
        let body_top_width = 0.24;
        let body_bottom_width = 0.10;
        let body_top_half_width = body_top_width / 2.0;
        let body_bottom_half_width = body_bottom_width / 2.0;

        let mut vertices = Self::generate_uv_sphere(
            Vec3::new(0.0, head_height, 0.0),
            head_radius,
            head_bands,
            head_bands,
        );

        let normals = [
            [0.0, 0.0, 1.0],  // front
            [0.0, 0.0, -1.0], // back
            [1.0, 0.0, 0.0],  // right
            [-1.0, 0.0, 0.0], // left
            [0.0, 1.0, 0.0],  // top
            [0.0, -1.0, 0.0], // bottom
        ];

        #[rustfmt::skip]
        let faces = vec![
            // front face
            [
                [
                    -body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
                [
                    body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    -body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    -body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
            ],
            // back face
            [
                [
                    body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
                [
                    -body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
                [
                    -body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
                [
                    -body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
                [
                    body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
            ],
            // right face
            [
                [
                    body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
                [
                    body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
            ],
            // left face
            [
                [
                    -body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
                [
                    -body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
                [
                    -body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    -body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    -body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
                [
                    -body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
            ],
            // top face
            [
                [
                    -body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    -body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    body_top_half_width,
                ],
                [
                    body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
                [
                    -body_top_half_width,
                    body_height,
                    -body_top_half_width,
                ],
            ],
            // bottom face
            [
                [
                    -body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
                [
                    body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
                [
                    body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
                [
                    body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
                [
                    -body_bottom_half_width,
                    0.0,
                    body_bottom_half_width,
                ],
                [
                    -body_bottom_half_width,
                    0.0,
                    -body_bottom_half_width,
                ],
            ],
        ];

        for (i, face) in faces.iter().enumerate() {
            let normal = normals[i];

            for &position in face {
                vertices.push(VertexData {
                    position,
                    normal,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
            }
        }

        vertices
    }

    fn generate_uv_sphere(
        center: Vec3,
        radius: f32,
        latitude_bands: usize,
        longitude_bands: usize,
    ) -> Vec<VertexData> {
        let mut vertices = Vec::new();

        for lat in 0..latitude_bands {
            let theta1 = (lat as f32) * std::f32::consts::PI / latitude_bands as f32;
            let theta2 = (lat as f32 + 1.0) * std::f32::consts::PI / latitude_bands as f32;

            for lon in 0..longitude_bands {
                let phi1 = (lon as f32) * 2.0 * std::f32::consts::PI / longitude_bands as f32;
                let phi2 = (lon as f32 + 1.0) * 2.0 * std::f32::consts::PI / longitude_bands as f32;

                let p1 = center + radius * Self::spherical_to_cartesian(theta1, phi1);
                let p2 = center + radius * Self::spherical_to_cartesian(theta2, phi1);
                let p3 = center + radius * Self::spherical_to_cartesian(theta2, phi2);
                let p4 = center + radius * Self::spherical_to_cartesian(theta1, phi2);

                let normal1 = (p2 - p1).cross(p3 - p1).normalize();
                vertices.push(VertexData {
                    position: p1.into(),
                    normal: normal1.into(),
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
                vertices.push(VertexData {
                    position: p2.into(),
                    normal: normal1.into(),
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
                vertices.push(VertexData {
                    position: p3.into(),
                    normal: normal1.into(),
                    uv: [0.0, 0.0],
                    light: 1.0,
                });

                let normal2 = (p3 - p1).cross(p4 - p1).normalize();
                vertices.push(VertexData {
                    position: p1.into(),
                    normal: normal2.into(),
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
                vertices.push(VertexData {
                    position: p3.into(),
                    normal: normal2.into(),
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
                vertices.push(VertexData {
                    position: p4.into(),
                    normal: normal2.into(),
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
}
