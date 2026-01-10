pub mod sector_face;
pub mod sector_model;
pub mod sector_vertex;

use crate::{
    include_assets,
    interface::{
        asset_manager::AssetManager,
        camera::Camera,
        gpu::{gpu_context::GPUContext, gpu_mesh::GpuMesh},
        renderer::{
            render_mode::RenderMode,
            sector_renderer::{sector_model::SectorModel, sector_vertex::SectorVertexData},
        },
    },
    simulation::{
        state::world::sector::sector_index::SectorIndex,
        supervisor::viewer::view::{SectorView, WorldView},
    },
};
use std::collections::{hash_map::Entry, HashMap, HashSet};
use tracing::instrument;

pub struct SectorRenderer {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: Option<wgpu::BindGroup>,
    pub sector_mesh_cache: HashMap<SectorIndex, SectorModel>,
    pub gpu_mesh_cache: HashMap<SectorIndex, GpuMesh>,
    pub active_sector_index_set: HashSet<SectorIndex>,
    pub active_gpu_mesh_vec: Vec<SectorIndex>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl SectorRenderer {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
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

        let sector_mesh_cache = HashMap::new();
        let gpu_mesh_cache = HashMap::new();

        let active_sector_index_set = HashSet::new();
        let active_gpu_mesh_vec = Vec::new();

        Self {
            bind_group_layout,
            bind_group,
            sector_mesh_cache,
            gpu_mesh_cache,
            active_sector_index_set,
            active_gpu_mesh_vec,
            render_pipeline,
        }
    }

    pub fn setup_bind_group(
        gpu_context: &GPUContext,
        asset_manager: &AssetManager,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        let gpu_texture_data = &asset_manager
            .texture_atlas
            .as_ref()
            .expect("texture atlas must be uploaded before bind group creation")
            .gpu_texture_data;

        gpu_context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("World Renderer Texture Bind Group"),
                layout: bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &gpu_texture_data.texture_view,
                        ),
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
                    buffers: &[SectorVertexData::desc()],
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
        asset_manager: &AssetManager,
        world_view: &WorldView,
        world_renderer: &mut Self,
    ) {
        world_renderer.active_sector_index_set.clear();
        world_renderer.active_gpu_mesh_vec.clear();

        for (sector_index, sector_view) in &world_view.sector_view_map {
            let sector_model = Self::get_or_build_sector_model(
                sector_view,
                asset_manager,
                &mut world_renderer.sector_mesh_cache,
            );

            if sector_model.vertex_vec.is_empty() {
                continue;
            }

            Self::get_or_build_gpu_sector_mesh(
                sector_model,
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
    fn get_or_build_sector_model<'a>(
        sector_view: &SectorView,
        asset_manager: &AssetManager,
        sector_mesh_cache: &'a mut HashMap<SectorIndex, SectorModel>,
    ) -> &'a SectorModel {
        match sector_mesh_cache.entry(sector_view.sector_index) {
            Entry::Vacant(vacant_entry) => {
                let sector_model = SectorModel::from_sector_view(sector_view, asset_manager);

                vacant_entry.insert(sector_model)
            }
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get().version != sector_view.version {
                    let sector_model = SectorModel::from_sector_view(sector_view, asset_manager);

                    *occupied_entry.get_mut() = sector_model;
                }

                occupied_entry.into_mut()
            }
        }
    }

    #[instrument(skip_all)]
    fn get_or_build_gpu_sector_mesh<'a>(
        sector_model: &SectorModel,
        device: &wgpu::Device,
        gpu_mesh_cache: &'a mut HashMap<SectorIndex, GpuMesh>,
    ) -> &'a GpuMesh {
        match gpu_mesh_cache.entry(sector_model.sector_index) {
            Entry::Vacant(vacant_entry) => {
                let gpu_mesh = SectorModel::to_gpu_mesh(sector_model, device);

                vacant_entry.insert(gpu_mesh)
            }
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get().version != sector_model.version {
                    let gpu_mesh = SectorModel::to_gpu_mesh(sector_model, device);

                    *occupied_entry.get_mut() = gpu_mesh;
                }

                occupied_entry.into_mut()
            }
        }
    }

    #[instrument(skip_all)]
    pub fn render(
        render_mode: &RenderMode,
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        asset_manager: &AssetManager,
        sector_renderer: &Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if sector_renderer.bind_group.is_none() {
            return;
        }

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: surface_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: RenderMode::get_load_op(render_mode),
                store: wgpu::StoreOp::Store,
            },
        };

        let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
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

        let bind_group = sector_renderer
            .bind_group
            .as_ref()
            .expect("population bind group must exist at this point");

        render_pass.set_pipeline(&sector_renderer.render_pipeline);

        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);
        render_pass.set_bind_group(1, bind_group, &[]);

        for sector_index in &sector_renderer.active_gpu_mesh_vec {
            let gpu_mesh = &sector_renderer.gpu_mesh_cache[sector_index];

            render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));

            render_pass
                .set_index_buffer(gpu_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..gpu_mesh.index_count, 0, 0..1);
        }

        drop(render_pass);
    }
}
