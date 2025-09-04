use crate::{
    include_assets,
    interface::{
        camera::Camera, debug::debug_vertex_data::DebugVertexData, gpu_context::GPUContext,
    },
};
use glam::Vec3;

pub struct DebugRender {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    vertex_capacity: usize,
    vertex_vec: Vec<DebugVertexData>,
}

impl DebugRender {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let vert_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Debug Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/debug.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Debug Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/debug.frag.wgsl").into(),
                    ),
                });

        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Debug Render Pipeline"),
                    bind_group_layouts: &[&camera.uniform_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            gpu_context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Debug Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &vert_shader_module,
                        entry_point: Some("main"),
                        buffers: &[DebugVertexData::desc()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &frag_shader_module,
                        entry_point: Some("main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: gpu_context.surface_config.format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::LineList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Cw,
                        cull_mode: None,
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: false,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                    cache: None,
                });

        let initial_capacity = 64;

        let vertex_buffer = gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Debug Lines Vertex Buffer"),
            size: (initial_capacity * std::mem::size_of::<DebugVertexData>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            render_pipeline,
            bind_group: camera.uniform_bind_group.clone(),
            vertex_buffer,
            vertex_capacity: initial_capacity,
            vertex_vec: Vec::new(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.vertex_vec.clear();
    }

    pub fn add_line(&mut self, position1: Vec3, position2: Vec3, color: [f32; 3]) {
        self.vertex_vec.push(DebugVertexData {
            position: position1.into(),
            color,
        });

        self.vertex_vec.push(DebugVertexData {
            position: position2.into(),
            color,
        });
    }

    pub fn add_ray(&mut self, origin: Vec3, direction: Vec3, length: f32, color: [f32; 3]) {
        self.add_line(
            origin,
            origin + direction.normalize_or_zero() * length,
            color,
        );
    }

    pub fn add_axes(&mut self, origin: Vec3, scale: f32) {
        self.add_line(origin, origin + Vec3::X * scale, [1.0, 0.1, 0.1]);
        self.add_line(origin, origin + Vec3::Y * scale, [0.1, 1.0, 0.1]);
        self.add_line(origin, origin + Vec3::Z * scale, [0.1, 0.1, 1.0]);
    }

    pub fn add_box(&mut self, min: Vec3, max: Vec3, color: [f32; 3]) {
        let (x0, y0, z0) = (min.x, min.y, min.z);
        let (x1, y1, z1) = (max.x, max.y, max.z);

        let edge_array = [
            (Vec3::new(x0, y0, z0), Vec3::new(x1, y0, z0)),
            (Vec3::new(x1, y0, z0), Vec3::new(x1, y1, z0)),
            (Vec3::new(x1, y1, z0), Vec3::new(x0, y1, z0)),
            (Vec3::new(x0, y1, z0), Vec3::new(x0, y0, z0)),
            (Vec3::new(x0, y0, z1), Vec3::new(x1, y0, z1)),
            (Vec3::new(x1, y0, z1), Vec3::new(x1, y1, z1)),
            (Vec3::new(x1, y1, z1), Vec3::new(x0, y1, z1)),
            (Vec3::new(x0, y1, z1), Vec3::new(x0, y0, z1)),
            (Vec3::new(x0, y0, z0), Vec3::new(x0, y0, z1)),
            (Vec3::new(x1, y0, z0), Vec3::new(x1, y0, z1)),
            (Vec3::new(x1, y1, z0), Vec3::new(x1, y1, z1)),
            (Vec3::new(x0, y1, z0), Vec3::new(x0, y1, z1)),
        ];

        for (position1, position2) in edge_array {
            self.add_line(position1, position2, color);
        }
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        gpu_context: &GPUContext,
        debug_render: &mut DebugRender,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if debug_render.vertex_vec.is_empty() {
            return;
        }

        if debug_render.vertex_capacity < debug_render.vertex_vec.len() {
            debug_render.vertex_capacity = debug_render.vertex_vec.len().next_power_of_two();

            debug_render.vertex_buffer =
                gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Debug Render Vertex Buffer"),
                    size: (debug_render.vertex_capacity * std::mem::size_of::<DebugVertexData>())
                        as u64,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        }

        gpu_context.queue.write_buffer(
            &debug_render.vertex_buffer,
            0,
            bytemuck::cast_slice(&debug_render.vertex_vec),
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Debug Renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&debug_render.render_pipeline);
        render_pass.set_bind_group(0, &debug_render.bind_group, &[]);
        render_pass.set_vertex_buffer(0, debug_render.vertex_buffer.slice(..));
        render_pass.draw(0..(debug_render.vertex_vec.len() as u32), 0..1);

        debug_render.clear();
    }
}
