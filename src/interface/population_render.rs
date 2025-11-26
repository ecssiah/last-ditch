pub mod entity_instance_data;
pub mod entity_mesh;
pub mod entity_vertex;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        gpu::{gpu_context::GPUContext, gpu_mesh::GpuMesh, gpu_texture_data::GpuTextureData},
        population_render::{
            entity_instance_data::EntityInstanceData, entity_mesh::EntityMesh,
            entity_vertex::EntityVertex,
        },
    },
    simulation::{
        constants::SIMULATION_MAX_ENTITIES,
        state::population::{self, nation},
        viewer::view::PopulationView,
    },
};
use obj::{load_obj, TexturedVertex};
use std::{collections::HashMap, fs::File, io::BufReader, ops::Deref, sync::Arc};
use tracing::{error, info};

pub struct PopulationRender {
    pub entity_gpu_mesh_map: HashMap<(population::Role, nation::Kind), Arc<GpuMesh>>,
    pub entity_instance_buffer: wgpu::Buffer,
    pub entity_instance_data_group_vec:
        Vec<((population::Role, nation::Kind), Vec<EntityInstanceData>)>,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_arc_map: HashMap<(population::Role, nation::Kind), Arc<wgpu::BindGroup>>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl PopulationRender {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let entity_gpu_mesh_map = Self::load_entity_gpu_mesh_map(&gpu_context.device);

        let entity_instance_buffer = gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Population Instance Buffer"),
            size: (SIMULATION_MAX_ENTITIES * std::mem::size_of::<EntityInstanceData>())
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let entity_instance_data_group_vec = Vec::new();

        let texture_bind_group_layout = Self::create_texture_bind_group_layout(&gpu_context.device);
        let texture_bind_group_arc_map =
            Self::load_texture_bind_group_arc_map(&gpu_context.device, &gpu_context.queue);

        let render_pipeline = Self::create_render_pipeline(
            gpu_context,
            &camera.uniform_bind_group_layout,
            &texture_bind_group_layout,
        );

        Self {
            entity_gpu_mesh_map,
            entity_instance_buffer,
            entity_instance_data_group_vec,
            texture_bind_group_layout,
            texture_bind_group_arc_map,
            render_pipeline,
        }
    }

    fn load_entity_gpu_mesh_map(
        device: &wgpu::Device,
    ) -> HashMap<(population::Role, nation::Kind), Arc<GpuMesh>> {
        let mut entity_gpu_mesh_map = HashMap::new();

        let entity_models_path = std::path::Path::new("assets/models/entity");

        let mut entity_model_entries =
            std::fs::read_dir(entity_models_path).expect("Failed to read entity models directory");

        while let Some(Ok(entity_model_entry)) = entity_model_entries.next() {
            let path = entity_model_entry.path();

            if path.extension().and_then(|extension| extension.to_str()) == Some("obj") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                if let Ok(model_file) = File::open(&path) {
                    let model_file_reader = BufReader::new(model_file);

                    match load_obj(model_file_reader) {
                        Ok(model) => {
                            let entity_mesh = EntityMesh {
                                vertex_vec: model
                                    .vertices
                                    .iter()
                                    .map(|vertex: &TexturedVertex| EntityVertex {
                                        position: vertex.position,
                                        normal: vertex.normal,
                                        uv: [vertex.texture[0], vertex.texture[1]],
                                    })
                                    .collect(),
                                index_vec: model.indices,
                            };

                            let entity_gpu_mesh_arc =
                                Arc::new(EntityMesh::to_gpu_mesh(&entity_mesh, device));

                            if let Some(role) = population::Role::from_string(file_stem) {
                                if let Some(nation_kind) = nation::Kind::from_string(file_stem) {
                                    info!("{:?} model loaded", file_stem);

                                    entity_gpu_mesh_map
                                        .insert((role, nation_kind), entity_gpu_mesh_arc);
                                }
                            }
                        }
                        Err(err) => {
                            error!("{:?}", err);
                        }
                    }
                }
            }
        }

        entity_gpu_mesh_map
    }

    fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        })
    }

    fn load_texture_bind_group_arc_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> HashMap<(population::Role, nation::Kind), Arc<wgpu::BindGroup>> {
        let mut texture_bind_group_map = HashMap::new();

        let entity_textures_path = std::path::Path::new("assets/textures/entity");

        let mut entity_texture_entries_itr = std::fs::read_dir(entity_textures_path)
            .expect("Failed to read entity models directory");

        while let Some(Ok(entity_texture_entry)) = entity_texture_entries_itr.next() {
            let path = entity_texture_entry.path();

            if path.extension().and_then(|extension| extension.to_str()) == Some("png") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                let gpu_texture_data = pollster::block_on(Self::load_texture_data(
                    device,
                    queue,
                    path.to_str().unwrap(),
                    file_stem,
                ));

                let texture_bind_group =
                    Arc::new(Self::create_texture_bind_group(device, &gpu_texture_data));

                if let Some(role) = population::Role::from_string(file_stem) {
                    if let Some(nation_kind) = nation::Kind::from_string(file_stem) {
                        info!("{:?} texture loaded", file_stem);

                        texture_bind_group_map.insert((role, nation_kind), texture_bind_group);
                    }
                }
            }
        }

        texture_bind_group_map
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

    fn create_render_pipeline(
        gpu_context: &GPUContext,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let vert_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Population Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/entity.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Population Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/entity.frag.wgsl").into(),
                    ),
                });

        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Population Render Pipeline Layout"),
                    bind_group_layouts: &[camera_bind_group_layout, texture_bind_group_layout],
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
                    buffers: &[EntityVertex::desc(), EntityInstanceData::desc()],
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

    pub fn apply_population_view(
        population_view: &PopulationView,
        entity_instance_data_group_vec: &mut Vec<(
            (population::Role, nation::Kind),
            Vec<EntityInstanceData>,
        )>,
    ) {
        entity_instance_data_group_vec.clear();

        let mut group_map: HashMap<(population::Role, nation::Kind), Vec<EntityInstanceData>> =
            HashMap::new();

        for agent_view in population_view.agent_view_map.values() {
            let entity_instance_data = EntityInstanceData {
                world_position: *agent_view.spatial.world_position.as_array(),
                size_y: agent_view.spatial.size.y,
                yaw: agent_view.spatial.yaw,
                _padding: [0.0, 0.0, 0.0],
            };

            group_map
                .entry((agent_view.role, agent_view.nation_kind))
                .or_default()
                .push(entity_instance_data);
        }

        entity_instance_data_group_vec.extend(group_map.into_iter());
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        gpu_context: &GPUContext,
        camera_uniform_bind_group: &wgpu::BindGroup,
        entity_render: &PopulationRender,
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

        render_pass.set_pipeline(&entity_render.render_pipeline);
        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);

        let mut offset_bytes = 0;

        for (kind, entity_instance_data_vec) in &entity_render.entity_instance_data_group_vec {
            let byte_len = (entity_instance_data_vec.len()
                * std::mem::size_of::<EntityInstanceData>())
                as wgpu::BufferAddress;

            gpu_context.queue.write_buffer(
                &entity_render.entity_instance_buffer,
                offset_bytes,
                bytemuck::cast_slice(&entity_instance_data_vec),
            );

            let entity_gpu_mesh_arc =
                Arc::clone(entity_render.entity_gpu_mesh_map.get(&kind).unwrap());

            let texture_bind_group_arc =
                Arc::clone(entity_render.texture_bind_group_arc_map.get(&kind).unwrap());

            render_pass.set_vertex_buffer(
                1,
                entity_render
                    .entity_instance_buffer
                    .slice(offset_bytes..offset_bytes + byte_len),
            );

            render_pass.set_bind_group(1, texture_bind_group_arc.deref(), &[]);

            render_pass.set_vertex_buffer(0, entity_gpu_mesh_arc.vertex_buffer.slice(..));

            render_pass.set_index_buffer(
                entity_gpu_mesh_arc.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            let instance_count = entity_instance_data_vec.len() as u32;
            render_pass.draw_indexed(0..entity_gpu_mesh_arc.index_count, 0, 0..instance_count);

            offset_bytes += byte_len;
        }

        drop(render_pass);
    }
}
