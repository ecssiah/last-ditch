//! User interaction

pub mod app;
pub mod camera;
pub mod constants;
pub mod debug_renderer;
pub mod gpu;
pub mod gui;
pub mod input;
pub mod object_renderer;
pub mod population_renderer;
pub mod world_renderer;

use crate::{
    interface::{
        camera::Camera, constants::*, debug_renderer::DebugRenderer, gpu::gpu_context::GPUContext,
        gui::GUI, input::Input, object_renderer::ObjectRenderer,
        population_renderer::PopulationRenderer, world_renderer::WorldRenderer,
    },
    simulation::{
        self,
        manager::{status::Status, viewer::View, Message, Viewer},
    },
};
use std::{collections::VecDeque, sync::Arc, time::Instant};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Fullscreen, WindowAttributes},
};

pub struct Interface<'window> {
    pub last_instant: Instant,
    pub message_tx: crossbeam::channel::Sender<Message>,
    pub input: Input,
    pub camera: Camera,
    pub gui: GUI,
    pub world_renderer: WorldRenderer,
    pub object_renderer: ObjectRenderer,
    pub population_renderer: PopulationRenderer,
    pub debug_renderer: DebugRenderer,
    pub gpu_context: GPUContext<'window>,
    pub view_output: triple_buffer::Output<View>,
}

impl<'window> Interface<'window> {
    pub fn new(
        message_tx: crossbeam::channel::Sender<Message>,
        view_output: triple_buffer::Output<View>,
        event_loop: &ActiveEventLoop,
    ) -> Self {
        let last_instant = Instant::now();

        let monitor = event_loop
            .primary_monitor()
            .expect("No primary monitor found");

        let window_title = format!(
            "{} {}",
            simulation::constants::PROJECT_TITLE,
            env!("CARGO_PKG_VERSION")
        );

        let window_attributes = if FULLSCREEN {
            WindowAttributes::default()
                .with_title(window_title)
                .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
        } else {
            WindowAttributes::default()
                .with_title(window_title)
                .with_inner_size(PhysicalSize::new(
                    constants::WINDOW_WIDTH,
                    constants::WINDOW_HEIGHT,
                ))
        };

        let window_arc = Arc::new(event_loop.create_window(window_attributes).unwrap());

        window_arc.set_cursor_visible(true);
        window_arc
            .set_cursor_grab(winit::window::CursorGrabMode::None)
            .expect("Failed to grab cursor");

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .expect("Failed to find GPU adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::TIMESTAMP_QUERY
                    | wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS,
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        ))
        .expect("Failed to create device");

        let size = window_arc.inner_size();

        let surface = instance.create_surface(window_arc.clone()).unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| *f == wgpu::TextureFormat::Bgra8UnormSrgb)
            .unwrap_or(surface_capabilities.formats[0]);

        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(surface_format),
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

        let egui_context = egui::Context::default();

        let egui_winit_state = egui_winit::State::new(
            egui_context.clone(),
            egui::ViewportId::ROOT,
            &window_arc,
            None,
            None,
            None,
        );

        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1, false);

        let gpu_context = GPUContext {
            window_arc,
            device,
            queue,
            size,
            surface,
            surface_config,
            texture_view_descriptor,
            egui_context,
            egui_winit_state,
            egui_renderer,
        };

        let input = Input::new();
        let camera = Camera::new(&gpu_context.device);
        let gui = GUI::new();
        let world_renderer = WorldRenderer::new(&gpu_context, &camera);
        let object_renderer = ObjectRenderer::new(&gpu_context, &camera);
        let population_renderer = PopulationRenderer::new(&gpu_context, &camera);
        let debug_renderer = DebugRenderer::new(&gpu_context, &camera);

        gpu_context.window_arc.request_redraw();

        Self {
            last_instant,
            message_tx,
            input,
            camera,
            gui,
            world_renderer,
            object_renderer,
            population_renderer,
            debug_renderer,
            gpu_context,
            view_output,
        }
    }

    pub fn handle_window_event(event: &WindowEvent, interface: &mut Option<Self>) {
        if let Some(interface) = interface.as_mut() {
            let _ = tracing::info_span!("window_event").entered();

            match event {
                WindowEvent::RedrawRequested => Self::render(
                    &interface.camera,
                    &mut interface.gpu_context,
                    &mut interface.gui,
                    &mut interface.world_renderer,
                    &mut interface.object_renderer,
                    &mut interface.population_renderer,
                    &mut interface.debug_renderer,
                ),
                WindowEvent::Resized(size) => {
                    Self::handle_resized(*size, &mut interface.gpu_context)
                }
                _ => {
                    if Input::handle_window_event(
                        event,
                        &mut interface.gui,
                        &mut interface.debug_renderer,
                        &mut interface.gpu_context,
                        &mut interface.input,
                    ) {
                        return;
                    };

                    GUI::handle_window_event(event, &mut interface.gpu_context);
                }
            }
        }
    }

    pub fn handle_device_event(event: &DeviceEvent, interface: &mut Option<Self>) {
        if let Some(interface) = interface.as_mut() {
            if Input::handle_device_event(event, &interface.gui, &mut interface.input) {
                return;
            }

            GUI::handle_device_event(event, &mut interface.gpu_context);
        }
    }

    fn render(
        camera: &Camera,
        gpu_context: &mut GPUContext,
        gui: &mut GUI,
        world_renderer: &mut WorldRenderer,
        object_renderer: &mut ObjectRenderer,
        population_renderer: &mut PopulationRenderer,
        debug_renderer: &mut DebugRenderer,
    ) {
        let _ = tracing::info_span!("redraw").entered();

        let mut encoder = gpu_context
            .device
            .create_command_encoder(&Default::default());

        let surface_texture = gpu_context
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let surface_texture_view = surface_texture
            .texture
            .create_view(&gpu_context.texture_view_descriptor);

        let depth_texture_view =
            Self::create_depth_texture(&gpu_context.device, &gpu_context.surface_config);

        WorldRenderer::render(
            &surface_texture_view,
            &depth_texture_view,
            &camera.uniform_bind_group,
            world_renderer,
            &mut encoder,
        );

        ObjectRenderer::render(
            &surface_texture_view,
            &depth_texture_view,
            gpu_context,
            &camera.uniform_bind_group,
            object_renderer,
            &mut encoder,
        );

        PopulationRenderer::render(
            &surface_texture_view,
            &depth_texture_view,
            gpu_context,
            &camera.uniform_bind_group,
            population_renderer,
            &mut encoder,
        );

        DebugRenderer::render(
            &surface_texture_view,
            &depth_texture_view,
            gpu_context,
            debug_renderer,
            &mut encoder,
        );

        GUI::render(
            &surface_texture_view,
            Arc::clone(&gpu_context.window_arc),
            &gpu_context.device,
            &gpu_context.queue,
            &gpu_context.egui_context,
            gui,
            &mut gpu_context.egui_winit_state,
            &mut gpu_context.egui_renderer,
            &mut encoder,
        );

        gpu_context.queue.submit([encoder.finish()]);
        gpu_context.window_arc.pre_present_notify();

        surface_texture.present();
    }

    fn handle_resized(size: winit::dpi::PhysicalSize<u32>, gpu_context: &mut GPUContext) {
        gpu_context.size = size;

        gpu_context
            .surface
            .configure(&gpu_context.device, &gpu_context.surface_config);
    }

    fn update(event_loop: &ActiveEventLoop, interface: &mut Option<Self>) {
        let _ = tracing::info_span!("interface update").entered();

        if let Some(interface) = interface.as_mut() {
            let instant = Instant::now();
            let next_instant = interface.last_instant + INTERFACE_FRAME_DURATION;
            interface.last_instant = instant;

            let view = Viewer::get_view(&mut interface.view_output);

            Self::send_message_deque(
                &mut interface.gui,
                &mut interface.input,
                &interface.message_tx,
            );

            Self::apply_view(
                event_loop,
                view,
                &interface.gpu_context,
                &mut interface.camera,
                &mut interface.gui,
                &mut interface.world_renderer,
                &mut interface.object_renderer,
                &mut interface.population_renderer,
                &mut interface.debug_renderer,
            );

            let instant = Instant::now();

            if next_instant > instant {
                event_loop.set_control_flow(ControlFlow::WaitUntil(next_instant));
            };

            interface.gpu_context.window_arc.request_redraw();
        }
    }

    fn apply_view(
        event_loop: &ActiveEventLoop,
        view: &View,
        gpu_context: &GPUContext,
        camera: &mut Camera,
        gui: &mut GUI,
        world_renderer: &mut WorldRenderer,
        object_renderer: &mut ObjectRenderer,
        population_renderer: &mut PopulationRenderer,
        debug_renderer: &mut DebugRenderer,
    ) {
        if view.manager_view.status == Status::Done {
            event_loop.exit();
            return;
        }

        GUI::apply_view(view, gui);
        Camera::apply_view(view, camera);

        gpu_context.queue.write_buffer(
            &camera.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera.uniform_data]),
        );

        WorldRenderer::apply_world_view(
            &gpu_context.device,
            camera,
            &view.world_view,
            &mut world_renderer.sector_mesh_cache,
            &mut world_renderer.gpu_mesh_cache,
            &mut world_renderer.active_sector_id_set,
            &mut world_renderer.active_gpu_mesh_vec,
            &mut object_renderer.object_instance_data_group_vec,
        );

        PopulationRenderer::apply_population_view(
            &view.population_view,
            &mut population_renderer.person_instance_data_group_vec,
        );

        DebugRenderer::apply_debug_view(view, debug_renderer);
    }

    fn send_message_deque(
        gui: &mut GUI,
        input: &mut Input,
        message_tx: &crossbeam::channel::Sender<Message>,
    ) {
        let mut message_deque = VecDeque::new();

        let gui_message_deque = GUI::get_message_deque(&mut gui.message_deque);

        let input_message_deque = Input::get_message_deque(
            &input.key_inputs,
            &mut input.mouse_inputs,
            &mut input.message_deque,
        );

        message_deque.extend(gui_message_deque);
        message_deque.extend(input_message_deque);

        for message in message_deque {
            match message_tx.send(message) {
                Ok(()) => (),
                Err(_) => tracing::error!("Message Send Failed"),
            }
        }
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::TextureView {
        let depth_texture_descriptor = wgpu::TextureDescriptor {
            label: None,
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
        };

        let depth_texture = device.create_texture(&depth_texture_descriptor);

        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
