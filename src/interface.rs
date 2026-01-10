//! User interaction

pub mod app;
pub mod asset_manager;
pub mod camera;
pub mod constants;
pub mod gpu;
pub mod input;
pub mod interface_mode;
pub mod renderer;

use crate::{
    interface::{
        asset_manager::{asset_status::AssetStatus, AssetManager},
        camera::Camera,
        constants::*,
        gpu::gpu_context::GPUContext,
        input::Input,
        interface_mode::InterfaceMode,
        renderer::{
            block_renderer::BlockRenderer, debug_renderer::DebugRenderer,
            overlay_renderer::OverlayRenderer, person_renderer::PersonRenderer,
            sector_renderer::SectorRenderer, Renderer,
        },
    },
    simulation::{
        self,
        supervisor::{supervisor_status::SupervisorStatus, viewer::view::View, Message, Viewer},
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
    pub interface_mode: InterfaceMode,
    pub last_instant: Instant,
    pub message_tx: crossbeam::channel::Sender<Message>,
    pub asset_manager: AssetManager,
    pub input: Input,
    pub camera: Camera,
    pub renderer: Renderer,
    pub gpu_context: GPUContext<'window>,
    pub view_output: triple_buffer::Output<View>,
}

impl<'window> Interface<'window> {
    pub fn new(
        message_tx: crossbeam::channel::Sender<Message>,
        view_output: triple_buffer::Output<View>,
        event_loop: &ActiveEventLoop,
    ) -> Self {
        let interface_mode = InterfaceMode::Setup;

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

        let instance = wgpu::Instance::new(&Default::default());

        let adapter = pollster::block_on(instance.request_adapter(&Default::default()))
            .expect("Failed to find GPU adapter");

        let required_features = wgpu::Features::TIMESTAMP_QUERY
            | wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS
            | wgpu::Features::TEXTURE_BINDING_ARRAY;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features,
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

        let gpu_context = GPUContext {
            window_arc,
            device,
            queue,
            size,
            surface,
            surface_config,
            texture_view_descriptor,
        };

        let input = Input::new();
        let camera = Camera::new(&gpu_context.device);
        let asset_manager = AssetManager::new(&gpu_context);
        let renderer = Renderer::new(&gpu_context, &surface_format, &camera);

        gpu_context.window_arc.request_redraw();

        Self {
            interface_mode,
            last_instant,
            message_tx,
            asset_manager,
            input,
            camera,
            renderer,
            gpu_context,
            view_output,
        }
    }

    #[instrument(skip_all)]
    fn update(event_loop: &ActiveEventLoop, interface: &mut Self) {
        let instant = Instant::now();

        let next_instant = interface.last_instant + INTERFACE_FRAME_DURATION;
        interface.last_instant = instant;

        let view = Viewer::get_view(&mut interface.view_output);

        if view.supervisor_view.supervisor_status == SupervisorStatus::Done {
            event_loop.exit();

            return;
        }

        match interface.interface_mode {
            InterfaceMode::Setup => Self::update_setup_mode(
                &mut interface.gpu_context,
                &mut interface.interface_mode,
                &mut interface.asset_manager,
                &mut interface.renderer,
            ),
            InterfaceMode::Menu => Self::update_menu_mode(),
            InterfaceMode::Run => Self::update_run_mode(
                view,
                &interface.gpu_context,
                &interface.asset_manager,
                &mut interface.camera,
                &mut interface.renderer,
            ),
        }

        Self::send_message_deque(
            &mut interface.input,
            &mut interface.renderer.overlay_renderer,
            &interface.message_tx,
        );

        let instant = Instant::now();

        if next_instant > instant {
            event_loop.set_control_flow(ControlFlow::WaitUntil(next_instant));
        };

        interface.gpu_context.window_arc.request_redraw();
    }

    fn send_message_deque(
        input: &mut Input,
        overlay_renderer: &mut OverlayRenderer,
        message_tx: &crossbeam::channel::Sender<Message>,
    ) {
        let mut message_deque = VecDeque::new();

        let overlay_message_deque =
            OverlayRenderer::get_message_deque(&mut overlay_renderer.message_deque);

        message_deque.extend(overlay_message_deque);

        let input_message_deque = Input::get_message_deque(
            &input.key_inputs,
            &mut input.mouse_inputs,
            &mut input.message_deque,
        );

        message_deque.extend(input_message_deque);

        for message in message_deque {
            match message_tx.send(message) {
                Ok(()) => (),
                Err(_) => tracing::error!("Message Send Failed"),
            }
        }
    }

    #[instrument(skip_all)]
    fn update_setup_mode(
        gpu_context: &mut GPUContext,
        interface_mode: &mut InterfaceMode,
        asset_manager: &mut AssetManager,
        renderer: &mut Renderer,
    ) {
        match &asset_manager.asset_status {
            AssetStatus::Startup => {
                // Load Startup resources for splash screen, etc.

                asset_manager.asset_status = AssetStatus::InitTextures;
            }
            AssetStatus::InitTextures => {
                renderer.overlay_renderer.content.setup_content.progress = 0.0;
                renderer
                    .overlay_renderer
                    .content
                    .setup_content
                    .loading_string = String::from("Initializing textures...");

                AssetManager::init_texture_loading(asset_manager);
            }
            AssetStatus::LoadingTextures => {
                renderer.overlay_renderer.content.setup_content.progress = 0.2;
                renderer
                    .overlay_renderer
                    .content
                    .setup_content
                    .loading_string = String::from("Loading textures...");

                AssetManager::update_texture_loading(gpu_context, asset_manager);
            }
            AssetStatus::InitModels => {
                renderer.overlay_renderer.content.setup_content.progress = 0.4;
                renderer
                    .overlay_renderer
                    .content
                    .setup_content
                    .loading_string = String::from("Initializing models...");

                AssetManager::init_model_loading(asset_manager);
            }
            AssetStatus::LoadingModels => {
                renderer.overlay_renderer.content.setup_content.progress = 0.6;
                renderer
                    .overlay_renderer
                    .content
                    .setup_content
                    .loading_string = String::from("Loading models...");

                AssetManager::update_model_loading(gpu_context, asset_manager);
            }
            AssetStatus::Complete => {
                renderer.overlay_renderer.content.setup_content.progress = 0.8;
                renderer
                    .overlay_renderer
                    .content
                    .setup_content
                    .loading_string = String::from("Setting up bind groups...");

                Renderer::setup_bind_groups(gpu_context, asset_manager, renderer);

                renderer.overlay_renderer.content.setup_content.progress = 1.0;
                renderer
                    .overlay_renderer
                    .content
                    .setup_content
                    .loading_string = String::from("Complete!");

                *interface_mode = InterfaceMode::Menu;
            }
        }
    }

    #[instrument(skip_all)]
    fn update_menu_mode() {}

    #[instrument(skip_all)]
    fn update_run_mode(
        view: &View,
        gpu_context: &GPUContext,
        asset_manager: &AssetManager,
        camera: &mut Camera,
        renderer: &mut Renderer,
    ) {
        Camera::apply_view(gpu_context, view, camera);

        BlockRenderer::apply_world_view(
            gpu_context,
            asset_manager,
            &view.world_view,
            &mut renderer.block_renderer,
        );

        SectorRenderer::apply_world_view(
            gpu_context,
            asset_manager,
            &view.world_view,
            &mut renderer.sector_renderer,
        );

        PersonRenderer::apply_population_view(
            gpu_context,
            asset_manager,
            &view.population_view,
            &mut renderer.person_renderer,
        );

        OverlayRenderer::apply_view_run_mode(view, &mut renderer.overlay_renderer);

        DebugRenderer::apply_debug_view(gpu_context, view, &mut renderer.debug_renderer);
    }

    #[instrument(skip_all)]
    pub fn handle_window_event(event: &WindowEvent, interface: &mut Self) {
        match event {
            WindowEvent::RedrawRequested => Self::render(
                &interface.interface_mode,
                &interface.camera,
                &interface.asset_manager,
                &mut interface.gpu_context,
                &mut interface.renderer,
            ),
            WindowEvent::Resized(size) => Self::handle_resized(*size, &mut interface.gpu_context),
            _ => {
                if Input::handle_window_event(
                    event,
                    &mut interface.renderer.debug_renderer,
                    &mut interface.renderer.overlay_renderer,
                    &mut interface.gpu_context,
                    &mut interface.input,
                ) {
                    return;
                };

                OverlayRenderer::handle_window_event(
                    event,
                    &interface.gpu_context,
                    &mut interface.renderer.overlay_renderer,
                );
            }
        }
    }

    #[instrument(skip_all)]
    pub fn handle_device_event(event: &DeviceEvent, interface: &mut Self) {
        if Input::handle_device_event(
            event,
            &interface.renderer.overlay_renderer,
            &mut interface.input,
        ) {
            return;
        }

        OverlayRenderer::handle_device_event(event, &mut interface.renderer.overlay_renderer);
    }

    #[instrument(skip_all)]
    fn render(
        interface_mode: &InterfaceMode,
        camera: &Camera,
        asset_manager: &AssetManager,
        gpu_context: &mut GPUContext,
        renderer: &mut Renderer,
    ) {
        let mut encoder = gpu_context
            .device
            .create_command_encoder(&Default::default());

        let surface_texture = AssetManager::get_surface_texture(gpu_context);

        let surface_texture_view = surface_texture
            .texture
            .create_view(&gpu_context.texture_view_descriptor);

        Renderer::render(
            interface_mode,
            &surface_texture_view,
            camera,
            gpu_context,
            asset_manager,
            renderer,
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
}
