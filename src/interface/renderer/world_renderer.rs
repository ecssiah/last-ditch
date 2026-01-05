pub mod block_model;
pub mod block_quad;
pub mod sector_face;
pub mod sector_mesh;
pub mod sector_vertex;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        constants::{TEXTURE_ATLAS_MAX, WINDOW_CLEAR_COLOR},
        gpu::{gpu_context::GPUContext, gpu_mesh::GpuMesh},
        renderer::{
            render_context::RenderContext,
            world_renderer::{sector_mesh::SectorMesh, sector_vertex::SectorVertex},
        },
    },
    simulation::{
        constants::SECTOR_RADIUS_IN_METERS,
        supervisor::viewer::view::{SectorView, WorldView},
    },
};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    num::NonZeroU32,
};
use tracing::instrument;

pub struct WorldRenderer {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub sector_mesh_cache: HashMap<usize, SectorMesh>,
    pub gpu_mesh_cache: HashMap<usize, GpuMesh>,
    pub active_sector_index_set: HashSet<usize>,
    pub active_gpu_mesh_vec: Vec<usize>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl WorldRenderer {
    pub fn new(gpu_context: &GPUContext, render_context: &RenderContext, camera: &Camera) -> Self {
        let texture_view_vec: Vec<&wgpu::TextureView> = render_context
            .texture_manager
            .texture_atlas_set
            .gpu_texture_data_vec
            .iter()
            .map(|gpu_texture_data| &gpu_texture_data.texture_view)
            .collect();

        let bind_group_layout =
            gpu_context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("World Renderer Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2Array,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: Some(NonZeroU32::new(TEXTURE_ATLAS_MAX).unwrap()),
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let bind_group = gpu_context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("World Renderer Texture Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureViewArray(&texture_view_vec),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &render_context
                                .texture_manager
                                .texture_atlas_set
                                .gpu_texture_data_vec[0]
                                .sampler,
                        ),
                    },
                ],
            });

        let render_pipeline = Self::create_render_pipeline(
            gpu_context,
            &camera.uniform_bind_group_layout,
            &bind_group_layout,
        );

        let sector_mesh_cache = HashMap::new();
        let gpu_mesh_cache = HashMap::new();

        let active_sector_index_set = HashSet::new();
        let active_gpu_mesh_vec = Vec::new();

        Self {
            bind_group,
            bind_group_layout,
            sector_mesh_cache,
            gpu_mesh_cache,
            active_sector_index_set,
            active_gpu_mesh_vec,
            render_pipeline,
        }
    }

    fn create_render_pipeline(
        gpu_context: &GPUContext,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        tile_bind_group_layout: &wgpu::BindGroupLayout,
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
                    bind_group_layouts: &[camera_bind_group_layout, tile_bind_group_layout],
                    push_constant_ranges: &[],
                });

        gpu_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("World Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vert_shader_module,
                    entry_point: Some("main"),
                    buffers: &[SectorVertex::desc()],
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

    #[instrument(skip_all)]
    pub fn apply_world_view(
        gpu_context: &GPUContext,
        render_context: &RenderContext,
        camera: &Camera,
        world_view: &WorldView,
        world_renderer: &mut Self,
    ) {
        world_renderer.active_sector_index_set.clear();
        world_renderer.active_gpu_mesh_vec.clear();

        // TODO: Block modification causes edge of sector to be invalid mesh
        for (sector_index, sector_view) in &world_view.sector_view_map {
            if !camera
                .frustum
                .sphere_in_frustum(sector_view.world_position, SECTOR_RADIUS_IN_METERS * 1.5)
            {
                continue;
            }

            let sector_mesh = Self::get_or_build_sector_mesh(
                sector_view,
                render_context,
                &mut world_renderer.sector_mesh_cache,
            );

            if sector_mesh.vertex_vec.is_empty() {
                continue;
            }

            Self::get_or_build_gpu_sector_mesh(
                sector_mesh,
                &gpu_context.device,
                &mut world_renderer.gpu_mesh_cache,
            );

            world_renderer.active_sector_index_set.insert(*sector_index);
            world_renderer.active_gpu_mesh_vec.push(*sector_index);
        }

        world_renderer.sector_mesh_cache.retain(|sector_index, _| {
            world_renderer
                .active_sector_index_set
                .contains(sector_index)
        });

        world_renderer
            .gpu_mesh_cache
            .retain(|sector_index, _| world_renderer.active_gpu_mesh_vec.contains(sector_index));

        world_renderer.active_gpu_mesh_vec.sort_unstable();
    }

    #[instrument(skip_all)]
    fn get_or_build_sector_mesh<'a>(
        sector_view: &SectorView,
        render_context: &RenderContext,
        sector_mesh_cache: &'a mut HashMap<usize, SectorMesh>,
    ) -> &'a SectorMesh {
        match sector_mesh_cache.entry(sector_view.sector_index) {
            Entry::Vacant(vacant_entry) => {
                let sector_mesh = SectorMesh::from_sector_view(sector_view, render_context);

                vacant_entry.insert(sector_mesh)
            }
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get().version != sector_view.version {
                    let sector_mesh = SectorMesh::from_sector_view(sector_view, render_context);

                    *occupied_entry.get_mut() = sector_mesh;
                }

                occupied_entry.into_mut()
            }
        }
    }

    #[instrument(skip_all)]
    fn get_or_build_gpu_sector_mesh<'a>(
        sector_mesh: &SectorMesh,
        device: &wgpu::Device,
        gpu_mesh_cache: &'a mut HashMap<usize, GpuMesh>,
    ) -> &'a GpuMesh {
        match gpu_mesh_cache.entry(sector_mesh.sector_index) {
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

    #[instrument(skip_all)]
    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        world_renderer: &Self,
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

        render_pass.set_pipeline(&world_renderer.render_pipeline);
        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &world_renderer.bind_group, &[]);

        for sector_index in &world_renderer.active_gpu_mesh_vec {
            let gpu_mesh = &world_renderer.gpu_mesh_cache[sector_index];

            render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));

            render_pass
                .set_index_buffer(gpu_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..gpu_mesh.index_count, 0, 0..1);
        }

        drop(render_pass);
    }
}
