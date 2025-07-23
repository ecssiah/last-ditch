//! Primary drawing system

pub mod data;
pub mod mesh_render;

pub use mesh_render::MeshRender;

use crate::{
    include_assets,
    interface::{
        camera::Camera,
        consts::*,
        gpu_context::GPUContext,
        render::data::{BlockAtlasData, MeshData, RenderData, VertexData},
    },
    simulation::{self, observation::view::PopulationView, state::population::entity},
};
use glam::{IVec2, Mat4};
use std::collections::HashMap;

pub struct Render {
    pub block_atlas_data_map: HashMap<simulation::state::world::block::Kind, BlockAtlasData>,
    pub mesh_render: MeshRender,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Render {
    pub fn new(gpu_context: &GPUContext, camera: &Camera) -> Self {
        let block_atlas_data_map = BlockAtlasData::setup();

        let mesh_render = MeshRender::new(gpu_context);

        let render_pipeline = Self::create_render_pipeline(
            gpu_context,
            &camera.uniform_bind_group_layout,
            &mesh_render.texture_bind_group_layout,
        );

        Self {
            block_atlas_data_map,
            mesh_render,
            render_pipeline,
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

    pub fn apply_population_view(population_view: &PopulationView, mesh_render: &mut MeshRender) {
        let entity_render_data_vec = mesh_render
            .render_data_map
            .get_mut(&mesh_render::RenderType::Entity)
            .unwrap();

        entity_render_data_vec.clear();

        for agent_view in population_view.agent_view_map.values() {
            let mesh_data = mesh_render
                .mesh_data_map
                .get(&agent_view.kind)
                .unwrap()
                .clone();

            let transform = Mat4::from_translation(agent_view.spatial.world_position);

            let texture_bind_group = mesh_render
                .texture_bind_group_map
                .get(&format!("agent_{:?}", agent_view.kind))
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
        texture_bind_group_map: &HashMap<String, wgpu::BindGroup>,
        block_atlas_data_map: &HashMap<simulation::state::world::block::Kind, BlockAtlasData>,
        block_render_data_vec: &mut Vec<RenderData>,
    ) {
        block_render_data_vec.clear();

        for chunk_view in world_view.chunk_view_map.values() {
            let mut vertex_vec = Vec::new();
            let mut index_vec = Vec::new();
            let mut index_offset = 0;

            for face in &chunk_view.geometry.face_vec {
                if face.kind == simulation::state::world::block::Kind::Empty {
                    continue;
                }

                let face_vertex_vec = face.vertices();
                let block_render_data = block_atlas_data_map.get(&face.kind).unwrap();

                let tile_face_index = BlockAtlasData::face_direction_to_index(face.direction);

                let tile_coordinates = IVec2::new(
                    block_render_data.tile_index_array[tile_face_index][0] as i32,
                    block_render_data.tile_index_array[tile_face_index][1] as i32,
                );

                let tile_size = 64;
                let tile_atlas_size = IVec2::new(32, 32);

                let uv_coordinates =
                    BlockAtlasData::uv_coordinates(tile_coordinates, tile_size, tile_atlas_size);

                for (index, vertex) in face_vertex_vec.iter().enumerate() {
                    vertex_vec.push(VertexData {
                        position: vertex.to_array(),
                        normal: face.normal().as_vec3().to_array(),
                        uv: uv_coordinates[index],
                    });
                }

                index_vec.push(index_offset);
                index_vec.push(index_offset + 1);
                index_vec.push(index_offset + 2);
                index_vec.push(index_offset);
                index_vec.push(index_offset + 2);
                index_vec.push(index_offset + 3);

                index_offset += 4;
            }

            let render_data = RenderData {
                mesh_data: MeshData::new(device, vertex_vec, index_vec),
                transform: Mat4::IDENTITY,
                texture_bind_group: texture_bind_group_map["Block"].clone(),
            };

            block_render_data_vec.push(render_data);
        }
    }

    pub fn update(
        gpu_context: &GPUContext,
        mesh_render: &MeshRender,
        camera: &Camera,
        surface_texture_view: &wgpu::TextureView,
        render_pipeline: &wgpu::RenderPipeline,
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
        render_pass.set_bind_group(0, &camera.uniform_bind_group, &[]);

        MeshRender::render(&mesh_render.render_data_map, &mut render_pass);

        drop(render_pass);
    }
}
