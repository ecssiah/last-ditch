pub mod block_render_info;
pub mod sector_render_data;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        constants::WINDOW_CLEAR_COLOR,
        gpu::{gpu_context::GPUContext, gpu_mesh::GpuMesh, gpu_texture_data::GpuTextureData},
        mesh::{mesh_optimizer, sector_mesh::SectorMesh},
        vertex_data::VertexData,
        world_render::block_render_info::BlockRenderInfo,
    },
    simulation::{
        observation::view::{SectorView, WorldView},
        state::world::{block, grid::Grid, sector},
    },
};
use std::collections::{hash_map::Entry, HashMap, HashSet};
use tracing::info_span;

pub struct WorldRender {
    pub block_render_info: BlockRenderInfo,
    pub block_tile_coordinates_map: HashMap<block::Kind, [[u32; 2]; 6]>,
    pub tile_atlas_texture_bind_group: wgpu::BindGroup,
    pub sector_mesh_cache: HashMap<sector::ID, SectorMesh>,
    pub gpu_mesh_cache: HashMap<sector::ID, GpuMesh>,
    pub active_sector_id_set: HashSet<sector::ID>,
    pub active_gpu_mesh_vec: Vec<sector::ID>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl WorldRender {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let block_render_info = BlockRenderInfo::new(64, 2048, 2048);
        let block_tile_coordinates_map = BlockRenderInfo::get_tile_coordinates_map();

        let texture_bind_group_layout = Self::create_texture_bind_group_layout(&gpu_context.device);

        let tile_atlas_texture_data = pollster::block_on(Self::load_texture_data(
            &gpu_context.device,
            &gpu_context.queue,
            "assets/textures/cell/tile_atlas.png",
            "tile_atlas",
        ));

        let tile_atlas_texture_bind_group =
            Self::create_texture_bind_group(&gpu_context.device, &tile_atlas_texture_data);

        let render_pipeline = Self::create_render_pipeline(
            gpu_context,
            &camera.uniform_bind_group_layout,
            &texture_bind_group_layout,
        );

        let sector_mesh_cache = HashMap::new();
        let gpu_mesh_cache = HashMap::new();

        let active_sector_id_set = HashSet::new();
        let active_gpu_mesh_vec = Vec::new();

        Self {
            block_render_info,
            block_tile_coordinates_map,
            tile_atlas_texture_bind_group,
            sector_mesh_cache,
            gpu_mesh_cache,
            active_sector_id_set,
            active_gpu_mesh_vec,
            render_pipeline,
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
        gpu_texture_data: &GpuTextureData,
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
                    resource: wgpu::BindingResource::TextureView(&gpu_texture_data.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&gpu_texture_data.sampler),
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
                    bind_group_layouts: &[camera_bind_group_layout, texture_bind_group_layout],
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

    pub fn apply_world_view(
        device: &wgpu::Device,
        camera: &Camera,
        world_view: &WorldView,
        block_tile_coordinates_map: &HashMap<block::Kind, [[u32; 2]; 6]>,
        sector_mesh_cache: &mut HashMap<sector::ID, SectorMesh>,
        gpu_mesh_cache: &mut HashMap<sector::ID, GpuMesh>,
        active_sector_id_set: &mut HashSet<sector::ID>,
        active_gpu_mesh_vec: &mut Vec<sector::ID>,
    ) {
        let _span = info_span!("apply_world_view").entered();

        active_sector_id_set.clear();
        active_gpu_mesh_vec.clear();

        for (sector_id, sector_view) in &world_view.sector_view_map {
            let _sector_span =
                info_span!("sector", id = usize::from(sector_view.sector_id)).entered();

            if !camera
                .frustum
                .sphere_in_frustum(sector_view.world_position, sector_view.radius)
            {
                continue;
            }

            let sector_mesh = Self::get_or_build_sector_mesh(
                sector_view,
                block_tile_coordinates_map,
                &world_view.grid,
                sector_mesh_cache,
            );

            Self::get_or_build_gpu_sector_mesh(sector_mesh, device, gpu_mesh_cache);

            active_sector_id_set.insert(*sector_id);
            active_gpu_mesh_vec.push(*sector_id);
        }

        sector_mesh_cache.retain(|id, _| active_sector_id_set.contains(id));
        gpu_mesh_cache.retain(|id, _| active_gpu_mesh_vec.contains(id));
    }

    fn get_or_build_sector_mesh<'a>(
        sector_view: &SectorView,
        block_tile_coordinates_map: &HashMap<block::Kind, [[u32; 2]; 6]>,
        grid: &Grid,
        sector_mesh_cache: &'a mut HashMap<sector::ID, SectorMesh>,
    ) -> &'a SectorMesh {
        match sector_mesh_cache.entry(sector_view.sector_id) {
            Entry::Vacant(vacant_entry) => {
                let sector_mesh = mesh_optimizer::lysenko_optimization(
                    sector_view,
                    block_tile_coordinates_map,
                    grid,
                );

                vacant_entry.insert(sector_mesh)
            }
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get().version != sector_view.version {
                    let sector_mesh = mesh_optimizer::lysenko_optimization(
                        sector_view,
                        block_tile_coordinates_map,
                        grid,
                    );
                    *occupied_entry.get_mut() = sector_mesh;
                }
                occupied_entry.into_mut()
            }
        }
    }

    fn get_or_build_gpu_sector_mesh<'a>(
        sector_mesh: &SectorMesh,
        device: &wgpu::Device,
        gpu_mesh_cache: &'a mut HashMap<sector::ID, GpuMesh>,
    ) -> &'a GpuMesh {
        match gpu_mesh_cache.entry(sector_mesh.sector_id) {
            Entry::Vacant(vacant_entry) => {
                let gpu_mesh = SectorMesh::to_gpu_mesh(sector_mesh, device);

                vacant_entry.insert(gpu_mesh)
            }
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get().version != sector_mesh.version {
                    let gpu_mesh = SectorMesh::to_gpu_mesh(sector_mesh, device);

                    *occupied_entry.get_mut() = gpu_mesh;
                }

                occupied_entry.into_mut()
            }
        }
    }

    pub async fn load_texture_data(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        label: &str,
    ) -> GpuTextureData {
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

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

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

        GpuTextureData {
            texture,
            texture_view,
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

        for sector_id in &world_render.active_gpu_mesh_vec {
            let gpu_mesh = &world_render.gpu_mesh_cache[sector_id];

            render_pass.set_bind_group(1, &world_render.tile_atlas_texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(gpu_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..gpu_mesh.index_count, 0, 0..1);
        }

        drop(render_pass);

        // for sector_render_data in &world_render.sector_render_data_vec {
        //     render_pass.set_bind_group(1, &world_render.tile_atlas_texture_bind_group, &[]);

        //     render_pass.set_vertex_buffer(0, sector_render_data.mesh_data.vertex_buffer.slice(..));

        //     render_pass.set_index_buffer(
        //         sector_render_data.mesh_data.index_buffer.slice(..),
        //         wgpu::IndexFormat::Uint32,
        //     );

        //     render_pass.draw_indexed(0..sector_render_data.mesh_data.index_count, 0, 0..1);
        // }
    }
}
