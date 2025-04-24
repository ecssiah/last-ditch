pub mod agent_instance_data;
pub mod agent_render;
pub mod camera_uniform_data;
pub mod chunk_render;
pub mod fog;
pub mod fog_uniform_data;
pub mod gpu_block;
pub mod gpu_chunk;
pub mod gpu_mesh;
pub mod texture_atlas;
pub mod textures;
pub mod vertex_data;

use std::{collections::HashMap, sync::Arc};

pub use agent_instance_data::AgentInstanceData;
pub use agent_render::EntityRender;
pub use chunk_render::ChunkRender;
pub use gpu_block::GPUBlock;
pub use gpu_chunk::GPUChunk;
pub use gpu_mesh::GPUMesh;
pub use texture_atlas::TextureAtlas;
pub use textures::Textures;
pub use vertex_data::VertexData;

use crate::{
    interface::{camera::Camera, consts::GPU_BLOCKS, render::fog::Fog},
    simulation,
};

pub struct Render {
    // size: winit::dpi::PhysicalSize<u32>,
    // surface: wgpu::Surface<'static>,
    // surface_config: wgpu::SurfaceConfiguration,
    // surface_texture_view_descriptor: wgpu::TextureViewDescriptor<'static>,
    textures: Textures,
    fog: Fog,
    chunk_render: ChunkRender,
    entity_render: EntityRender,
}

impl Render {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        window: Arc<winit::window::Window>,
        instance: &wgpu::Instance,
        adapter: &wgpu::Adapter,
        surface_format: &wgpu::TextureFormat,
        camera: &Camera,
    ) -> Render {
        // let size = window.inner_size();

        // let surface = instance.create_surface(window.clone()).unwrap();
        // let surface_capabilities = surface.get_capabilities(adapter);
        // let surface_format = surface_capabilities.formats[0];

        // let surface_texture_view_descriptor = wgpu::TextureViewDescriptor {
        //     format: Some(surface_format.add_srgb_suffix()),
        //     ..Default::default()
        // };

        // let surface_config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: surface_format,
        //     view_formats: vec![surface_format],
        //     alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
        //     width: size.width,
        //     height: size.height,
        //     desired_maximum_frame_latency: 2,
        //     present_mode: wgpu::PresentMode::AutoVsync,
        // };

        // surface.configure(&device, &surface_config);

        let mut textures = Textures::new(&device);

        pollster::block_on(textures.load_texture_atlas(
            &device,
            &queue,
            &"assets/textures/atlas.png".to_string(),
            "atlas",
        ));

        textures.generate_texture_sampler_bind_group(&device);

        let fog = Fog::new(&device);

        let chunk_render = ChunkRender::new(
            &device,
            &surface_format,
            &fog.uniform_bind_group_layout,
            &camera.uniform_bind_group_layout,
            &textures.texture_sampler_bind_group_layout,
        );

        let entity_render = EntityRender::new(
            &device,
            &surface_format,
            &fog.uniform_bind_group_layout,
            &camera.uniform_bind_group_layout,
        );

        let render = Render {
            textures,
            fog,
            chunk_render,
            entity_render,
        };

        render
    }

    pub fn prepare_agent_views(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        agent_views: &HashMap<
            simulation::population::agent::ID,
            simulation::observation::view::AgentView,
        >,
    ) {
        self.entity_render.gpu_entities = agent_views
            .iter()
            .map(|(_, agent_view)| {
                let agent_instance_data = AgentInstanceData {
                    position: agent_view.position.next.to_array(),
                    height: agent_view.height,
                    color: agent_view.kind.color(),
                };

                agent_instance_data
            })
            .collect();

        let required_size =
            (agent_views.len() * std::mem::size_of::<AgentInstanceData>()) as wgpu::BufferAddress;

        if self.entity_render.instance_buffer.size() < required_size {
            self.entity_render.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Agent Instance Buffer"),
                size: required_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(
            &self.entity_render.instance_buffer,
            0,
            bytemuck::cast_slice(&self.entity_render.gpu_entities),
        );
    }

    pub fn prepare_world_view(
        &mut self,
        device: &wgpu::Device,
        world_view: &simulation::observation::view::WorldView,
    ) {
        self.chunk_render.gpu_chunks.clear();

        for (chunk_id, chunk_view) in &world_view.chunk_views {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
            let mut index_offset = 0;

            for face in &chunk_view.mesh.next.faces {
                if face.kind == simulation::block::Kind::Air {
                    continue;
                }

                let face_vertices = face.vertices();
                let render_block = GPU_BLOCKS.get(&face.kind).unwrap();
                let atlas_coordinates =
                    render_block.atlas_coordinates.get(&face.direction).unwrap();

                let uvs = self
                    .textures
                    .texture_atlas
                    .get_uv_coords(atlas_coordinates[0], atlas_coordinates[1]);

                for (index, vertex) in face_vertices.iter().enumerate() {
                    vertices.push(VertexData {
                        position: vertex.to_array(),
                        normal: face.normal().as_vec3().to_array(),
                        uv: uvs[index].to_array(),
                        light: face.light[index],
                    });
                }

                indices.push(index_offset + 0);
                indices.push(index_offset + 1);
                indices.push(index_offset + 2);
                indices.push(index_offset + 0);
                indices.push(index_offset + 2);
                indices.push(index_offset + 3);

                index_offset += 4;
            }

            let chunk_id = *chunk_id;

            let chunk = GPUChunk {
                chunk_id,
                tick: chunk_view.tick.next,
                gpu_mesh: GPUMesh::new(device, vertices, indices),
            };

            self.chunk_render.gpu_chunks.push(chunk);
        }
    }

    // pub fn resize(&mut self, device: &wgpu::Device, new_size: winit::dpi::PhysicalSize<u32>) {
    //     self.size = new_size;

    //     self.surface.configure(device, &self.surface_config);
    // }

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
                &self.fog.uniform_bind_group,
                &camera.uniform_bind_group,
                texture_sampler_bind_group,
            );
        }

        self.entity_render.render(
            encoder,
            texture_view,
            &depth_texture_view,
            &self.fog.uniform_bind_group,
            &camera.uniform_bind_group,
        );
    }
}
