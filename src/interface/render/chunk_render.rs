use crate::{
    include_assets,
    interface::{
        consts::WINDOW_CLEAR_COLOR,
        render::data::{ChunkData, VertexData},
    },
};
use wgpu::{BindGroupLayout, CommandEncoder, Device, TextureFormat, TextureView};

pub struct ChunkRender {
    pub shader_module: wgpu::ShaderModule,
    pub chunk_data_list: Vec<ChunkData>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl ChunkRender {
    pub fn new(
        device: &wgpu::Device,
        surface_format: &wgpu::TextureFormat,
        fog_uniform_bind_group_layout: &wgpu::BindGroupLayout,
        camera_uniform_bind_group_layout: &wgpu::BindGroupLayout,
        texture_sampler_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> ChunkRender {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Chunk Shader"),
            source: wgpu::ShaderSource::Wgsl(include_assets!("shaders/chunk.wgsl").into()),
        });

        let chunk_data_list = Vec::new();

        let render_pipeline = Self::create_render_pipeline(
            device,
            surface_format,
            &shader_module,
            fog_uniform_bind_group_layout,
            camera_uniform_bind_group_layout,
            texture_sampler_bind_group_layout,
        );

        let chunk_renderer = ChunkRender {
            shader_module,
            chunk_data_list,
            render_pipeline,
        };

        chunk_renderer
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
                buffers: &[VertexData::desc()],
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

    pub fn create_render_pipeline(
        device: &Device,
        surface_format: &TextureFormat,
        shader_module: &wgpu::ShaderModule,
        fog_uniform_bind_group_layout: &BindGroupLayout,
        camera_uniform_bind_group_layout: &BindGroupLayout,
        texture_sampler_bind_group_layout: &BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Chunk Pipeline Layout"),
                bind_group_layouts: &[
                    &fog_uniform_bind_group_layout,
                    &camera_uniform_bind_group_layout,
                    &texture_sampler_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = Self::create_chunk_render_pipeline(
            &device,
            &render_pipeline_layout,
            &shader_module,
            surface_format,
        );

        render_pipeline
    }

    pub fn render(
        &mut self,
        encoder: &mut CommandEncoder,
        texture_view: &TextureView,
        depth_texture_view: &TextureView,
        fog_bind_group: &wgpu::BindGroup,
        camera_bind_group: &wgpu::BindGroup,
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
            depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, fog_bind_group, &[]);
        render_pass.set_bind_group(1, camera_bind_group, &[]);
        render_pass.set_bind_group(2, texture_sampler_bind_group, &[]);

        for gpu_chunk in self.chunk_data_list.iter() {
            if gpu_chunk.mesh_data.index_count > 0 {
                render_pass.set_vertex_buffer(0, gpu_chunk.mesh_data.vertex_buffer.slice(..));

                render_pass.set_index_buffer(
                    gpu_chunk.mesh_data.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                render_pass.draw_indexed(0..gpu_chunk.mesh_data.index_count, 0, 0..1);
            }
        }

        drop(render_pass);
    }
}
