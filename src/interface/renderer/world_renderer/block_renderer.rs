pub mod block_instance_data;
pub mod block_vertex_data;

use crate::{include_assets, interface::{asset_manager::AssetManager, camera::Camera, gpu::gpu_context::GPUContext, renderer::world_renderer::block_renderer::{block_instance_data::BlockInstanceData, block_vertex_data::BlockVertexData}}, simulation::supervisor::viewer::view::WorldView};

pub struct BlockRenderer {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: Option<wgpu::BindGroup>,
    pub block_instance_buffer: wgpu::Buffer,
    pub block_instance_data_group_vec: Vec<(String, Vec<BlockInstanceData>)>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl BlockRenderer {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
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
            bind_group_layout,
            bind_group,
            block_instance_buffer,
            block_instance_data_group_vec,
            render_pipeline,
        }
    }

    pub fn apply_world_view(
        gpu_context: &GPUContext,
        camera: &Camera,
        asset_manager: &AssetManager,
        world_view: &WorldView,
        world_renderer: &mut Self
    ) {
        
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
                    label: Some("Person Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/person.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Person Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/person.frag.wgsl").into(),
                    ),
                });

        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Person Render Pipeline Layout"),
                    bind_group_layouts: &[
                        camera_bind_group_layout,
                        block_texture_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        gpu_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Person Render Pipeline"),
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
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        asset_manager: &AssetManager,
        block_renderer: &Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {

    }
}