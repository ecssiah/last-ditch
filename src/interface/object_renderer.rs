pub mod object_instance_data;
pub mod object_mesh;
pub mod object_vertex;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        gpu::{gpu_context::GPUContext, gpu_mesh::GpuMesh, gpu_texture_data::GpuTextureData},
        object_renderer::{
            object_instance_data::ObjectInstanceData, object_mesh::ObjectMesh,
            object_vertex::ObjectVertex,
        },
    },
    simulation::{
        constants::*,
        manager::viewer::view::ObjectView,
        state::world::{grid, object},
    },
};
use obj::{load_obj, TexturedVertex};
use std::{collections::HashMap, fs::File, io::BufReader, ops::Deref, sync::Arc};

pub struct ObjectRenderer {
    pub object_gpu_mesh_map: HashMap<String, Arc<GpuMesh>>,
    pub object_instance_buffer: wgpu::Buffer,
    pub object_instance_data_group_vec: Vec<(String, Vec<ObjectInstanceData>)>,
    pub object_texture_bind_group_layout: wgpu::BindGroupLayout,
    pub object_texture_bind_group_arc_map: HashMap<String, Arc<wgpu::BindGroup>>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl ObjectRenderer {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let object_gpu_mesh_map = Self::load_object_gpu_mesh_map(&gpu_context.device);

        let object_instance_buffer = gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Object Instance Buffer"),
            size: (OBJECT_MAX_COUNT * std::mem::size_of::<ObjectInstanceData>())
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let object_instance_data_group_vec = Vec::new();

        let object_texture_bind_group_layout =
            Self::create_object_texture_bind_group_layout(&gpu_context.device);

        let object_texture_bind_group_arc_map =
            Self::load_object_texture_bind_group_arc_map(&gpu_context.device, &gpu_context.queue);

        let render_pipeline = Self::create_render_pipeline(
            gpu_context,
            &camera.uniform_bind_group_layout,
            &object_texture_bind_group_layout,
        );

        Self {
            object_gpu_mesh_map,
            object_instance_buffer,
            object_instance_data_group_vec,
            object_texture_bind_group_layout,
            object_texture_bind_group_arc_map,
            render_pipeline,
        }
    }

    fn load_object_gpu_mesh_map(device: &wgpu::Device) -> HashMap<String, Arc<GpuMesh>> {
        let mut object_gpu_mesh_map = HashMap::new();

        let object_models_path = std::path::Path::new("assets/models/object");

        let mut object_models_directory_iterator =
            std::fs::read_dir(object_models_path).expect("Failed to read Object models directory");

        while let Some(Ok(object_model_entry)) = object_models_directory_iterator.next() {
            let path = object_model_entry.path();

            if path.extension().and_then(|extension| extension.to_str()) == Some("obj") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                if let Ok(model_file) = File::open(&path) {
                    let model_file_reader = BufReader::new(model_file);

                    match load_obj(model_file_reader) {
                        Ok(model) => {
                            let object_mesh = ObjectMesh {
                                vertex_vec: model
                                    .vertices
                                    .iter()
                                    .map(|vertex: &TexturedVertex| ObjectVertex {
                                        position: vertex.position,
                                        normal: vertex.normal,
                                        uv: [vertex.texture[0], vertex.texture[1]],
                                    })
                                    .collect(),
                                index_vec: model.indices,
                            };

                            let object_gpu_mesh_arc =
                                Arc::new(ObjectMesh::to_gpu_mesh(&object_mesh, device));

                            object_gpu_mesh_map.insert(file_stem.to_string(), object_gpu_mesh_arc);

                            tracing::info!("{}.obj loaded", file_stem);
                        }
                        Err(err) => {
                            tracing::error!("{:?}", err);
                        }
                    }
                }
            }
        }

        object_gpu_mesh_map
    }

    fn create_object_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Object Texture Bind Group Layout"),
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

    fn load_object_texture_bind_group_arc_map(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> HashMap<String, Arc<wgpu::BindGroup>> {
        let mut texture_bind_group_map = HashMap::new();

        let object_textures_path = std::path::Path::new("assets/textures/object");

        let mut object_textures_directory_iterator = std::fs::read_dir(object_textures_path)
            .expect("Failed to read Object textures directory");

        while let Some(Ok(object_texture_entry)) = object_textures_directory_iterator.next() {
            let path = object_texture_entry.path();

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

                texture_bind_group_map.insert(file_stem.to_string(), texture_bind_group);

                tracing::info!("{}.png loaded", file_stem);
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
            label: Some("Object Texture and Sampler Bind Group"),
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
            .expect("Failed to open Person texture data")
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
        object_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let vert_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Object Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/object.vert.wgsl").into(),
                    ),
                });

        let frag_shader_module =
            gpu_context
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Object Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_assets!("shaders/object.frag.wgsl").into(),
                    ),
                });

        let pipeline_layout =
            gpu_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Object Render Pipeline Layout"),
                    bind_group_layouts: &[
                        camera_bind_group_layout,
                        object_texture_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        gpu_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Object Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vert_shader_module,
                    entry_point: Some("main"),
                    buffers: &[ObjectVertex::desc(), ObjectInstanceData::desc()],
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
                    bias: wgpu::DepthBiasState {
                        constant: -2,
                        slope_scale: -0.5,
                        clamp: 0.0,
                    },
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

    pub fn apply_object_view_vec(
        object_view_vec: &Vec<ObjectView>,
        object_instance_data_group_vec: &mut Vec<(String, Vec<ObjectInstanceData>)>,
    ) {
        object_instance_data_group_vec.clear();

        let mut group_map: HashMap<String, Vec<ObjectInstanceData>> = HashMap::new();

        for object_view in object_view_vec {
            let world_position = grid::grid_position_to_world_position(object_view.grid_position);

            let rotation_xy = match object_view.direction {
                grid::Direction::East => 0.0f32.to_radians(),
                grid::Direction::West => 180.0f32.to_radians(),
                grid::Direction::North => 90.0f32.to_radians(),
                grid::Direction::South => 270.0f32.to_radians(),
                _ => 0.0,
            };

            let object_instance_data = ObjectInstanceData {
                world_position: *(world_position).as_array(),
                rotation_xy,
                _padding: [0.0, 0.0, 0.0],
            };

            let object_model_name = object::Kind::to_string(object_view.object_kind);

            group_map
                .entry(object_model_name)
                .or_default()
                .push(object_instance_data);
        }

        object_instance_data_group_vec.extend(group_map.into_iter());
    }

    pub fn render(
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        gpu_context: &GPUContext,
        camera_uniform_bind_group: &wgpu::BindGroup,
        object_renderer: &Self,
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

        render_pass.set_pipeline(&object_renderer.render_pipeline);
        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);

        let mut offset_bytes = 0;

        for (object_model_name, object_instance_data_vec) in
            &object_renderer.object_instance_data_group_vec
        {
            let byte_len = (object_instance_data_vec.len()
                * std::mem::size_of::<ObjectInstanceData>())
                as wgpu::BufferAddress;

            gpu_context.queue.write_buffer(
                &object_renderer.object_instance_buffer,
                offset_bytes,
                bytemuck::cast_slice(&object_instance_data_vec),
            );

            let object_gpu_mesh_arc = Arc::clone(
                object_renderer
                    .object_gpu_mesh_map
                    .get(object_model_name)
                    .unwrap(),
            );

            let texture_bind_group_arc = Arc::clone(
                object_renderer
                    .object_texture_bind_group_arc_map
                    .get(object_model_name)
                    .unwrap(),
            );

            render_pass.set_vertex_buffer(
                1,
                object_renderer
                    .object_instance_buffer
                    .slice(offset_bytes..offset_bytes + byte_len),
            );

            render_pass.set_bind_group(1, texture_bind_group_arc.deref(), &[]);

            render_pass.set_vertex_buffer(0, object_gpu_mesh_arc.vertex_buffer.slice(..));

            render_pass.set_index_buffer(
                object_gpu_mesh_arc.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            let instance_count = object_instance_data_vec.len() as u32;
            render_pass.draw_indexed(0..object_gpu_mesh_arc.index_count, 0, 0..instance_count);

            offset_bytes += byte_len;
        }

        drop(render_pass);
    }
}
