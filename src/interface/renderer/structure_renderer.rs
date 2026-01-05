pub mod structure_instance_data;
pub mod structure_mesh;
pub mod structure_vertex;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        constants::TEXTURE_ATLAS_MAX,
        gpu::{gpu_context::GPUContext, gpu_mesh::GpuMesh},
        renderer::{
            render_context::RenderContext,
            structure_renderer::{
                structure_instance_data::StructureInstanceData, structure_mesh::StructureMesh,
                structure_vertex::StructureVertex,
            },
        },
    },
    simulation::{
        state::world::grid::{self, Direction},
        supervisor::viewer::view::SectorView,
    },
};
use obj::{load_obj, TexturedVertex};
use std::{collections::HashMap, fs::File, io::BufReader, num::NonZeroU32, sync::Arc};
use tracing::instrument;

pub struct StructureRenderer {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub structure_gpu_mesh_map: HashMap<String, Arc<GpuMesh>>,
    pub structure_instance_buffer: wgpu::Buffer,
    pub structure_instance_data_group_vec: Vec<(String, Vec<StructureInstanceData>)>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl StructureRenderer {
    pub fn new(gpu_context: &GPUContext, render_context: &RenderContext, camera: &Camera) -> Self {
        let structure_gpu_mesh_map = Self::load_structure_gpu_mesh_map(&gpu_context.device);

        let structure_instance_buffer = gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Person Instance Buffer"),
            size: std::mem::size_of::<StructureInstanceData>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let structure_instance_data_group_vec = Vec::new();

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
                    label: Some("Structure Renderer Bind Group Layout"),
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
                label: Some("Structure Renderer Texture Bind Group"),
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

        Self {
            bind_group,
            bind_group_layout,
            structure_gpu_mesh_map,
            structure_instance_buffer,
            structure_instance_data_group_vec,
            render_pipeline,
        }
    }

    fn load_structure_gpu_mesh_map(device: &wgpu::Device) -> HashMap<String, Arc<GpuMesh>> {
        let mut structure_gpu_mesh_map = HashMap::new();

        let structure_models_path = std::path::Path::new("assets/models/population");

        let mut structure_models_directory_iterator = std::fs::read_dir(structure_models_path)
            .expect("Failed to read Population models directory");

        while let Some(Ok(structure_model_entry)) = structure_models_directory_iterator.next() {
            let path = structure_model_entry.path();

            if path.extension().and_then(|extension| extension.to_str()) == Some("obj") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                if let Ok(model_file) = File::open(&path) {
                    let model_file_reader = BufReader::new(model_file);

                    match load_obj(model_file_reader) {
                        Ok(model) => {
                            let structure_mesh = StructureMesh {
                                vertex_vec: model
                                    .vertices
                                    .iter()
                                    .map(|vertex: &TexturedVertex| StructureVertex {
                                        position: vertex.position,
                                        normal: vertex.normal,
                                        uv: [vertex.texture[0], 1.0 - vertex.texture[1]],
                                    })
                                    .collect(),
                                index_vec: model.indices,
                            };

                            let structure_gpu_mesh_arc =
                                Arc::new(StructureMesh::to_gpu_mesh(&structure_mesh, device));

                            structure_gpu_mesh_map
                                .insert(file_stem.to_string(), structure_gpu_mesh_arc);

                            tracing::info!("{}.obj loaded", file_stem);
                        }
                        Err(err) => {
                            tracing::error!("{:?}", err);
                        }
                    }
                }
            }
        }

        structure_gpu_mesh_map
    }

    fn create_render_pipeline(
        gpu_context: &GPUContext,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        structure_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let vert_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Population Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/population.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Population Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/population.frag.wgsl").into(),
                    ),
                });

        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Population Render Pipeline Layout"),
                    bind_group_layouts: &[
                        camera_bind_group_layout,
                        structure_texture_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        gpu_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Population Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vert_shader_module,
                    entry_point: Some("main"),
                    buffers: &[StructureVertex::desc(), StructureInstanceData::desc()],
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
    pub fn apply_sector_view(
        gpu_context: &GPUContext,
        sector_view: &SectorView,
        structure_renderer: &mut StructureRenderer,
    ) {
        structure_renderer.structure_instance_data_group_vec.clear();

        let mut instance_group_map: HashMap<String, Vec<StructureInstanceData>> = HashMap::new();

        for structure_view in &sector_view.structure_view_vec {
            let world_position =
                *(grid::grid_position_to_world_position(structure_view.grid_position).as_array());
            let rotation_xy = Direction::to_rotation(&structure_view.direction);

            let structure_instance_data = StructureInstanceData::new(world_position, rotation_xy);

            let structure_model_name = structure_view.structure_kind.to_string();

            instance_group_map
                .entry(structure_model_name)
                .or_default()
                .push(structure_instance_data);
        }

        structure_renderer
            .structure_instance_data_group_vec
            .extend(instance_group_map.into_iter());

        let total_instance_count: usize = structure_renderer
            .structure_instance_data_group_vec
            .iter()
            .map(|(_, v)| v.len())
            .sum();

        let byte_count_required = (total_instance_count
            * std::mem::size_of::<StructureInstanceData>())
            as wgpu::BufferAddress;

        let byte_count_current = structure_renderer.structure_instance_buffer.size();

        if byte_count_required > byte_count_current {
            let byte_count_updated = byte_count_required
                .next_power_of_two()
                .max(byte_count_current);

            structure_renderer.structure_instance_buffer =
                gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Structure Instance Buffer"),
                    size: byte_count_updated,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

            tracing::info!(
                "Resized structure instance buffer: {} -> {} bytes",
                byte_count_current,
                byte_count_updated
            );
        }
    }

    #[instrument(skip_all)]
    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        gpu_context: &GPUContext,
        camera_uniform_bind_group: &wgpu::BindGroup,
        structure_renderer: &Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let render_pass_color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: surface_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        });

        let depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
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

        render_pass.set_pipeline(&structure_renderer.render_pipeline);
        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);

        let mut offset_bytes = 0;

        for (structure_model_name, structure_instance_data_vec) in
            &structure_renderer.structure_instance_data_group_vec
        {
            let byte_len = (structure_instance_data_vec.len()
                * std::mem::size_of::<StructureInstanceData>())
                as wgpu::BufferAddress;

            gpu_context.queue.write_buffer(
                &structure_renderer.structure_instance_buffer,
                offset_bytes,
                bytemuck::cast_slice(&structure_instance_data_vec),
            );

            let structure_gpu_mesh_arc = Arc::clone(
                structure_renderer
                    .structure_gpu_mesh_map
                    .get(structure_model_name)
                    .unwrap(),
            );

            render_pass.set_vertex_buffer(
                1,
                structure_renderer
                    .structure_instance_buffer
                    .slice(offset_bytes..offset_bytes + byte_len),
            );

            render_pass.set_bind_group(1, &structure_renderer.bind_group, &[]);

            render_pass.set_vertex_buffer(0, structure_gpu_mesh_arc.vertex_buffer.slice(..));

            render_pass.set_index_buffer(
                structure_gpu_mesh_arc.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            let instance_count = structure_instance_data_vec.len() as u32;
            render_pass.draw_indexed(0..structure_gpu_mesh_arc.index_count, 0, 0..instance_count);

            offset_bytes += byte_len;
        }

        drop(render_pass);
    }
}
