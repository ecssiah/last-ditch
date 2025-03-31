//! The Interface module manages user interaction with the Simulation. This includes
//! both presentation and input management.

pub mod chunk;
pub mod consts;
pub mod input;

use crate::{
    include_shader_src,
    interface::{self, consts::*, input::Input},
    simulation::{
        action::{Action, AgentAction}, agent::Agent, id::chunk_id::ChunkID, observation::Observation, CHUNK_VOLUME
    },
};
use glam::{Mat4, Vec3};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::UnboundedSender;
use wgpu::{Adapter, Device, Instance, Queue};
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
    action_tx: UnboundedSender<Action>,
    observation: Arc<RwLock<Observation>>,
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
    chunk_pipeline: wgpu::RenderPipeline,
    mesh_cache: interface::chunk::MeshCache,
}

impl Interface {
    pub fn new(
        action_tx: UnboundedSender<Action>,
        observation: Arc<RwLock<Observation>>,
        window: Arc<Window>,
        instance: Instance,
        adapter: Adapter,
        device: Device,
        queue: Queue,
    ) -> Self {
        let input = Input::new(action_tx.clone());

        window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect("Failed to grab cursor");
        window.set_cursor_visible(false);

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

        let mesh_cache = interface::chunk::MeshCache::new();

        let interface = Self {
            action_tx,
            observation,
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
            chunk_pipeline,
            mesh_cache,
        };

        log::info!("Interface Initialized");

        interface
    }

    fn check_active(&mut self, event_loop: &ActiveEventLoop) {
        let world = self.state.world.read().unwrap();

        if world.active == false {
            event_loop.exit();
        }
    }

    fn send_movement_actions(&mut self) {
        let movement_actions = self.input.get_movement_actions();

        self.action_tx
            .send(Action::Agent(AgentAction::Movement(movement_actions)))
            .unwrap();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn update(&mut self, event_loop: &ActiveEventLoop) {
        self.check_active(event_loop);

        self.send_movement_actions();
    }

    fn update_view_projection(&mut self) {
        let view_projection_matrix =
            Self::create_view_projection_matrix(Arc::clone(&self.state.agent));

        self.queue.write_buffer(
            &self.view_projection_buffer,
            0,
            bytemuck::cast_slice(&view_projection_matrix),
        );
    }

    pub fn render(&mut self) {
        self.update_view_projection();

        let last_update = self.state.world.read().unwrap().last_update;

        if last_update > self.last_update {
            self.update_meshes();

            self.last_update = last_update;
        }

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
                    r: CLEAR_COLOR[0],
                    g: CLEAR_COLOR[1],
                    b: CLEAR_COLOR[2],
                    a: CLEAR_COLOR[3],
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

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Chunk Render Pass"),
            color_attachments: &[chunk_render_pass_color_attachment],
            depth_stencil_attachment: chunk_depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.chunk_pipeline);
        render_pass.set_bind_group(0, &self.view_projection_bind_group, &[]);

        for mesh in self.mesh_cache.values() {
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

        self.window.request_redraw();
    }

    fn update_meshes(&mut self) {
        for chunk_id in 0..CHUNK_VOLUME {
            let chunk_id = ChunkID(chunk_id);
            let chunk = self.state.chunks[chunk_id].read().unwrap();

            if self.mesh_cache.needs_update(chunk_id, chunk.last_update) {
                let vertices: Vec<chunk::Vertex> = chunk
                    .mesh
                    .vertices
                    .iter()
                    .map(|vertex| chunk::Vertex {
                        position: vertex.position.to_array(),
                        normal: vertex.normal.to_array(),
                        color: vertex.color.to_array(),
                        ao: vertex.ao,
                    })
                    .collect();

                let indices: Vec<u32> = chunk.mesh.indices.clone();

                self.mesh_cache.upload_mesh(
                    &self.device,
                    chunk_id,
                    vertices,
                    indices,
                    chunk.last_update,
                );
            }
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        self.input.handle_window_event(&event);

        match event {
            WindowEvent::RedrawRequested => self.render(),
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
                buffers: &[interface::chunk::Vertex::desc()],
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

    fn create_view_projection_matrix(agent: Arc<RwLock<Agent>>) -> [[f32; 4]; 4] {
        let agent = agent.read().unwrap();

        let opengl_projection =
            Mat4::perspective_rh(FOV.to_radians(), ASPECT_RATIO, NEAR_PLANE, FAR_PLANE);
        let projection = OPENGL_TO_WGPU_MATRIX * opengl_projection;

        let forward = agent.orientation * Vec3::Z;
        let up = agent.orientation * Vec3::Y;

        let eye = agent.position;
        let target = eye + forward;

        let view = Mat4::look_at_rh(eye, target, up);

        let view_projection = projection * view;

        view_projection.to_cols_array_2d()
    }
}
