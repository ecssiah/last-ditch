//! Primary drawing system

pub mod agent_render;
pub mod chunk_render;
pub mod data;
pub mod texture_atlas;
pub mod textures;

pub use agent_render::AgentRender;
pub use chunk_render::ChunkRender;
pub use texture_atlas::TextureAtlas;
pub use textures::Textures;

use crate::{
    interface::{
        camera::Camera,
        render::data::{AgentInstanceData, BlockRenderData, ChunkData, MeshData, VertexData},
    },
    simulation,
};
use std::collections::HashMap;

pub struct Render {
    pub block_render_data_map: HashMap<simulation::state::world::block::Kind, BlockRenderData>,
    pub textures: Textures,
    pub chunk_render: ChunkRender,
    pub agent_render: AgentRender,
}

impl Render {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_format: &wgpu::TextureFormat,
        camera: &Camera,
    ) -> Self {
        let block_render_data_map = BlockRenderData::setup();

        let mut textures = Textures::new(&device);

        pollster::block_on(textures.load_texture_atlas(
            &device,
            &queue,
            &"assets/textures/atlas.png".to_string(),
            "atlas",
        ));

        textures.setup_texture_sampler_bind_group(&device);

        let chunk_render = ChunkRender::new(
            &device,
            &surface_format,
            &camera.uniform_bind_group_layout,
            &textures.texture_sampler_bind_group_layout,
        );

        let agent_render =
            AgentRender::new(&device, &surface_format, &camera.uniform_bind_group_layout);

        Self {
            block_render_data_map,
            textures,
            chunk_render,
            agent_render,
        }
    }

    pub fn prepare_agent_view_map(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        agent_view_map: &HashMap<
            simulation::state::population::entity::ID,
            simulation::observation::view::AgentView,
        >,
    ) {
        self.agent_render.instance_data_list = agent_view_map
            .iter()
            .map(|(_, agent_view)| {
                let agent_instance_data = AgentInstanceData {
                    world_position: agent_view.spatial.current.world_position.to_array(),
                    height: agent_view.spatial.current.aabb.size().y,
                    color: agent_view.kind.color(),
                };

                agent_instance_data
            })
            .collect();

        let required_size = (agent_view_map.len() * std::mem::size_of::<AgentInstanceData>())
            as wgpu::BufferAddress;

        if self.agent_render.instance_buffer.size() < required_size {
            self.agent_render.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Agent Instance Buffer"),
                size: required_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(
            &self.agent_render.instance_buffer,
            0,
            bytemuck::cast_slice(&self.agent_render.instance_data_list),
        );
    }

    pub fn prepare_world_view(
        &mut self,
        device: &wgpu::Device,
        world_view: &simulation::observation::view::WorldView,
    ) {
        self.chunk_render.chunk_data_list.clear();

        for (chunk_id, chunk_view) in &world_view.chunk_view_map {
            let mut vertex_list = Vec::new();
            let mut index_list = Vec::new();
            let mut index_offset = 0;

            for face in &chunk_view.geometry.next.face_list {
                if face.kind == simulation::state::world::block::Kind::Empty {
                    continue;
                }

                let face_vertex_list = face.vertices();
                let block_render_data = self.block_render_data_map.get(&face.kind).unwrap();

                let tile_position_index = block_render_data.direction_to_index(face.direction);
                let tile_position = block_render_data.tile_index_array[tile_position_index];

                let uv_coordinates = self
                    .textures
                    .texture_atlas
                    .get_uv_coords(tile_position[0], tile_position[1]);

                for (index, vertex) in face_vertex_list.iter().enumerate() {
                    vertex_list.push(VertexData {
                        position: vertex.to_array(),
                        normal: face.normal().as_vec3().to_array(),
                        uv: uv_coordinates[index],
                        light: face.light[index],
                    });
                }

                index_list.push(index_offset + 0);
                index_list.push(index_offset + 1);
                index_list.push(index_offset + 2);
                index_list.push(index_offset + 0);
                index_list.push(index_offset + 2);
                index_list.push(index_offset + 3);

                index_offset += 4;
            }

            let chunk_id = *chunk_id;

            let chunk = ChunkData {
                chunk_id,
                mesh_data: MeshData::new(device, vertex_list, index_list),
            };

            self.chunk_render.chunk_data_list.push(chunk);
        }
    }

    pub fn update(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        texture_view: &wgpu::TextureView,
        camera: &Camera,
    ) {
        let depth_texture_view = Textures::create_depth_texture(device, &surface_config);

        if let Some(ref texture_sampler_bind_group) = self.textures.texture_sampler_bind_group {
            self.chunk_render.render(
                encoder,
                texture_view,
                &depth_texture_view,
                &camera.uniform_bind_group,
                texture_sampler_bind_group,
            );
        }

        self.agent_render.render(
            encoder,
            texture_view,
            &depth_texture_view,
            &camera.uniform_bind_group,
        );
    }
}
