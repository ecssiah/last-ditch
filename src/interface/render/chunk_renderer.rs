use crate::{
    include_assets,
    interface::{
        consts::WINDOW_CLEAR_COLOR,
        gpu_chunk::{gpu_vertex::GPUVertex, GPUChunk},
    },
};
use wgpu::{BindGroupLayout, CommandEncoder, Device, TextureFormat, TextureView};

pub struct ChunkRenderer {
    pub gpu_chunks: Vec<GPUChunk>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl ChunkRenderer {
    pub fn new(
        device: &wgpu::Device,
        surface_format: &wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
        texture_sampler_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> ChunkRenderer {
        let gpu_chunks = Vec::new();

        let render_pipeline = Self::setup(
            device,
            surface_format,
            uniform_bind_group_layout,
            texture_sampler_bind_group_layout,
        );

        let chunk_renderer = ChunkRenderer {
            gpu_chunks,
            render_pipeline,
        };

        chunk_renderer
    }

    pub fn setup(
        device: &Device,
        surface_format: &TextureFormat,
        uniform_bind_group_layout: &BindGroupLayout,
        texture_sampler_bind_group_layout: &BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let chunk_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Chunk Shader"),
            source: wgpu::ShaderSource::Wgsl(include_assets!("shaders/chunk.wgsl").into()),
        });

        let chunk_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Chunk Pipeline Layout"),
                bind_group_layouts: &[
                    &uniform_bind_group_layout,
                    &texture_sampler_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let chunk_pipeline = Self::create_chunk_render_pipeline(
            &device,
            &chunk_pipeline_layout,
            &chunk_shader,
            surface_format,
        );

        chunk_pipeline
    }

    fn create_chunk_render_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        shader_module: &wgpu::ShaderModule,
        surface_format: &wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Chunk Mesh Pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: Some("vs_main"),
                buffers: &[GPUVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: *surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
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
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }

    pub fn render(
        &mut self,
        encoder: &mut CommandEncoder,
        texture_view: &TextureView,
        depth_texture_view: &TextureView,
        view_projection_bind_group: &wgpu::BindGroup,
        texture_sampler_bind_group: &wgpu::BindGroup,
    ) {
        let render_pass_color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: WINDOW_CLEAR_COLOR[0],
                    g: WINDOW_CLEAR_COLOR[1],
                    b: WINDOW_CLEAR_COLOR[2],
                    a: WINDOW_CLEAR_COLOR[3],
                }),
                store: wgpu::StoreOp::Store,
            },
        });

        let depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Chunk Render Pass"),
            color_attachments: &[render_pass_color_attachment],
            depth_stencil_attachment: depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, view_projection_bind_group, &[]);
        render_pass.set_bind_group(1, texture_sampler_bind_group, &[]);

        for gpu_chunk in self.gpu_chunks.iter() {
            if gpu_chunk.gpu_mesh.index_count > 0 {
                render_pass.set_vertex_buffer(0, gpu_chunk.gpu_mesh.vertex_buffer.slice(..));

                render_pass.set_index_buffer(
                    gpu_chunk.gpu_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                render_pass.draw_indexed(0..gpu_chunk.gpu_mesh.index_count, 0, 0..1);
            }
        }

        drop(render_pass);
    }
}
