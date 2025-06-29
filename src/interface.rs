//! Interactions with the User

pub mod camera;
pub mod consts;
pub mod dispatch;
pub mod gpu_context;
pub mod hud;
pub mod input;
pub mod render;

use crate::{
    interface::{
        camera::Camera, consts::*, dispatch::Dispatch, gpu_context::GPUContext, hud::HUD,
        input::Input, render::Render,
    },
    simulation::{self},
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Fullscreen, WindowAttributes},
};

pub struct Interface<'window> {
    dt: Duration,
    instant: Instant,
    last_instant: Instant,
    alpha: f32,
    gpu_context: GPUContext<'window>,
    observation_arc: Arc<simulation::observation::Observation>,
    dispatch: Dispatch,
    input: Input,
    camera: Camera,
    render: Render,
    hud: HUD,
}

impl Interface<'_> {
    pub fn new(
        event_loop: &ActiveEventLoop,
        action_tx: UnboundedSender<simulation::state::receiver::action::Action>,
        observation_arc: Arc<simulation::observation::Observation>,
    ) -> Self {
        let dt = Duration::ZERO;
        let instant = Instant::now();
        let last_instant = Instant::now();
        let alpha = 0.0;

        let monitor = event_loop
            .primary_monitor()
            .expect("No primary monitor found");

        let window_title = format!(
            "{} {}",
            simulation::consts::PROJECT_TITLE,
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
                    consts::WINDOW_WIDTH,
                    consts::WINDOW_HEIGHT,
                ))
        };

        let window_arc = Arc::new(event_loop.create_window(window_attributes).unwrap());

        window_arc.set_cursor_visible(false);
        window_arc
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect("Failed to grab cursor");

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .expect("Failed to find GPU adapter");

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .expect("Failed to create device");

        let size = window_arc.inner_size();

        let surface = instance.create_surface(window_arc.clone()).unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| *f == wgpu::TextureFormat::Bgra8Unorm)
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

        let dispatch = Dispatch::new(action_tx);
        let input = Input::new();
        let camera = Camera::new(&gpu_context.device);

        let render = Render::new(
            &gpu_context.device,
            &gpu_context.queue,
            &surface_format,
            &camera,
        );

        let hud = HUD::new();

        gpu_context.window_arc.request_redraw();

        Self {
            dt,
            instant,
            last_instant,
            alpha,
            observation_arc,
            gpu_context,
            input,
            camera,
            dispatch,
            render,
            hud,
        }
    }

    pub fn handle_about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let instant = Instant::now();
        let next_instant = self.last_instant + INTERFACE_FRAME_DURATION;
        self.last_instant = instant;

        self.update(event_loop);

        let instant = Instant::now();

        if next_instant > instant {
            event_loop.set_control_flow(ControlFlow::WaitUntil(next_instant));
        };

        self.gpu_context.window_arc.request_redraw();
    }

    pub fn handle_device_event(&mut self, event: &DeviceEvent) {
        let hud_handled = self.hud.handle_device_event(event, &mut self.gpu_context);

        if !hud_handled {
            self.input.handle_device_event(event);
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => self.handle_redraw_requested(),
            WindowEvent::Resized(size) => self.handle_resized(*size),
            _ => {
                let hud_handled = self.hud.handle_window_event(event, &mut self.gpu_context);

                if !hud_handled {
                    self.input.handle_window_event(event);
                }
            }
        }
    }

    fn handle_redraw_requested(&mut self) {
        let mut encoder = self
            .gpu_context
            .device
            .create_command_encoder(&Default::default());

        let surface_texture = self
            .gpu_context
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let texture_view = surface_texture
            .texture
            .create_view(&self.gpu_context.texture_view_descriptor);

        self.render
            .update(&mut encoder, &self.gpu_context, &texture_view, &self.camera);

        self.hud
            .update(&mut encoder, &mut self.gpu_context, &texture_view);

        self.gpu_context.queue.submit([encoder.finish()]);
        self.gpu_context.window_arc.pre_present_notify();

        surface_texture.present();
    }

    fn handle_resized(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.gpu_context.size = new_size;

        self.gpu_context
            .surface
            .configure(&self.gpu_context.device, &self.gpu_context.surface_config);
    }

    fn update(&mut self, event_loop: &ActiveEventLoop) {
        if !self.dispatch_actions() {
            event_loop.exit();
        } else {
            self.apply_view(event_loop);
        }
    }

    fn apply_view(&mut self, event_loop: &ActiveEventLoop) {
        let view = self.observation_arc.get_view();

        match view.admin_view.mode {
            simulation::state::admin::Mode::Menu => self.apply_menu_view(&view),
            simulation::state::admin::Mode::Load => self.apply_load_view(&view),
            simulation::state::admin::Mode::Simulate => self.apply_simulate_view(&view),
            simulation::state::admin::Mode::Shutdown => self.apply_shutdown_view(&view, event_loop),
        }
    }

    fn apply_menu_view(&mut self, view: &simulation::observation::view::View) {
        self.gpu_context.window_arc.set_cursor_visible(true);
        self.gpu_context
            .window_arc
            .set_cursor_grab(winit::window::CursorGrabMode::None)
            .expect("Failed to grab cursor");

        self.hud.prepare_menu(view);
    }

    fn apply_load_view(&mut self, view: &simulation::observation::view::View) {
        self.hud.prepare_load(view);
    }

    fn apply_simulate_view(&mut self, view: &simulation::observation::view::View) {
        self.gpu_context.window_arc.set_cursor_visible(false);
        self.gpu_context
            .window_arc
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect("Failed to grab cursor");

        self.hud.prepare_simulate(view);

        self.dt = self.instant.elapsed();
        self.instant = Instant::now();

        let now = self.instant;
        let current = view.time_view.instant.current;
        let next = view.time_view.instant.next;

        let total_duration = next.duration_since(current).as_secs_f32();
        let elapsed_since_current = now.duration_since(current).as_secs_f32();

        self.alpha = (elapsed_since_current / total_duration).clamp(0.0, 1.0);

        self.camera.update(
            &self.gpu_context.queue,
            self.alpha,
            &view.population_view.judge_view,
        );

        self.render.prepare_agent_view_map(
            &self.gpu_context.device,
            &self.gpu_context.queue,
            &view.population_view.agent_view_map,
        );

        self.render
            .prepare_world_view(&self.gpu_context.device, &view.world_view);
    }

    fn apply_shutdown_view(
        &mut self,
        view: &simulation::observation::view::View,
        event_loop: &ActiveEventLoop,
    ) {
        self.hud.prepare_shutdown(view);

        event_loop.exit();
    }

    fn dispatch_actions(&mut self) -> bool {
        let input_actions = self.input.get_actions();
        let hud_actions = self.hud.get_actions();

        let action_vec: Vec<_> = input_actions.iter().chain(&hud_actions).cloned().collect();

        for action in action_vec {
            match self.dispatch.send(action) {
                Ok(()) => (),
                Err(_) => {
                    log::error!("Send Failed: {:?}", action);

                    return false;
                }
            }
        }

        true
    }
}
