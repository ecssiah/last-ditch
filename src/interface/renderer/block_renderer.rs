pub mod block_instance_data;
pub mod block_model;
pub mod block_vertex_data;

use std::collections::HashMap;

use crate::{
    include_assets,
    interface::{
        asset_manager::{
            block_model_key::BlockModelKey, block_texture_key::BlockTextureKey, AssetManager,
        },
        camera::Camera,
        gpu::gpu_context::GPUContext,
        renderer::block_renderer::{
            block_instance_data::BlockInstanceData, block_vertex_data::BlockVertexData,
        },
    },
    simulation::{
        state::world::grid::{self, Direction},
        supervisor::viewer::view::WorldView,
    },
};

pub struct BlockRenderer {
    pub render_order: u32,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: Option<wgpu::BindGroup>,
    pub block_instance_buffer: wgpu::Buffer,
    pub block_instance_data_group_vec: Vec<(BlockModelKey, Vec<BlockInstanceData>)>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl BlockRenderer {
    pub fn new(render_order: u32, gpu_context: &GPUContext, camera: &Camera) -> Self {
        let block_instance_buffer = gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Block Instance Buffer"),
            size: std::mem::size_of::<BlockInstanceData>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let block_instance_data_group_vec = Vec::new();

        let bind_group_layout =
            gpu_context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Block Renderer Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2Array,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let bind_group = None;

        let render_pipeline = Self::create_render_pipeline(
            gpu_context,
            &camera.uniform_bind_group_layout,
            &bind_group_layout,
        );

        Self {
            render_order,
            bind_group_layout,
            bind_group,
            block_instance_buffer,
            block_instance_data_group_vec,
            render_pipeline,
        }
    }

    pub fn apply_world_view(
        gpu_context: &GPUContext,
        asset_manager: &AssetManager,
        world_view: &WorldView,
        block_renderer: &mut Self,
    ) {
        block_renderer.block_instance_data_group_vec.clear();

        let mut block_model_instance_map: HashMap<BlockModelKey, Vec<BlockInstanceData>> =
            HashMap::new();

        for sector_view in world_view.sector_view_map.values() {
            for block in sector_view.block_vec.iter().flatten() {
                let world_position =
                    *(grid::grid_position_to_world_position(block.grid_position)).as_array();
                let rotation_xy = Direction::to_rotation(&block.direction);

                let layer_index = AssetManager::get_block_layer_index(
                    &BlockTextureKey::from_block_kind(&block.block_kind),
                    asset_manager,
                );

                let block_instance_data =
                    BlockInstanceData::new(world_position, rotation_xy, layer_index.into());

                let block_model_key = BlockModelKey::from_block_shape(&block.block_shape);

                block_model_instance_map
                    .entry(block_model_key)
                    .or_default()
                    .push(block_instance_data);
            }
        }

        block_renderer
            .block_instance_data_group_vec
            .extend(block_model_instance_map.into_iter());

        let total_instance_count: usize = block_renderer
            .block_instance_data_group_vec
            .iter()
            .map(|(_, v)| v.len())
            .sum();

        let byte_count_required = (total_instance_count * std::mem::size_of::<BlockInstanceData>())
            as wgpu::BufferAddress;

        let byte_count_current = block_renderer.block_instance_buffer.size();

        if byte_count_required > byte_count_current {
            let byte_count_updated = byte_count_required
                .next_power_of_two()
                .max(byte_count_current);

            block_renderer.block_instance_buffer =
                gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Block Instance Buffer"),
                    size: byte_count_updated,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

            tracing::info!(
                "Resized block instance buffer: {} -> {} bytes",
                byte_count_current,
                byte_count_updated
            );
        }
    }

    fn create_render_pipeline(
        gpu_context: &GPUContext,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        block_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let vert_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Block Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/block.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Block Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/block.frag.wgsl").into(),
                    ),
                });

        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Block Render Pipeline Layout"),
                    bind_group_layouts: &[
                        camera_bind_group_layout,
                        block_texture_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        gpu_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Block Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vert_shader_module,
                    entry_point: Some("main"),
                    buffers: &[BlockVertexData::desc(), BlockInstanceData::desc()],
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
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
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

    pub fn render(
        gpu_context: &GPUContext,
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        asset_manager: &AssetManager,
        block_renderer: &Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if block_renderer.bind_group.is_none() {
            return;
        }

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: surface_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        };

        let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: Some(depth_stencil_attachment),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let bind_group = block_renderer
            .bind_group
            .as_ref()
            .expect("population bind group must exist at this point");

        render_pass.set_pipeline(&block_renderer.render_pipeline);

        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);
        render_pass.set_bind_group(1, bind_group, &[]);

        let mut offset_bytes = 0;

        for (block_model_key, block_instance_data_vec) in
            &block_renderer.block_instance_data_group_vec
        {
            let byte_len = (block_instance_data_vec.len()
                * std::mem::size_of::<BlockInstanceData>())
                as wgpu::BufferAddress;

            gpu_context.queue.write_buffer(
                &block_renderer.block_instance_buffer,
                offset_bytes,
                bytemuck::cast_slice(&block_instance_data_vec),
            );

            let block_gpu_mesh =
                AssetManager::get_block_model_gpu_mesh(block_model_key, asset_manager);

            render_pass.set_vertex_buffer(
                1,
                block_renderer
                    .block_instance_buffer
                    .slice(offset_bytes..offset_bytes + byte_len),
            );

            render_pass.set_vertex_buffer(0, block_gpu_mesh.vertex_buffer.slice(..));

            render_pass.set_index_buffer(
                block_gpu_mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            let instance_count = block_instance_data_vec.len() as u32;
            render_pass.draw_indexed(0..block_gpu_mesh.index_count, 0, 0..instance_count);

            offset_bytes += byte_len;
        }

        drop(render_pass);
    }
}
