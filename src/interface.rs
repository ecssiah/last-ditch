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
pub mod texture;
pub mod world_renderer;

use crate::{
    interface::{
        camera::Camera, constants::*, debug_renderer::DebugRenderer, gpu::gpu_context::GPUContext,
        gui::GUI, input::Input, population_renderer::PopulationRenderer,
        texture::texture_manager::TextureManager, world_renderer::WorldRenderer,
    },
    simulation::{
        self,
        manager::{status::Status, viewer::view::View, Message, Viewer},
    },
};
use std::{collections::VecDeque, sync::Arc, time::Instant};
use tracing::instrument;
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
    pub texture_manager: TextureManager,
    pub gui: GUI,
    pub world_renderer: WorldRenderer,
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

        let texture_manager = TextureManager::new(&gpu_context);

        let world_renderer = WorldRenderer::new(&gpu_context, &camera, &texture_manager);
        let population_renderer = PopulationRenderer::new(&gpu_context, &camera, &texture_manager);
        let debug_renderer = DebugRenderer::new(&gpu_context, &camera);
        
        let gui = GUI::new();

        gpu_context.window_arc.request_redraw();

        Self {
            last_instant,
            message_tx,
            input,
            camera,
            texture_manager,
            world_renderer,
            population_renderer,
            debug_renderer,
            gui,
            gpu_context,
            view_output,
        }
    }

    #[instrument(skip_all)]
    pub fn handle_window_event(event: &WindowEvent, interface: &mut Option<Self>) {
        if let Some(interface) = interface.as_mut() {
            match event {
                WindowEvent::RedrawRequested => Self::render(
                    &interface.camera,
                    &interface.texture_manager,
                    &mut interface.gpu_context,
                    &mut interface.world_renderer,
                    &mut interface.population_renderer,
                    &mut interface.debug_renderer,
                    &mut interface.gui,
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

    #[instrument(skip_all)]
    pub fn handle_device_event(event: &DeviceEvent, interface: &mut Option<Self>) {
        if let Some(interface) = interface.as_mut() {
            if Input::handle_device_event(event, &interface.gui, &mut interface.input) {
                return;
            }

            GUI::handle_device_event(event, &mut interface.gpu_context);
        }
    }

    #[instrument(skip_all)]
    fn render(
        camera: &Camera,
        texture_manager: &TextureManager,
        gpu_context: &mut GPUContext,
        world_renderer: &mut WorldRenderer,
        population_renderer: &mut PopulationRenderer,
        debug_renderer: &mut DebugRenderer,
        gui: &mut GUI,
    ) {
        let mut encoder = gpu_context
            .device
            .create_command_encoder(&Default::default());

        let surface_texture = TextureManager::get_surface_texture(gpu_context);

        let surface_texture_view = surface_texture
            .texture
            .create_view(&gpu_context.texture_view_descriptor);

        WorldRenderer::render(
            &surface_texture_view,
            &texture_manager.depth_texture_view,
            &camera.uniform_bind_group,
            world_renderer,
            &mut encoder,
        );

        PopulationRenderer::render(
            &surface_texture_view,
            &texture_manager.depth_texture_view,
            gpu_context,
            &camera.uniform_bind_group,
            population_renderer,
            &mut encoder,
        );

        DebugRenderer::render(
            &surface_texture_view,
            &texture_manager.depth_texture_view,
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

    #[instrument(skip_all)]
    fn update(event_loop: &ActiveEventLoop, interface: &mut Option<Self>) {
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
                &mut interface.world_renderer,
                &mut interface.population_renderer,
                &mut interface.debug_renderer,
                &mut interface.gui,
            );

            let instant = Instant::now();

            if next_instant > instant {
                event_loop.set_control_flow(ControlFlow::WaitUntil(next_instant));
            };

            interface.gpu_context.window_arc.request_redraw();
        }
    }

    #[instrument(skip_all)]
    fn apply_view(
        event_loop: &ActiveEventLoop,
        view: &View,
        gpu_context: &GPUContext,
        camera: &mut Camera,
        world_renderer: &mut WorldRenderer,
        population_renderer: &mut PopulationRenderer,
        debug_renderer: &mut DebugRenderer,
        gui: &mut GUI,
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

        WorldRenderer::apply_world_view(gpu_context, camera, &view.world_view, world_renderer);

        PopulationRenderer::apply_population_view(
            gpu_context,
            &view.population_view,
            population_renderer,
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
}
