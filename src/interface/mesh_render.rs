pub mod block_render_info;
pub mod data;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        consts::*,
        gpu_context::GPUContext,
        mesh_render::{
            block_render_info::BlockRenderInfo,
            data::{MeshData, RenderData, TextureData, VertexData},
        },
    },
    simulation::{self, observation::view::PopulationView, state::population::entity},
};
use glam::{IVec2, Mat4};
use obj::load_obj;
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(PartialEq, Eq, Hash)]
pub enum RenderType {
    Block,
    Entity,
    Item,
}

pub struct MeshRender {
    pub mesh_data_map: HashMap<entity::Kind, MeshData>,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_map: HashMap<String, wgpu::BindGroup>,
    pub block_render_info_map: HashMap<simulation::state::world::block::Kind, BlockRenderInfo>,
    pub render_data_map: HashMap<RenderType, Vec<RenderData>>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl MeshRender {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let block_render_info_map = BlockRenderInfo::setup();

        let mesh_data_map = Self::load_mesh_data_map(&gpu_context.device);
        let texture_bind_group_layout = Self::create_texture_bind_group_layout(&gpu_context.device);
        let texture_bind_group_map =
            Self::load_texture_bind_group_map(&gpu_context.device, &gpu_context.queue);

        let render_pipeline = Self::create_render_pipeline(
            gpu_context,
            &camera.uniform_bind_group_layout,
            &texture_bind_group_layout,
        );

        let render_data_map = HashMap::from([
            (RenderType::Block, Vec::new()),
            (RenderType::Entity, Vec::new()),
            (RenderType::Item, Vec::new()),
        ]);

        Self {
            block_render_info_map,
            mesh_data_map,
            texture_bind_group_layout,
            texture_bind_group_map,
            render_pipeline,
            render_data_map,
        }
    }

    fn load_mesh_data_map(device: &wgpu::Device) -> HashMap<entity::Kind, MeshData> {
        let mut mesh_data_map = HashMap::new();

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
                            let vertex_vec = model.vertices;
                            let index_vec = model.indices;

                            let mesh_data = MeshData::new(device, vertex_vec, index_vec);

                            if let Some(kind) = entity::Kind::from_string(file_stem) {
                                log::info!("{:?} model loaded", file_stem);
                                mesh_data_map.insert(kind, mesh_data);
                            }
                        }
                        Err(err) => {
                            log::error!("{:?}", err);
                        }
                    }
                }
            }
        }

        mesh_data_map
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

    fn load_texture_bind_group_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> HashMap<String, wgpu::BindGroup> {
        let mut texture_bind_group_map = HashMap::new();

        let tile_atlas_texture_data = pollster::block_on(Self::load_texture_data(
            device,
            queue,
            "assets/textures/block/tile_atlas.png",
            "block",
        ));

        let tile_atlas_texture_bind_group_key = "block".to_string();

        let tile_atlas_texture_bind_group =
            Self::create_texture_bind_group(device, &tile_atlas_texture_data);

        texture_bind_group_map.insert(
            tile_atlas_texture_bind_group_key,
            tile_atlas_texture_bind_group,
        );

        let entity_textures_path = std::path::Path::new("assets/textures/entity");

        let mut entity_texture_entries_itr = std::fs::read_dir(entity_textures_path)
            .expect("Failed to read entity models directory");

        while let Some(Ok(entity_texture_entry)) = entity_texture_entries_itr.next() {
            let path = entity_texture_entry.path();

            if path.extension().and_then(|extension| extension.to_str()) == Some("png") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                let texture_data = pollster::block_on(Self::load_texture_data(
                    device,
                    queue,
                    path.to_str().unwrap(),
                    file_stem,
                ));

                let texture_bind_group_key = file_stem.to_string();
                let texture_bind_group = Self::create_texture_bind_group(device, &texture_data);

                texture_bind_group_map.insert(texture_bind_group_key, texture_bind_group);
            }
        }

        texture_bind_group_map
    }

    pub fn render(
        gpu_context: &GPUContext,
        surface_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        render_pipeline: &wgpu::RenderPipeline,
        render_data_map: &HashMap<RenderType, Vec<RenderData>>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let depth_texture_view =
            MeshRender::create_depth_texture(&gpu_context.device, &gpu_context.surface_config);

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
            view: &depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Mesh Render Pass"),
            color_attachments: &[render_pass_color_attachment],
            depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);

        for render_data_vec in render_data_map.values() {
            for render_data in render_data_vec {
                render_pass.set_bind_group(1, &render_data.texture_bind_group, &[]);

                render_pass.set_vertex_buffer(0, render_data.mesh_data.vertex_buffer.slice(..));

                render_pass.set_index_buffer(
                    render_data.mesh_data.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                render_pass.draw_indexed(0..render_data.mesh_data.index_count, 0, 0..1);
            }
        }

        drop(render_pass);
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

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
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
            label: Some("texture_atlas_sampler"),
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

    fn create_render_pipeline(
        gpu_context: &GPUContext,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let vert_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Vert Shader Module"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/mesh.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Frag Shader Module"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/mesh.frag.wgsl").into(),
                    ),
                });

        gpu_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Primary Render Pipeline"),
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

    pub fn apply_population_view(
        population_view: &PopulationView,
        mesh_data_map: &HashMap<entity::Kind, MeshData>,
        texture_bind_group_map: &HashMap<String, wgpu::BindGroup>,
        render_data_map: &mut HashMap<RenderType, Vec<RenderData>>,
    ) {
        let entity_render_data_vec = render_data_map.get_mut(&RenderType::Entity).unwrap();

        entity_render_data_vec.clear();

        for agent_view in population_view.agent_view_map.values() {
            let mesh_data = mesh_data_map.get(&agent_view.kind).unwrap().clone();

            let transform = Mat4::from_translation(agent_view.spatial.world_position);

            let texture_bind_group = texture_bind_group_map
                .get(&format!("agent_{:?}", agent_view.kind).to_lowercase())
                .unwrap()
                .clone();

            let render_data = RenderData {
                mesh_data,
                transform,
                texture_bind_group,
            };

            entity_render_data_vec.push(render_data);
        }
    }

    pub fn apply_world_view(
        device: &wgpu::Device,
        world_view: &simulation::observation::view::WorldView,
        block_render_info_map: &HashMap<simulation::state::world::block::Kind, BlockRenderInfo>,
        texture_bind_group_map: &HashMap<String, wgpu::BindGroup>,
        render_data_map: &mut HashMap<RenderType, Vec<RenderData>>,
    ) {
        let block_render_data_vec = render_data_map.get_mut(&RenderType::Block).unwrap();

        let block_texture_bind_group = &texture_bind_group_map["block"];

        block_render_data_vec.clear();

        for chunk_view in world_view.chunk_view_map.values() {
            let mut vertex_vec = Vec::new();
            let mut index_vec = Vec::new();
            let mut index_offset = 0;

            for block in &chunk_view.block_vec {
                if block.kind == simulation::state::world::block::Kind::Empty {
                    continue;
                }

                let block_render_info = block_render_info_map.get(&block.kind).unwrap();

                for face in &block.face_array {
                    let face_vertex_array =
                        BlockRenderInfo::face_vertices(block.position, face.direction);

                    let tile_face_index = BlockRenderInfo::face_direction_to_index(face.direction);

                    let tile_coordinates = IVec2::new(
                        block_render_info.tile_index_array[tile_face_index][0] as i32,
                        block_render_info.tile_index_array[tile_face_index][1] as i32,
                    );

                    let tile_size = 64;
                    let tile_atlas_size = IVec2::new(32, 32);

                    let tile_uv_coordinates = BlockRenderInfo::tile_uv_coordinates(
                        tile_coordinates,
                        tile_size,
                        tile_atlas_size,
                    );

                    for (index, &position) in face_vertex_array.iter().enumerate() {
                        let normal = face.direction.offset().as_vec3().to_array();
                        let texture = [
                            tile_uv_coordinates[index][0],
                            tile_uv_coordinates[index][1],
                            0.0,
                        ];

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
                let render_data = RenderData {
                    mesh_data: MeshData::new(device, vertex_vec, index_vec),
                    transform: Mat4::IDENTITY,
                    texture_bind_group: block_texture_bind_group.clone(),
                };

                block_render_data_vec.push(render_data);
            }
        }
    }
}
