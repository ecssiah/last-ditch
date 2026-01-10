pub mod person_instance_data;
pub mod person_model;
pub mod person_vertex_data;

use crate::{
    include_assets,
    interface::{
        asset_manager::{
            person_model_key::PersonModelKey, person_texture_key::PersonTextureKey, AssetManager,
        },
        camera::Camera,
        gpu::gpu_context::GPUContext,
        renderer::population_renderer::{
            person_renderer::{
                self, person_instance_data::PersonInstanceData,
                person_vertex_data::PersonVertexData,
            },
            PopulationRenderer,
        },
    },
    simulation::{
        constants::PERSON_DEFAULT_RADIUS_Z,
        state::{physics::body::Body, population::person::person_id::PersonID},
        supervisor::viewer::view::PopulationView,
    },
};
use std::{collections::HashMap, sync::Arc};
use tracing::instrument;

pub struct PersonRenderer {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: Option<wgpu::BindGroup>,
    pub person_instance_buffer: wgpu::Buffer,
    pub person_instance_data_group_vec: Vec<(PersonModelKey, Vec<PersonInstanceData>)>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl PersonRenderer {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let person_instance_buffer = gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Person Instance Buffer"),
            size: std::mem::size_of::<PersonInstanceData>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let person_instance_data_group_vec = Vec::new();

        let bind_group_layout =
            gpu_context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Population Renderer Bind Group Layout"),
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
            person_instance_buffer,
            person_instance_data_group_vec,
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
        person_texture_bind_group_layout: &wgpu::BindGroupLayout,
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
                        person_texture_bind_group_layout,
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
                    buffers: &[PersonVertexData::desc(), PersonInstanceData::desc()],
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
    pub fn apply_population_view(
        gpu_context: &GPUContext,
        asset_manager: &AssetManager,
        population_view: &PopulationView,
        person_renderer: &mut Self,
    ) {
        person_renderer.person_instance_data_group_vec.clear();

        let mut person_model_instance_map: HashMap<PersonModelKey, Vec<PersonInstanceData>> =
            HashMap::new();

        for (person_id, person_view) in &population_view.person_view_map {
            if person_id == &PersonID::JUDGE_ID_1 {
                continue;
            }

            let world_position = *(person_view.transform.world_position).as_array();
            let person_scale = Body::get_size(&person_view.body).z / PERSON_DEFAULT_RADIUS_Z;
            let rotation_xy = person_view.transform.rotation_xy;
            let layer_index = AssetManager::get_person_layer_index(
                &PersonTextureKey::from_skin_tone(&person_view.appearance.skin_tone),
                asset_manager,
            );

            let person_instance_data = PersonInstanceData::new(
                world_position,
                person_scale,
                rotation_xy,
                layer_index.into(),
            );

            let person_model_key = PersonModelKey::from_sex_and_age(
                &person_view.identity.sex,
                person_view.identity.age.period,
            );

            person_model_instance_map
                .entry(person_model_key)
                .or_default()
                .push(person_instance_data);
        }

        person_renderer
            .person_instance_data_group_vec
            .extend(person_model_instance_map.into_iter());

        let total_instance_count: usize = person_renderer
            .person_instance_data_group_vec
            .iter()
            .map(|(_, v)| v.len())
            .sum();

        let byte_count_required = (total_instance_count * std::mem::size_of::<PersonInstanceData>())
            as wgpu::BufferAddress;

        let byte_count_current = person_renderer.person_instance_buffer.size();

        if byte_count_required > byte_count_current {
            let byte_count_updated = byte_count_required
                .next_power_of_two()
                .max(byte_count_current);

            person_renderer.person_instance_buffer =
                gpu_context.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Person Instance Buffer"),
                    size: byte_count_updated,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

            tracing::info!(
                "Resized person instance buffer: {} -> {} bytes",
                byte_count_current,
                byte_count_updated
            );
        }
    }

    #[instrument(skip_all)]
    pub fn render(
        gpu_context: &GPUContext,
        surface_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_uniform_bind_group: &wgpu::BindGroup,
        asset_manager: &AssetManager,
        person_renderer: &Self,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if person_renderer.bind_group.is_none() {
            return;
        }

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

        let bind_group = person_renderer
            .bind_group
            .as_ref()
            .expect("population bind group must exist at this point");

        render_pass.set_pipeline(&person_renderer.render_pipeline);

        render_pass.set_bind_group(0, camera_uniform_bind_group, &[]);
        render_pass.set_bind_group(1, bind_group, &[]);

        let mut offset_bytes = 0;

        for (person_model_key, person_instance_data_vec) in
            &person_renderer.person_instance_data_group_vec
        {
            let byte_len = (person_instance_data_vec.len()
                * std::mem::size_of::<PersonInstanceData>())
                as wgpu::BufferAddress;

            gpu_context.queue.write_buffer(
                &person_renderer.person_instance_buffer,
                offset_bytes,
                bytemuck::cast_slice(&person_instance_data_vec),
            );

            let person_gpu_mesh =
                AssetManager::get_person_model_gpu_mesh(person_model_key, asset_manager);

            render_pass.set_vertex_buffer(
                1,
                person_renderer
                    .person_instance_buffer
                    .slice(offset_bytes..offset_bytes + byte_len),
            );

            render_pass.set_vertex_buffer(0, person_gpu_mesh.vertex_buffer.slice(..));

            render_pass.set_index_buffer(
                person_gpu_mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            let instance_count = person_instance_data_vec.len() as u32;
            render_pass.draw_indexed(0..person_gpu_mesh.index_count, 0, 0..instance_count);

            offset_bytes += byte_len;
        }

        drop(render_pass);
    }
}
