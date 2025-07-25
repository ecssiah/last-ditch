pub mod block_render_info;
pub mod chunk_render_data;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        consts::WINDOW_CLEAR_COLOR,
        gpu_context::GPUContext,
        mesh_data::MeshData,
        texture_data::TextureData,
        vertex_data::VertexData,
        world_render::{block_render_info::BlockRenderInfo, chunk_render_data::ChunkRenderData},
    },
    simulation::{
        observation::view::WorldView,
        state::world::{block, grid},
    },
};
use std::collections::HashMap;

pub struct WorldRender {
    pub block_render_info: BlockRenderInfo,
    pub block_tile_coordinates_map: HashMap<block::Kind, HashMap<grid::Direction, [u32; 2]>>,
    pub tile_atlas_texture_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub chunk_render_data_vec: Vec<ChunkRenderData>,
}

impl WorldRender {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let block_render_info = BlockRenderInfo::new(64, 2048, 2048);
        let block_tile_coordinates_map = BlockRenderInfo::setup_tile_coordinates_map();

        let texture_bind_group_layout = Self::create_texture_bind_group_layout(&gpu_context.device);

        let tile_atlas_texture_data = pollster::block_on(Self::load_texture_data(
            &gpu_context.device,
            &gpu_context.queue,
            "assets/textures/block/tile_atlas.png",
            "tile_atlas",
        ));

        let tile_atlas_texture_bind_group =
            Self::create_texture_bind_group(&gpu_context.device, &tile_atlas_texture_data);

        let render_pipeline = Self::create_render_pipeline(
            gpu_context,
            &camera.uniform_bind_group_layout,
            &texture_bind_group_layout,
        );

        let chunk_render_data_vec = Vec::new();

        Self {
            block_render_info,
            block_tile_coordinates_map,
            tile_atlas_texture_bind_group,
            render_pipeline,
            chunk_render_data_vec,
        }
    }

    fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture BindGroupLayout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
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
        })
    }

    pub fn create_texture_bind_group(
        device: &wgpu::Device,
        texture_data: &TextureData,
    ) -> wgpu::BindGroup {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
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

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture and Sampler Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_data.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_data.sampler),
                },
            ],
        })
    }

    fn create_render_pipeline(
        gpu_context: &GPUContext,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let vert_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("World Vert Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/world.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("World Frag Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/world.frag.wgsl").into(),
                    ),
                });

        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("World Render Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                    push_constant_ranges: &[],
                });

        gpu_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Mesh Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vert_shader_module,
                    entry_point: Some("main"),
                    buffers: &[VertexData::desc()],
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

    pub fn apply_world_view(
        device: &wgpu::Device,
        world_view: &WorldView,
        block_render_info: &BlockRenderInfo,
        block_tile_coordinates_map: &HashMap<block::Kind, HashMap<grid::Direction, [u32; 2]>>,
        chunk_render_data_vec: &mut Vec<ChunkRenderData>,
    ) {
        chunk_render_data_vec.clear();

        for chunk_view in world_view.chunk_view_map.values() {
            let mut vertex_vec = Vec::new();
            let mut index_vec = Vec::new();
            let mut index_offset = 0;

            for block in &chunk_view.block_vec {
                if block.kind == block::Kind::Empty {
                    continue;
                }

                let tile_coordinates_map = block_tile_coordinates_map.get(&block.kind).unwrap();

                for face in &block.face_array {
                    let tile_coordinates = tile_coordinates_map.get(&face.direction).unwrap();

                    let tile_uv_array = BlockRenderInfo::tile_uv_array(
                        tile_coordinates,
                        block_render_info.tile_size,
                        block_render_info.tile_atlas_size,
                    );

                    let face_vertex_position_array =
                        BlockRenderInfo::face_vertex_position_array(block.position, face.direction);

                    for (index, &position) in face_vertex_position_array.iter().enumerate() {
                        let normal = face.direction.offset().as_vec3().to_array();

                        let texture = [tile_uv_array[index][0], tile_uv_array[index][1], 0.0];

                        let textured_vertex = obj::TexturedVertex {
                            position,
                            normal,
                            texture,
                        };

                        vertex_vec.push(textured_vertex);
                    }

                    index_vec.push(index_offset);
                    index_vec.push(index_offset + 1);
                    index_vec.push(index_offset + 2);
                    index_vec.push(index_offset);
                    index_vec.push(index_offset + 2);
                    index_vec.push(index_offset + 3);

                    index_offset += 4;
                }
            }

            if vertex_vec.len() > 0 {
                let chunk_render_data = ChunkRenderData {
                    chunk_id: chunk_view.id,
                    mesh_data: MeshData::new(device, vertex_vec, index_vec),
                };

                chunk_render_data_vec.push(chunk_render_data);
            }
        }
    }

    pub async fn load_texture_data(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        label: &str,
    ) -> TextureData {
        let img = image::open(path)
            .expect("Failed to open texture atlas")
            .into_rgba8();

        let (width, height) = img.dimensions();

        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        TextureData {
            texture,
            view,
            sampler,
        }
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        world_render: &WorldRender,
        encoder: &mut wgpu::CommandEncoder,
    ) {        
        let render_pass_color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: surface_texture_view,
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
            view: depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[render_pass_color_attachment],
            depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&world_render.render_pipeline);
        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);

        for chunk_render_data in &world_render.chunk_render_data_vec {
            render_pass.set_bind_group(1, &world_render.tile_atlas_texture_bind_group, &[]);

            render_pass.set_vertex_buffer(0, chunk_render_data.mesh_data.vertex_buffer.slice(..));

            render_pass.set_index_buffer(
                chunk_render_data.mesh_data.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            render_pass.draw_indexed(0..chunk_render_data.mesh_data.index_count, 0, 0..1);
        }

        drop(render_pass);
    }
}
