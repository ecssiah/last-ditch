use crate::{
    include_shader_src,
    interface::{
        chunk::{ChunkMesh, ChunkMeshCache, ChunkVertex, GpuChunkMeshCache},
        AO_INTENSITY, ASPECT_RATIO, DEBUG_COLOR, FAR_PLANE, FOV, NEAR_PLANE,
    },
    simulation::{
        agent::Agent, block, chunk::ChunkID, state::State, Simulation, CHUNK_VOLUME, WORLD_VOLUME,
    },
};
use glam::{IVec3, Mat4, Vec3};
use std::sync::{Arc, RwLock};
use winit::{event::WindowEvent, window::Window};

const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
]);

pub struct Render {
    last_render: u64,
    window: Arc<Window>,
    state: Arc<State>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    surface_config: wgpu::SurfaceConfiguration,
    view_projection_buffer: wgpu::Buffer,
    view_projection_bind_group: wgpu::BindGroup,
    chunk_pipeline: wgpu::RenderPipeline,
    chunk_mesh_cache: ChunkMeshCache,
    gpu_chunk_mesh_cache: GpuChunkMeshCache,
}

impl Render {
    pub async fn new(window: Arc<Window>, state: Arc<State>) -> Self {
        window.set_cursor_visible(false);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            view_formats: vec![surface_format],
            alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        surface.configure(&device, &surface_config);

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let view_projection_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("View Projection Buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("View Projection Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_projection_buffer.as_entire_binding(),
            }],
        });

        let chunk_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Chunk Shader"),
            source: wgpu::ShaderSource::Wgsl(include_shader_src!("chunk.wgsl").into()),
        });

        let chunk_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Chunk Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let chunk_pipeline = Render::create_chunk_render_pipeline(
            &device,
            &chunk_pipeline_layout,
            &chunk_shader,
            surface_format,
        );

        let chunk_mesh_cache = ChunkMeshCache::new();
        let gpu_chunk_mesh_cache = GpuChunkMeshCache::new();

        let render = Self {
            last_render: 0,
            window,
            state,
            device,
            queue,
            size,
            surface,
            surface_format,
            surface_config,
            view_projection_buffer,
            view_projection_bind_group,
            chunk_pipeline,
            chunk_mesh_cache,
            gpu_chunk_mesh_cache,
        };

        render
    }

    fn create_chunk_render_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        shader_module: &wgpu::ShaderModule,
        surface_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Chunk Mesh Pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: Some("vs_main"),
                buffers: &[ChunkVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.surface.configure(&self.device, &self.surface_config);
    }

    fn update(&mut self) {
        self.update_view_projection();

        let last_update = self.state.world.read().unwrap().last_update;

        if last_update > self.last_render {
            self.update_chunk_meshes();
            self.update_gpu_meshes();

            self.last_render = last_update;
        }
    }

    fn update_chunk_meshes(&mut self) {
        for chunk_id in 0..WORLD_VOLUME {
            let chunk = self.state.chunks[chunk_id].read().unwrap();

            if self
                .chunk_mesh_cache
                .needs_update(chunk_id, chunk.last_update)
            {
                let mut chunk_mesh = self.generate_chunk_mesh(chunk_id);

                chunk_mesh.last_render = chunk.last_update;

                self.chunk_mesh_cache.insert(chunk_id, chunk_mesh);
            }
        }
    }

    fn update_gpu_meshes(&mut self) {
        for (chunk_id, mesh) in self.chunk_mesh_cache.meshes.iter().enumerate() {
            if let Some(chunk_mesh) = mesh {
                let needs_upload = self.gpu_chunk_mesh_cache.get(chunk_id).is_none();

                if needs_upload {
                    self.gpu_chunk_mesh_cache
                        .upload_mesh(&self.device, chunk_id, chunk_mesh);
                }
            }
        }
    }

    fn generate_chunk_mesh(&self, chunk_id: ChunkID) -> ChunkMesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let chunk = self.state.chunks[chunk_id].read().unwrap();

        for block_id in 0..CHUNK_VOLUME {
            let grid_position = Simulation::ids_to_grid_position(chunk_id, block_id);

            let meta = &chunk.meta[block_id];
            let block = chunk.get_block(block_id).unwrap();

            for face in block::Face::ALL {
                if meta.visibility.contains(face) == false {
                    continue;
                }

                let face_vertices = self.generate_quad(grid_position, face);
                let face_normal = face.normal().as_vec3().to_array();

                let face_color = if DEBUG_COLOR {
                    face.debug_color()
                } else {
                    [block.color.0, block.color.1, block.color.2, block.color.3]
                };

                let face_ao = self.generate_ao(&meta, face);

                let chunk_vertices =
                    face_vertices
                        .iter()
                        .enumerate()
                        .map(|(face_index, face_vertex)| {
                            let vertex_position = face_vertex.to_array();
                            let vertex_ao = face_ao[face_index];

                            ChunkVertex {
                                position: vertex_position,
                                normal: face_normal,
                                color: face_color,
                                ao: vertex_ao,
                            }
                        });

                let start_index = vertices.len() as u32;

                let face_indices = [
                    start_index,
                    start_index + 1,
                    start_index + 2,
                    start_index,
                    start_index + 2,
                    start_index + 3,
                ];

                vertices.extend(chunk_vertices);
                indices.extend(face_indices);
            }
        }

        ChunkMesh {
            last_render: 0,
            vertices,
            indices,
        }
    }

    fn generate_quad(&self, grid_position: IVec3, face: block::Face) -> [Vec3; 4] {
        let base = grid_position.as_vec3();
        let offsets = face.quad_offsets();

        offsets.map(|(ox, oy, oz)| {
            let position = base + Vec3::new(ox, oy, oz);

            position
        })
    }

    fn generate_ao(&self, meta: &block::Meta, face: block::Face) -> [f32; 4] {
        let mut face_ao = [AO_INTENSITY[0]; 4];
        let neighbors = &meta.neighbors;

        match face {
            block::Face::XP => {
                let edge0 = neighbors.is_solid(block::Direction::XP_YN_Z0);
                let edge1 = neighbors.is_solid(block::Direction::XP_Y0_ZP);
                let edge3 = neighbors.is_solid(block::Direction::XP_Y0_ZN);
                let edge2 = neighbors.is_solid(block::Direction::XP_YP_Z0);

                if edge0 {
                    face_ao[0] = AO_INTENSITY[1];
                    face_ao[1] = AO_INTENSITY[1];
                }

                if edge1 {
                    face_ao[1] = AO_INTENSITY[1];
                    face_ao[2] = AO_INTENSITY[1];
                }

                if edge2 {
                    face_ao[2] = AO_INTENSITY[1];
                    face_ao[3] = AO_INTENSITY[1];
                }
                
                if edge3 {
                    face_ao[3] = AO_INTENSITY[1];
                    face_ao[0] = AO_INTENSITY[1];
                }

                if edge3 && edge0 {
                    face_ao[0] = AO_INTENSITY[2];
                }

                if edge0 && edge1 {
                    face_ao[1] = AO_INTENSITY[2];
                }

                if edge1 && edge2 {
                    face_ao[2] = AO_INTENSITY[2];
                }

                if edge2 && edge3 {
                    face_ao[3] = AO_INTENSITY[2];
                }

                face_ao
            }
            block::Face::XN => {
                let edge0 = neighbors.is_solid(block::Direction::XN_YN_Z0);
                let edge1 = neighbors.is_solid(block::Direction::XN_Y0_ZP);
                let edge2 = neighbors.is_solid(block::Direction::XN_YP_Z0);
                let edge3 = neighbors.is_solid(block::Direction::XN_Y0_ZN);

                if edge0 {
                    face_ao[0] = AO_INTENSITY[1];
                    face_ao[1] = AO_INTENSITY[1];
                }

                if edge1 {
                    face_ao[1] = AO_INTENSITY[1];
                    face_ao[2] = AO_INTENSITY[1];
                }

                if edge2 {
                    face_ao[2] = AO_INTENSITY[1];
                    face_ao[3] = AO_INTENSITY[1];
                }

                if edge3 {
                    face_ao[3] = AO_INTENSITY[1];
                    face_ao[0] = AO_INTENSITY[1];
                }

                if edge3 && edge0 {
                    face_ao[0] = AO_INTENSITY[2];
                }

                if edge0 && edge1 {
                    face_ao[1] = AO_INTENSITY[2];
                } 
                
                if edge1 && edge2 {
                    face_ao[2] = AO_INTENSITY[2];
                } 
                
                if edge2 && edge3 {
                    face_ao[3] = AO_INTENSITY[2];
                } 

                face_ao
            }
            block::Face::YP => {
                let edge0 = neighbors.is_solid(block::Direction::XN_YP_Z0);
                let edge1 = neighbors.is_solid(block::Direction::X0_YP_ZP);
                let edge2 = neighbors.is_solid(block::Direction::XP_YP_Z0);
                let edge3 = neighbors.is_solid(block::Direction::X0_YP_ZN);

                if edge1 {
                    face_ao[0] = AO_INTENSITY[1];
                    face_ao[1] = AO_INTENSITY[1];
                }

                if edge2 {
                    face_ao[1] = AO_INTENSITY[1];
                    face_ao[2] = AO_INTENSITY[1];
                }

                if edge3 {
                    face_ao[2] = AO_INTENSITY[1];
                    face_ao[3] = AO_INTENSITY[1];
                }

                if edge0 {
                    face_ao[3] = AO_INTENSITY[1];
                    face_ao[0] = AO_INTENSITY[1];
                }

                if edge3 && edge0 {
                    face_ao[3] = AO_INTENSITY[2];
                }

                if edge0 && edge1 {
                    face_ao[0] = AO_INTENSITY[2];
                }
                
                if edge1 && edge2 {
                    face_ao[1] = AO_INTENSITY[2];
                }
                
                if edge2 && edge3 {
                    face_ao[2] = AO_INTENSITY[2];
                }

                face_ao
            }
            block::Face::YN => {
                let edge0 = neighbors.is_solid(block::Direction::XN_YN_Z0);
                let edge1 = neighbors.is_solid(block::Direction::X0_YN_ZP);
                let edge2 = neighbors.is_solid(block::Direction::XP_YN_Z0);
                let edge3 = neighbors.is_solid(block::Direction::X0_YN_ZN);

                if edge0 {
                    face_ao[0] = AO_INTENSITY[1];
                    face_ao[3] = AO_INTENSITY[1];
                }

                if edge1 {
                    face_ao[3] = AO_INTENSITY[1];
                    face_ao[2] = AO_INTENSITY[1];
                }

                if edge2 {
                    face_ao[2] = AO_INTENSITY[1];
                    face_ao[1] = AO_INTENSITY[1];
                }

                if edge3 {
                    face_ao[1] = AO_INTENSITY[1];
                    face_ao[0] = AO_INTENSITY[1];
                }

                if edge0 && edge1 {
                    face_ao[3] = AO_INTENSITY[2];
                }
                
                if edge1 && edge2 {
                    face_ao[2] = AO_INTENSITY[2];
                }
                
                if edge2 && edge3 {
                    face_ao[1] = AO_INTENSITY[2];
                }
                
                if edge3 && edge0 {
                    face_ao[0] = AO_INTENSITY[2];
                }

                face_ao
            }
            block::Face::ZP => {
                let edge0 = neighbors.is_solid(block::Direction::XN_Y0_ZP);
                let edge1 = neighbors.is_solid(block::Direction::X0_YP_ZP);
                let edge2 = neighbors.is_solid(block::Direction::XP_Y0_ZP);
                let edge3 = neighbors.is_solid(block::Direction::X0_YN_ZP);

                if edge0 {
                    face_ao[0] = AO_INTENSITY[1];
                    face_ao[3] = AO_INTENSITY[1];
                }

                if edge1 {
                    face_ao[3] = AO_INTENSITY[1];
                    face_ao[2] = AO_INTENSITY[1];
                }

                if edge2 {
                    face_ao[2] = AO_INTENSITY[1];
                    face_ao[1] = AO_INTENSITY[1];
                }

                if edge3 {
                    face_ao[1] = AO_INTENSITY[1];
                    face_ao[0] = AO_INTENSITY[1];
                }

                if edge0 && edge1 {
                    face_ao[3] = AO_INTENSITY[2];
                }
                
                if edge1 && edge2 {
                    face_ao[2] = AO_INTENSITY[2];
                }
                
                if edge2 && edge3 {
                    face_ao[1] = AO_INTENSITY[2];
                }
                
                if edge3 && edge0 {
                    face_ao[0] = AO_INTENSITY[2];
                }

                face_ao
            }
            block::Face::ZN => {
                let edge0 = neighbors.is_solid(block::Direction::XN_Y0_ZN);
                let edge1 = neighbors.is_solid(block::Direction::X0_YP_ZN);
                let edge2 = neighbors.is_solid(block::Direction::XP_Y0_ZN);
                let edge3 = neighbors.is_solid(block::Direction::X0_YN_ZN);

                if edge0 {
                    face_ao[0] = AO_INTENSITY[1];
                    face_ao[3] = AO_INTENSITY[1];
                }

                if edge1 {
                    face_ao[3] = AO_INTENSITY[1];
                    face_ao[2] = AO_INTENSITY[1];
                }

                if edge2 {
                    face_ao[2] = AO_INTENSITY[1];
                    face_ao[1] = AO_INTENSITY[1];
                }

                if edge3 {
                    face_ao[1] = AO_INTENSITY[1];
                    face_ao[0] = AO_INTENSITY[1];
                }

                if edge0 && edge1 {
                    face_ao[3] = AO_INTENSITY[2];
                }

                if edge1 && edge2 {
                    face_ao[2] = AO_INTENSITY[2];
                }

                if edge2 && edge3 {
                    face_ao[1] = AO_INTENSITY[2];
                }

                if edge3 && edge0 {
                    face_ao[0] = AO_INTENSITY[2];
                }

                face_ao
            }
            _ => face_ao,
        }
    }

    fn weighted_corner_ao(&self, edge_flags: [bool; 4]) -> [f32; 4] {
        let mut ao = [AO_INTENSITY[0]; 4];

        if edge_flags[0] && edge_flags[1] {
            ao[1] = AO_INTENSITY[2];
        } else if edge_flags[1] && edge_flags[2] {
            ao[3] = AO_INTENSITY[2];
        } else if edge_flags[2] && edge_flags[3] {
            ao[2] = AO_INTENSITY[2];
        } else if edge_flags[3] && edge_flags[0] {
            ao[0] = AO_INTENSITY[2];
        } else {
            if edge_flags[0] {
                ao[0] = AO_INTENSITY[1];
                ao[1] = AO_INTENSITY[1];
            }
            if edge_flags[1] {
                ao[1] = AO_INTENSITY[1];
                ao[3] = AO_INTENSITY[1];
            }
            if edge_flags[2] {
                ao[3] = AO_INTENSITY[1];
                ao[2] = AO_INTENSITY[1];
            }
            if edge_flags[3] {
                ao[2] = AO_INTENSITY[1];
                ao[0] = AO_INTENSITY[1];
            }
        }

        ao
    }

    fn update_view_projection(&mut self) {
        let view_projection_matrix = Self::create_view_projection_matrix(self.state.agent.clone());

        self.queue.write_buffer(
            &self.view_projection_buffer,
            0,
            bytemuck::cast_slice(&view_projection_matrix),
        );
    }

    pub fn render(&self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let depth_texture_view = Self::create_depth_texture(&self.device, &self.surface_config);

        let mut encoder = self.device.create_command_encoder(&Default::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("World Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.chunk_pipeline);
        render_pass.set_bind_group(0, &self.view_projection_bind_group, &[]);

        for mesh in self.gpu_chunk_mesh_cache.meshes.values() {
            if mesh.index_count > 0 {
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
            }
        }

        drop(render_pass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();

        surface_texture.present();
    }

    fn create_view_projection_matrix(agent: Arc<RwLock<Agent>>) -> [[f32; 4]; 4] {
        let agent = agent.read().unwrap();

        let opengl_projection =
            Mat4::perspective_rh(FOV.to_radians(), ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);
        let projection = OPENGL_TO_WGPU_MATRIX * opengl_projection;

        let forward = agent.look_rotation * Vec3::Z;
        let up = agent.look_rotation * Vec3::Y;

        let eye = agent.position;
        let target = eye + forward;

        let view = Mat4::look_at_rh(eye, target, up);

        let view_projection = projection * view;

        view_projection.to_cols_array_2d()
    }

    fn create_depth_texture(
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

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                self.update();
                self.render();

                self.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.resize(*size);
            }
            _ => (),
        }
    }
}
