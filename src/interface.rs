//! The Interface module manages user interaction with the Simulation. This includes
//! both presentation and input management.

pub mod camera;
pub mod consts;
pub mod input;
pub mod render;

use crate::{
    interface::{
        camera::Camera,
        consts::*,
        input::Input,
        render::{
            gpu_chunk::GPUChunk, gpu_entity::GPUEntity, ChunkRender, EntityRender, GPUMesh,
            GPUVertex, Textures,
        },
    },
    simulation::{self},
};
use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    window::Window,
};

pub struct Interface {
    delta_time: Duration,
    render_instant: Instant,
    alpha: f32,
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
    textures: Textures,
    camera: Camera,
    chunk_renderer: ChunkRender,
    entity_renderer: EntityRender,
}

impl Interface {
    pub fn new(
        action_tx: UnboundedSender<simulation::dispatch::Action>,
        observation_lock: Arc<RwLock<simulation::observation::Observation>>,
        window: Arc<Window>,
        instance: wgpu::Instance,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> Self {
        let delta_time = Duration::ZERO;
        let render_instant = Instant::now();
        let alpha = 0.0;

        let input = Input::new(action_tx.clone());

        let size = window.inner_size();

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];

        let surface_texture_view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(surface_format.add_srgb_suffix()),
            ..Default::default()
        };

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

        let mut textures = Textures::new(&device);

        pollster::block_on(textures.load_texture_atlas(
            &device,
            &queue,
            &"assets/textures/atlas.png".to_string(),
            "atlas",
        ));

        textures.generate_texture_sampler_bind_group(&device);

        let camera = Camera::new(&device);

        let chunk_renderer = ChunkRender::new(
            &device,
            &surface_format,
            &camera.uniform_bind_group_layout,
            &textures.texture_sampler_bind_group_layout,
        );

        let entity_renderer =
            EntityRender::new(&device, &surface_format, &camera.uniform_bind_group_layout);

        let interface = Self {
            delta_time,
            render_instant,
            alpha,
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
            camera,
            textures,
            chunk_renderer,
            entity_renderer,
        };

        log::info!("Interface Initialized");

        interface
    }

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
            self.update_alpha(&view.time_view);

            self.update_entity_view(&entity_view);

            self.update_population_view(&view.population_view);
            self.update_world_view(&view.world_view);
        }
    }

    fn update_alpha(&mut self, time_view: &simulation::observation::view::TimeView) {
        let now = Instant::now();
        self.delta_time = now - self.render_instant;
        self.render_instant = now;

        let alpha = (now - time_view.simulation_instant).as_secs_f32();
        self.alpha = alpha.clamp(0.0, 1.0);
    }

    fn update_entity_view(&mut self, entity_view: &simulation::observation::view::EntityView) {
        self.camera.update(&self.queue, self.alpha, entity_view);
    }

    fn update_population_view(
        &mut self,
        population_view: &simulation::observation::view::PopulationView,
    ) {
        self.entity_renderer.gpu_entities = population_view
            .entity_views
            .iter()
            .map(|entity_view| GPUEntity {
                position: entity_view.1.position.to_array(),
                height: 1.8,
            })
            .collect();

        let required_size = (population_view.entity_views.len() * std::mem::size_of::<GPUEntity>())
            as wgpu::BufferAddress;

        if self.entity_renderer.instance_buffer.size() < required_size {
            self.entity_renderer.instance_buffer =
                self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Entity Instance Buffer"),
                    size: required_size,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        }

        self.queue.write_buffer(
            &self.entity_renderer.instance_buffer,
            0,
            bytemuck::cast_slice(&self.entity_renderer.gpu_entities),
        );
    }

    fn update_world_view(&mut self, world_view: &simulation::observation::view::WorldView) {
        self.chunk_renderer.gpu_chunks.clear();

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

            self.chunk_renderer.gpu_chunks.push(chunk);
        }
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

        let depth_texture_view = Textures::create_depth_texture(&self.device, &self.surface_config);

        if let Some(ref texture_sampler_bind_group) = self.textures.texture_sampler_bind_group {
            self.chunk_renderer.render(
                &mut encoder,
                &texture_view,
                &depth_texture_view,
                &self.camera.view_projection_bind_group,
                texture_sampler_bind_group,
            );
        }

        self.entity_renderer.render(
            &mut encoder,
            &texture_view,
            &depth_texture_view,
            &self.camera.view_projection_bind_group,
        );

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
}
