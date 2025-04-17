use crate::{
    include_assets,
    interface::{gpu_chunk::gpu_vertex::GPUVertex, gpu_entity::GPUEntity},
};
use wgpu::{
    util::DeviceExt, CommandEncoder, Device, PipelineCompilationOptions, TextureFormat, TextureView,
};

pub struct EntityRenderer {
    pub shader_module: wgpu::ShaderModule,
    pub vertex_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub entity_vertices: Vec<GPUVertex>,
    pub gpu_entities: Vec<GPUEntity>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl EntityRenderer {
    pub fn new(device: &wgpu::Device, surface_format: &wgpu::TextureFormat) -> EntityRenderer {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Entity Shader"),
            source: wgpu::ShaderSource::Wgsl(include_assets!("shaders/entity.wgsl").into()),
        });

        let entity_sphere_vertices = Self::generate_sphere(8, 8);
        let entity_rectangle_vertices = Self::generate_rectangle();
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

        let render_pipeline = Self::create_render_pipeline(device, surface_format, &shader_module);

        let gpu_entities = Vec::new();

        let entity_renderer = EntityRenderer {
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
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Entity Render Pipeline Layout"),
                bind_group_layouts: &[],
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
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
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
        _depth_texture_view: &TextureView,
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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Entity Render Pass"),
                color_attachments: &[render_pass_color_attachment],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.draw(
                0..(self.entity_vertices.len() as u32),
                0..(self.gpu_entities.len() as u32),
            );

            drop(render_pass);
        }
    }

    fn generate_sphere(latitude_bands: u32, longitude_bands: u32) -> Vec<GPUVertex> {
        let mut vertices = Vec::new();

        for lat in 0..latitude_bands {
            let theta1 = (lat as f32) * std::f32::consts::PI / latitude_bands as f32;
            let theta2 = (lat as f32 + 1.0) * std::f32::consts::PI / latitude_bands as f32;

            for lon in 0..longitude_bands {
                let phi1 = (lon as f32) * 2.0 * std::f32::consts::PI / longitude_bands as f32;
                let phi2 = (lon as f32 + 1.0) * 2.0 * std::f32::consts::PI / longitude_bands as f32;

                let p1 = Self::spherical_to_cartesian(theta1, phi1);
                let p2 = Self::spherical_to_cartesian(theta2, phi1);
                let p3 = Self::spherical_to_cartesian(theta2, phi2);
                let p4 = Self::spherical_to_cartesian(theta1, phi2);

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

    fn spherical_to_cartesian(theta: f32, phi: f32) -> [f32; 3] {
        let x = phi.sin() * theta.sin();
        let y = theta.cos();
        let z = phi.cos() * theta.sin();
        [x, y, z]
    }

    fn generate_rectangle() -> Vec<GPUVertex> {
        let normal = [0.0, 0.0, 1.0];
        let light = 1.0;

        let uvs = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

        let positions = [
            [-0.5, -0.5, 0.0],
            [0.5, -0.5, 0.0],
            [0.5, 0.5, 0.0],
            [-0.5, -0.5, 0.0],
            [0.5, 0.5, 0.0],
            [-0.5, 0.5, 0.0],
        ];

        positions
            .iter()
            .enumerate()
            .map(|(i, &pos)| GPUVertex {
                position: pos,
                normal,
                uv: uvs[i % 4],
                light,
            })
            .collect()
    }
}
