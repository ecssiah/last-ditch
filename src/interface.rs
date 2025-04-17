//! The Interface module manages user interaction with the Simulation. This includes
//! both presentation and input management.

pub mod consts;
pub mod gpu_block;
pub mod gpu_chunk;
pub mod gpu_entity;
pub mod input;
pub mod render;

use crate::{
    include_assets,
    interface::{
        consts::*,
        gpu_chunk::{
            gpu_vertex::{self, GPUVertex},
            GPUChunk, GPUMesh,
        },
        gpu_entity::GPUEntity,
        input::Input,
        render::Textures,
    },
    simulation::{self, observation::view::entity_view, USER_VIEW_OFFSET},
};
use glam::{Mat4, Vec3};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;
use wgpu::{util::DeviceExt, Adapter, Device, Instance, PipelineCompilationOptions, Queue};
use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    window::Window,
};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
]);

pub struct Interface {
    delta_time: Duration,
    render_instant: Instant,
    render_alpha: f32,
    action_tx: UnboundedSender<simulation::dispatch::Action>,
    observation_lock: Arc<RwLock<simulation::observation::Observation>>,
    window: Arc<Window>,
    input: Input,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    surface_texture_view_descriptor: wgpu::TextureViewDescriptor<'static>,
    view_projection_buffer: wgpu::Buffer,
    view_projection_bind_group: wgpu::BindGroup,
    texture_sampler_bind_group: wgpu::BindGroup,
    textures: Textures,
    chunk_pipeline: wgpu::RenderPipeline,
    gpu_chunks: HashMap<simulation::chunk::ID, GPUChunk>,
    entity_vertex_buffer: wgpu::Buffer,
    entity_instance_buffer: wgpu::Buffer,
    gpu_entity_vertices: Vec<gpu_vertex::GPUVertex>,
    gpu_entities: Vec<gpu_entity::GPUEntity>,
    entity_pipeline: wgpu::RenderPipeline,
}

impl Interface {
    pub fn new(
        action_tx: UnboundedSender<simulation::dispatch::Action>,
        observation_lock: Arc<RwLock<simulation::observation::Observation>>,
        window: Arc<Window>,
        instance: Instance,
        adapter: Adapter,
        device: Device,
        queue: Queue,
    ) -> Self {
        let input = Input::new(action_tx.clone());

        let size = window.inner_size();

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
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

        let mut textures = Textures::new();

        pollster::block_on(textures.load_texture_atlas(
            &device,
            &queue,
            &"assets/textures/atlas.png".to_string(),
            "atlas",
        ));

        let texture_sampler_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture and Sampler Bind Group Layout"),
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

        let (_, atlas_texture_view, atlas_sampler) = textures.texture_map.get("atlas").unwrap();

        let texture_sampler_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture and Sampler Bind Group"),
            layout: &texture_sampler_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&atlas_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&atlas_sampler),
                },
            ],
        });

        let chunk_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Chunk Shader"),
            source: wgpu::ShaderSource::Wgsl(include_assets!("shaders/chunk.wgsl").into()),
        });

        let chunk_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Chunk Pipeline Layout"),
                bind_group_layouts: &[
                    &uniform_bind_group_layout,
                    &texture_sampler_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let chunk_pipeline = Self::create_chunk_render_pipeline(
            &device,
            &chunk_pipeline_layout,
            &chunk_shader,
            surface_format,
        );

        let surface_texture_view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(surface_format.add_srgb_suffix()),
            ..Default::default()
        };

        let entity_sphere_vertices = Self::generate_sphere(8, 8);
        let entity_rectangle_vertices = Self::generate_rectangle();
        let gpu_entity_vertices = [entity_sphere_vertices, entity_rectangle_vertices].concat();

        let entity_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Entity Vertex Buffer"),
            contents: bytemuck::cast_slice(&gpu_entity_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let entity_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Entity Instance Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let entity_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Entity Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let entity_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Entity Shader"),
            source: wgpu::ShaderSource::Wgsl(include_assets!("shaders/entity.wgsl").into()),
        });

        let entity_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Entity Render Pipeline"),
            layout: Some(&entity_render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &entity_shader_module,
                entry_point: Some("vs_main"),
                buffers: &[GPUVertex::desc(), GPUEntity::desc()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &entity_shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let gpu_chunks = HashMap::new();
        let gpu_entities = Vec::new();

        let delta_time = Duration::ZERO;
        let render_instant = Instant::now();
        let render_alpha = 0.0;

        let interface = Self {
            delta_time,
            render_instant,
            render_alpha,
            action_tx,
            observation_lock,
            window,
            input,
            device,
            queue,
            size,
            surface,
            surface_config,
            surface_texture_view_descriptor,
            view_projection_buffer,
            view_projection_bind_group,
            texture_sampler_bind_group,
            chunk_pipeline,
            gpu_chunks,
            gpu_entities,
            gpu_entity_vertices,
            entity_vertex_buffer,
            entity_instance_buffer,
            entity_pipeline,
            textures,
        };

        log::info!("Interface Initialized");

        interface
    }

    pub fn setup(&mut self) {}

    fn check_active(
        &mut self,
        event_loop: &ActiveEventLoop,
        view: &simulation::observation::view::View,
    ) {
        if view.admin_view.mode == simulation::admin::Mode::Exit {
            event_loop.exit();
        }
    }

    fn send_movement_actions(&mut self) {
        let movement_actions = self.input.get_movement_actions();
        let entity_action = simulation::dispatch::EntityAction::Movement(movement_actions);
        let action = simulation::dispatch::Action::Agent(entity_action);

        self.action_tx.send(action).unwrap();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn handle_about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(view) = self.get_view(&simulation::population::entity::ID::USER_ENTITY1) {
            self.check_active(event_loop, &view);
            self.send_movement_actions();

            self.update_view(&view);
        }
    }

    fn get_view(
        &self,
        entity_id: &simulation::population::entity::ID,
    ) -> Option<simulation::observation::view::View> {
        let observation = self.observation_lock.read().unwrap();

        observation.get_view(entity_id)
    }

    fn update_view(&mut self, view: &simulation::observation::view::View) {
        if let Some(entity_view) = view.population_view.entity_views.get(&view.entity_id) {
            self.update_render_alpha(&view.time_view);

            self.update_entity_view(&entity_view);

            self.update_population_view(&view.population_view);
            self.update_world_view(&view.world_view);
        }
    }

    fn update_render_alpha(&mut self, time_view: &simulation::observation::view::TimeView) {
        let now = Instant::now();
        self.delta_time = now - self.render_instant;
        self.render_instant = now;

        let render_alpha = (now - time_view.simulation_instant).as_secs_f32();
        self.render_alpha = render_alpha.clamp(0.0, 1.0);
    }

    fn update_entity_view(&mut self, entity_view: &simulation::observation::view::EntityView) {
        self.update_view_projection(entity_view);
    }

    fn update_population_view(
        &mut self,
        population_view: &simulation::observation::view::PopulationView,
    ) {
        self.gpu_entities = population_view
            .entity_views
            .iter()
            .map(|entity_view| GPUEntity {
                position: entity_view.1.position.to_array(),
                height: 1.8,
            })
            .collect();

        let required_size = (population_view.entity_views.len() * std::mem::size_of::<GPUEntity>())
            as wgpu::BufferAddress;

        if self.entity_instance_buffer.size() < required_size {
            self.entity_instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Entity Instance Buffer"),
                size: required_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        self.queue.write_buffer(
            &self.entity_instance_buffer,
            0,
            bytemuck::cast_slice(&self.gpu_entities),
        );
    }

    fn update_world_view(&mut self, world_view: &simulation::observation::view::WorldView) {
        self.gpu_chunks.clear();

        for (chunk_id, chunk_view) in &world_view.chunk_views {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
            let mut index_offset = 0;

            for face in &chunk_view.mesh.faces {
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
                    vertices.push(GPUVertex {
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
                tick: chunk_view.tick,
                gpu_mesh: GPUMesh::new(&self.device, vertices, indices),
            };

            self.gpu_chunks.insert(chunk_id, chunk);
        }
    }

    fn update_view_projection(&mut self, entity_view: &simulation::observation::view::EntityView) {
        let view_projection_matrix = self.create_view_projection_matrix(entity_view);

        self.queue.write_buffer(
            &self.view_projection_buffer,
            0,
            bytemuck::cast_slice(&view_projection_matrix),
        );
    }

    pub fn handle_redraw_requested(&mut self) {
        let mut encoder = self.device.create_command_encoder(&Default::default());

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let texture_view = surface_texture
            .texture
            .create_view(&self.surface_texture_view_descriptor);

        let chunk_render_pass_color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
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

        let depth_texture_view = Self::create_depth_texture(&self.device, &self.surface_config);

        let chunk_depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        });

        let entity_render_pass_color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        });

        let mut chunk_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Chunk Render Pass"),
            color_attachments: &[chunk_render_pass_color_attachment],
            depth_stencil_attachment: chunk_depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        chunk_render_pass.set_pipeline(&self.chunk_pipeline);
        chunk_render_pass.set_bind_group(0, &self.view_projection_bind_group, &[]);
        chunk_render_pass.set_bind_group(1, &self.texture_sampler_bind_group, &[]);

        for gpu_chunk in self.gpu_chunks.values() {
            if gpu_chunk.gpu_mesh.index_count > 0 {
                chunk_render_pass.set_vertex_buffer(0, gpu_chunk.gpu_mesh.vertex_buffer.slice(..));

                chunk_render_pass.set_index_buffer(
                    gpu_chunk.gpu_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                chunk_render_pass.draw_indexed(0..gpu_chunk.gpu_mesh.index_count, 0, 0..1);
            }
        }

        drop(chunk_render_pass);

        if self.gpu_entities.len() > 0 {
            let mut entity_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Entity Render Pass"),
                color_attachments: &[entity_render_pass_color_attachment],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            entity_render_pass.set_pipeline(&self.entity_pipeline);
            entity_render_pass.set_vertex_buffer(0, self.entity_vertex_buffer.slice(..));
            entity_render_pass.set_vertex_buffer(1, self.entity_instance_buffer.slice(..));
            entity_render_pass.draw(
                0..(self.gpu_entity_vertices.len() as u32),
                0..(self.gpu_entities.len() as u32),
            );

            drop(entity_render_pass);
        }

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();

        surface_texture.present();

        self.window.request_redraw();
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        self.input.handle_window_event(&event);

        match event {
            WindowEvent::RedrawRequested => self.handle_redraw_requested(),
            WindowEvent::Resized(size) => self.resize(*size),
            _ => (),
        }
    }

    pub fn handle_device_event(&mut self, event: &DeviceEvent) {
        self.input.handle_device_event(&event);
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
                buffers: &[GPUVertex::desc()],
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

    fn generate_sphere(latitude_bands: u32, longitude_bands: u32) -> Vec<GPUVertex> {
        let mut vertices = Vec::new();

        for lat in 0..latitude_bands {
            let theta1 = (lat as f32) * std::f32::consts::PI / latitude_bands as f32;
            let theta2 = (lat as f32 + 1.0) * std::f32::consts::PI / latitude_bands as f32;

            for lon in 0..longitude_bands {
                let phi1 = (lon as f32) * 2.0 * std::f32::consts::PI / longitude_bands as f32;
                let phi2 = (lon as f32 + 1.0) * 2.0 * std::f32::consts::PI / longitude_bands as f32;

                let p1 = Self::spherical_to_cartesian(theta1, phi1);
                let p2 = Self::spherical_to_cartesian(theta2, phi1);
                let p3 = Self::spherical_to_cartesian(theta2, phi2);
                let p4 = Self::spherical_to_cartesian(theta1, phi2);

                vertices.push(GPUVertex {
                    position: p1,
                    normal: p1,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
                vertices.push(GPUVertex {
                    position: p2,
                    normal: p2,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
                vertices.push(GPUVertex {
                    position: p3,
                    normal: p3,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });

                vertices.push(GPUVertex {
                    position: p1,
                    normal: p1,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
                vertices.push(GPUVertex {
                    position: p3,
                    normal: p3,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
                vertices.push(GPUVertex {
                    position: p4,
                    normal: p4,
                    uv: [0.0, 0.0],
                    light: 1.0,
                });
            }
        }

        vertices
    }

    fn spherical_to_cartesian(theta: f32, phi: f32) -> [f32; 3] {
        let x = phi.sin() * theta.sin();
        let y = theta.cos();
        let z = phi.cos() * theta.sin();
        [x, y, z]
    }

    fn generate_rectangle() -> Vec<GPUVertex> {
        let normal = [0.0, 0.0, 1.0]; // Facing forward
        let light = 1.0;

        // Define UV coordinates for the rectangle
        let uvs = [
            [0.0, 0.0], // Bottom-left
            [1.0, 0.0], // Bottom-right
            [1.0, 1.0], // Top-right
            [0.0, 1.0], // Top-left
        ];

        // Define positions for two triangles forming the rectangle
        let positions = [
            // First triangle
            [-0.5, -0.5, 0.0], // Bottom-left
            [0.5, -0.5, 0.0],  // Bottom-right
            [0.5, 0.5, 0.0],   // Top-right
            // Second triangle
            [-0.5, -0.5, 0.0], // Bottom-left
            [0.5, 0.5, 0.0],   // Top-right
            [-0.5, 0.5, 0.0],  // Top-left
        ];

        // Map positions and UVs to GPUVertex instances
        positions
            .iter()
            .enumerate()
            .map(|(i, &pos)| GPUVertex {
                position: pos,
                normal,
                uv: uvs[i % 4],
                light,
            })
            .collect()
    }

    fn create_view_projection_matrix(
        &self,
        entity_view: &simulation::observation::view::EntityView,
    ) -> [[f32; 4]; 4] {
        let entity_position_interpolated = entity_view
            .position
            .lerp(entity_view.next_position, self.render_alpha);
        let entity_orientation_interpolated = entity_view
            .orientation
            .lerp(entity_view.next_orientation, self.render_alpha);

        let opengl_projection =
            Mat4::perspective_rh(FOV.to_radians(), WINDOW_ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);
        let projection = OPENGL_TO_WGPU_MATRIX * opengl_projection;

        let forward = entity_orientation_interpolated * Vec3::Z;
        let up = entity_orientation_interpolated * Vec3::Y;

        let eye = entity_position_interpolated + USER_VIEW_OFFSET * up;
        let target = eye + forward;

        let view = Mat4::look_at_rh(eye, target, up);

        let view_projection = projection * view;

        view_projection.to_cols_array_2d()
    }
}
