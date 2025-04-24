//! The Interface module manages user interaction with the Simulation. This includes
//! both presentation and input management.

pub mod camera;
pub mod consts;
pub mod hud;
pub mod input;
pub mod render;

use crate::{
    interface::{camera::Camera, consts::*, hud::HUD, input::Input, render::Render},
    simulation::{self},
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Fullscreen, Window, WindowAttributes},
};

pub struct Interface {
    dt: Duration,
    instant: Instant,
    last_frame_instant: Option<Instant>,
    alpha: f32,
    action_tx: Arc<UnboundedSender<simulation::dispatch::Action>>,
    window: Arc<Window>,
    device: wgpu::Device,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    surface_texture_view_descriptor: wgpu::TextureViewDescriptor<'static>,
    observation: Arc<simulation::observation::Observation>,
    input: Input,
    queue: wgpu::Queue,
    render: Render,
    hud: HUD,
    camera: Camera,
}

impl Interface {
    pub fn new(
        event_loop: &ActiveEventLoop,
        action_tx: Arc<UnboundedSender<simulation::dispatch::Action>>,
        observation: Arc<simulation::observation::Observation>,
    ) -> Self {
        let dt = Duration::ZERO;
        let instant = Instant::now();
        let last_frame_instant = None;
        let alpha = 0.0;

        let input = Input::new(action_tx.clone());

        let monitor = event_loop
            .primary_monitor()
            .expect("No primary monitor found");

        let window_attributes = if FULLSCREEN {
            WindowAttributes::default()
                .with_title(consts::WINDOW_TITLE)
                .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
        } else {
            WindowAttributes::default()
                .with_title(consts::WINDOW_TITLE)
                .with_inner_size(PhysicalSize::new(
                    consts::WINDOW_WIDTH,
                    consts::WINDOW_HEIGHT,
                ))
        };

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        window.set_cursor_visible(false);
        window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect("Failed to grab cursor");

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .expect("Failed to find GPU adapter");

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .expect("Failed to create device");

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
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

        let camera = Camera::new(&device);

        let render = Render::new(
            &device,
            &queue,
            window.clone(),
            &instance,
            &adapter,
            &surface_format,
            &camera,
        );

        let hud = HUD::new(&device, window.clone(), surface_format);

        let interface = Self {
            dt,
            instant,
            last_frame_instant,
            alpha,
            action_tx,
            observation,
            window: window.clone(),
            device,
            size,
            surface,
            surface_config,
            surface_texture_view_descriptor,
            input,
            queue,
            camera,
            render,
            hud,
        };

        window.request_redraw();

        log::info!("Interface Initialized");

        interface
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        self.input.handle_window_event(&event);

        match event {
            WindowEvent::RedrawRequested => self.handle_redraw_requested(),
            WindowEvent::Resized(size) => self.handle_resized(*size),
            _ => (),
        }
    }

    pub fn handle_device_event(&mut self, event: &DeviceEvent) {
        self.input.handle_device_event(&event);
    }

    pub fn handle_about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();

        let next_frame_time = match self.last_frame_instant {
            Some(last) => last + FRAME_DURATION,
            None => now + FRAME_DURATION,
        };

        self.last_frame_instant = Some(now);

        let view = self.observation.get_view();

        self.apply_view(&view, event_loop);
        self.send_movement_actions();

        let now = Instant::now();

        let delay = if next_frame_time > now {
            next_frame_time - now
        } else {
            Duration::ZERO
        };

        event_loop.set_control_flow(ControlFlow::WaitUntil(now + delay));
    }

    fn handle_redraw_requested(&mut self) {
        let mut encoder = self.device.create_command_encoder(&Default::default());

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let texture_view = surface_texture
            .texture
            .create_view(&self.surface_texture_view_descriptor);

        self.render.update(
            &mut encoder,
            &self.device,
            &self.surface_config,
            &texture_view,
            &self.camera,
        );

        self.hud.update(
            &mut encoder,
            &self.window,
            &self.device,
            &self.queue,
            &texture_view,
        );

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();

        surface_texture.present();

        self.window.request_redraw();
    }

    fn handle_resized(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.surface.configure(&self.device, &self.surface_config);
    }

    fn apply_view(
        &mut self,
        view: &simulation::observation::view::View,
        event_loop: &ActiveEventLoop,
    ) {
        self.apply_admin_view(&view.admin_view, event_loop);
        self.apply_time_view(&view.time_view);
        self.apply_population_view(&view.population_view);
        self.apply_world_view(&view.world_view);
    }

    fn apply_admin_view(
        &mut self,
        admin_view: &simulation::observation::view::AdminView,
        event_loop: &ActiveEventLoop,
    ) {
        if admin_view.mode == simulation::admin::Mode::Exit {
            event_loop.exit();
        }
    }

    fn apply_time_view(&mut self, time_view: &simulation::observation::view::TimeView) {
        self.dt = self.instant.elapsed();
        self.instant = Instant::now();

        let alpha = self.instant.duration_since(time_view.instant.current);

        self.alpha = alpha.as_secs_f32().clamp(0.0, 1.0);
    }

    fn apply_population_view(
        &mut self,
        population_view: &simulation::observation::view::PopulationView,
    ) {
        self.apply_judge_view(&population_view.judge_view);
        self.apply_agent_views(&population_view.agent_views);
    }

    fn apply_judge_view(&mut self, judge_view: &simulation::observation::view::JudgeView) {
        self.camera.update(&self.queue, self.alpha, judge_view);
    }

    fn apply_agent_views(
        &mut self,
        agent_views: &HashMap<
            simulation::population::agent::ID,
            simulation::observation::view::AgentView,
        >,
    ) {
        self.render
            .prepare_agent_views(&self.device, &self.queue, agent_views);
    }

    fn apply_world_view(&mut self, world_view: &simulation::observation::view::WorldView) {
        self.render.prepare_world_view(&self.device, world_view);
    }

    fn send_movement_actions(&mut self) {
        let movement_actions = self.input.get_movement_actions();
        let entity_action = simulation::dispatch::EntityAction::Movement(movement_actions);
        let action = simulation::dispatch::Action::Agent(entity_action);

        self.action_tx.send(action).unwrap();
    }
}
