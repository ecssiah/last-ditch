//! Interactions with the User.

pub mod camera;
pub mod consts;
pub mod hud;
pub mod input;
pub mod render;
pub mod wgpu_state;

use crate::{
    interface::{
        camera::Camera, consts::*, hud::HUD, input::Input, render::Render, wgpu_state::WGPUState,
    },
    simulation::{self, consts::PROJECT_TITLE},
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
    window::{Fullscreen, WindowAttributes},
};

pub struct Interface<'window> {
    dt: Duration,
    instant: Instant,
    last_instant: Instant,
    alpha: f32,
    wgpu_state: WGPUState<'window>,
    action_tx: Arc<UnboundedSender<simulation::dispatch::Action>>,
    observation: Arc<simulation::observation::Observation>,
    input: Input,
    render: Render,
    hud: HUD,
    camera: Camera,
}

impl<'window> Interface<'window> {
    pub fn new(
        event_loop: &ActiveEventLoop,
        action_tx: Arc<UnboundedSender<simulation::dispatch::Action>>,
        observation: Arc<simulation::observation::Observation>,
    ) -> Self {
        let dt = Duration::ZERO;
        let instant = Instant::now();
        let last_instant = Instant::now();
        let alpha = 0.0;

        let monitor = event_loop
            .primary_monitor()
            .expect("No primary monitor found");

        let window_title = format!("{} {}", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));

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

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| *f == wgpu::TextureFormat::Bgra8Unorm)
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_texture_view_descriptor = wgpu::TextureViewDescriptor {
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

        let wgpu_state = WGPUState {
            window,
            device,
            queue,
            size,
            surface,
            surface_config,
            surface_texture_view_descriptor,
        };

        let input = Input::new(action_tx.clone());
        let camera = Camera::new(&wgpu_state.device);

        let render = Render::new(
            &wgpu_state.device,
            &wgpu_state.queue,
            &surface_format,
            &camera,
        );

        let hud = HUD::new(
            &wgpu_state.device,
            wgpu_state.window.clone(),
            surface_format,
        );

        wgpu_state.window.request_redraw();

        Self {
            dt,
            instant,
            last_instant,
            alpha,
            action_tx,
            observation,
            wgpu_state,
            input,
            camera,
            render,
            hud,
        }
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
        let instant = Instant::now();
        let next_instant = self.last_instant + INTERFACE_FRAME_DURATION;
        self.last_instant = instant;

        self.update(event_loop);

        let instant = Instant::now();

        if next_instant > instant {
            event_loop.set_control_flow(ControlFlow::WaitUntil(next_instant));
        };

        self.wgpu_state.window.request_redraw();
    }

    fn update(&mut self, event_loop: &ActiveEventLoop) {
        let view = self.observation.get_view();

        match view.admin_view.mode {
            simulation::admin::Mode::Load => {
                self.apply_admin_view(&view.admin_view);
            }
            simulation::admin::Mode::Simulate => {
                self.apply_admin_view(&view.admin_view);
                self.apply_time_view(&view.time_view);
                self.apply_population_view(&view.population_view);
                self.apply_world_view(&view.world_view);

                self.send_movement_actions();
            }
            simulation::admin::Mode::Shutdown => {}
            simulation::admin::Mode::Exit => {
                event_loop.exit();
            }
        }
    }

    fn handle_redraw_requested(&mut self) {
        let mut encoder = self
            .wgpu_state
            .device
            .create_command_encoder(&Default::default());

        let surface_texture = self
            .wgpu_state
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let texture_view = surface_texture
            .texture
            .create_view(&self.wgpu_state.surface_texture_view_descriptor);

        self.render.update(
            &mut encoder,
            &self.wgpu_state.device,
            &self.wgpu_state.surface_config,
            &texture_view,
            &self.camera,
        );

        self.hud.update(
            &mut encoder,
            &self.wgpu_state.window,
            &self.wgpu_state.device,
            &self.wgpu_state.queue,
            &texture_view,
        );

        self.wgpu_state.queue.submit([encoder.finish()]);
        self.wgpu_state.window.pre_present_notify();

        surface_texture.present();
    }

    fn handle_resized(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.wgpu_state.size = new_size;

        self.wgpu_state
            .surface
            .configure(&self.wgpu_state.device, &self.wgpu_state.surface_config);
    }

    fn apply_admin_view(&mut self, admin_view: &simulation::observation::view::AdminView) {
        self.hud.prepare_load(admin_view);
    }

    fn apply_time_view(&mut self, time_view: &simulation::observation::view::TimeView) {
        self.dt = self.instant.elapsed();
        self.instant = Instant::now();

        let now = self.instant;
        let current = time_view.instant.current;
        let next = time_view.instant.next;

        let total_duration = next.duration_since(current).as_secs_f32();
        let elapsed_since_current = now.duration_since(current).as_secs_f32();

        self.alpha = (elapsed_since_current / total_duration).clamp(0.0, 1.0);
    }

    fn apply_population_view(
        &mut self,
        population_view: &simulation::observation::view::PopulationView,
    ) {
        self.apply_judge_view(&population_view.judge_view);
        self.apply_agent_view_map(&population_view.agent_view_map);
    }

    fn apply_judge_view(&mut self, judge_view: &simulation::observation::view::JudgeView) {
        self.camera
            .update(&self.wgpu_state.queue, self.alpha, judge_view);
    }

    fn apply_agent_view_map(
        &mut self,
        agent_view_map: &HashMap<
            simulation::population::agent::ID,
            simulation::observation::view::AgentView,
        >,
    ) {
        self.render.prepare_agent_view_map(
            &self.wgpu_state.device,
            &self.wgpu_state.queue,
            agent_view_map,
        );
    }

    fn apply_world_view(&mut self, world_view: &simulation::observation::view::WorldView) {
        self.render
            .prepare_world_view(&self.wgpu_state.device, world_view);
    }

    fn send_movement_actions(&mut self) {
        let movement_actions = self.input.get_movement_actions();

        let agent_action = simulation::dispatch::AgentAction::Movement(movement_actions);
        let action = simulation::dispatch::Action::Agent(agent_action);

        self.action_tx.send(action).unwrap();
    }
}
