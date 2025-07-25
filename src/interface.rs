//! Interactions with the User

pub mod camera;
pub mod consts;
pub mod dispatch;
pub mod gpu_context;
pub mod hud;
pub mod input;
pub mod item_render;
pub mod mesh_data;
pub mod population_render;
pub mod texture_data;
pub mod vertex_data;
pub mod world_render;

use crate::{
    interface::{
        camera::Camera, consts::*, dispatch::Dispatch, gpu_context::GPUContext, hud::HUD,
        input::Input, item_render::ItemRender, population_render::PopulationRender,
        world_render::WorldRender,
    },
    simulation::{self},
};
use std::{ops::Deref, sync::Arc, time::Instant};
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Fullscreen, WindowAttributes},
};

pub struct Interface<'window> {
    last_instant: Instant,
    observation_arc: Arc<simulation::observation::Observation>,
    dispatch: Dispatch,
    input: Input,
    camera: Camera,
    hud: HUD,
    world_render: WorldRender,
    item_render: ItemRender,
    population_render: PopulationRender,
    gpu_context: GPUContext<'window>,
}

impl<'window> Interface<'window> {
    pub fn new(
        event_loop: &ActiveEventLoop,
        action_tx: UnboundedSender<simulation::state::receiver::action::Action>,
        observation_arc: Arc<simulation::observation::Observation>,
    ) -> Self {
        let last_instant = Instant::now();

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
        let hud = HUD::new();
        let world_render = WorldRender::new(&gpu_context, &camera);
        let item_render = ItemRender::new(&gpu_context, &camera);
        let population_render = PopulationRender::new(&gpu_context, &camera);

        gpu_context.window_arc.request_redraw();

        Self {
            last_instant,
            observation_arc,
            dispatch,
            input,
            camera,
            hud,
            world_render,
            item_render,
            population_render,
            gpu_context,
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

        let surface_texture_view = surface_texture
            .texture
            .create_view(&self.gpu_context.texture_view_descriptor);

        let depth_texture_view =
            Self::create_depth_texture(&self.gpu_context.device, &self.gpu_context.surface_config);

        WorldRender::render(
            &surface_texture_view,
            &depth_texture_view,
            &self.camera.uniform_bind_group,
            &self.world_render,
            &mut encoder,
        );

        ItemRender::render(
            &self.gpu_context,
            &surface_texture_view,
            &depth_texture_view,
            &self.camera.uniform_bind_group,
            &self.item_render,
            &mut encoder,
        );

        PopulationRender::render(
            &self.gpu_context,
            &surface_texture_view,
            &depth_texture_view,
            &self.camera.uniform_bind_group,
            &self.population_render,
            &mut encoder,
        );

        let full_output = self.hud.get_full_output(
            Arc::clone(&self.gpu_context.window_arc),
            &self.gpu_context.egui_context,
            &mut self.gpu_context.egui_winit_state,
        );

        HUD::render(
            full_output,
            &self.gpu_context.device,
            &self.gpu_context.queue,
            Arc::clone(&self.gpu_context.window_arc),
            &surface_texture_view,
            &self.gpu_context.egui_context,
            &mut self.gpu_context.egui_renderer,
            &mut encoder,
        );

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
        let observation = self.observation_arc.deref();
        let view = simulation::observation::Observation::get_view(&observation.view_buffer_lock);

        if !self.dispatch_actions(&view) {
            let admin_action = simulation::state::receiver::action::AdminAction::Exit;
            let action = simulation::state::receiver::action::Action::Admin(admin_action);

            log::info!("Interface Exit");

            let _ = self.dispatch.send(action);
        } else {
            self.apply_view(event_loop, &view);
        }
    }

    fn apply_view(
        &mut self,
        event_loop: &ActiveEventLoop,
        view: &simulation::observation::view::View,
    ) {
        match view.admin_view.mode {
            simulation::state::admin::Mode::Menu => self.apply_menu_view(view),
            simulation::state::admin::Mode::Load => self.apply_load_view(view),
            simulation::state::admin::Mode::Simulate => self.apply_simulate_view(view),
            simulation::state::admin::Mode::Shutdown => self.apply_shutdown_view(view, event_loop),
        }
    }

    fn apply_menu_view(&mut self, view: &simulation::observation::view::View) {
        self.gpu_context.window_arc.set_cursor_visible(true);
        self.gpu_context
            .window_arc
            .set_cursor_grab(winit::window::CursorGrabMode::None)
            .expect("Failed to grab cursor");

        self.hud.apply_menu_view(view);
    }

    fn apply_load_view(&mut self, view: &simulation::observation::view::View) {
        self.hud.apply_load_view(view);
    }

    fn apply_simulate_view(&mut self, view: &simulation::observation::view::View) {
        self.gpu_context.window_arc.set_cursor_visible(false);
        self.gpu_context
            .window_arc
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect("Failed to grab cursor");

        self.hud.apply_simulate_view(view);

        self.camera
            .apply_judge_view(&self.gpu_context.queue, &view.population_view.judge_view);

        WorldRender::apply_world_view(
            &self.gpu_context.device,
            &view.world_view,
            &self.world_render.block_render_info,
            &self.world_render.block_tile_coordinates_map,
            &mut self.world_render.chunk_render_data_vec,
        );

        PopulationRender::apply_population_view(
            &view.population_view,
            &mut self.population_render.entity_render_data_group_vec,
        );
    }

    fn apply_shutdown_view(
        &mut self,
        view: &simulation::observation::view::View,
        event_loop: &ActiveEventLoop,
    ) {
        self.hud.apply_shutdown_view(view);

        event_loop.exit();
    }

    fn dispatch_actions(&mut self, view: &simulation::observation::view::View) -> bool {
        let mut action_vec = Vec::new();

        match view.admin_view.mode {
            simulation::state::admin::Mode::Menu => {
                let hud_actions = self.hud.get_actions();

                action_vec.extend(hud_actions);
            }
            simulation::state::admin::Mode::Load => {}
            simulation::state::admin::Mode::Simulate => {
                let input_actions = self.input.get_actions();
                let hud_actions = self.hud.get_actions();

                action_vec.extend(input_actions);
                action_vec.extend(hud_actions);
            }
            simulation::state::admin::Mode::Shutdown => {
                let admin_action = simulation::state::receiver::action::AdminAction::Exit;
                let action = simulation::state::receiver::action::Action::Admin(admin_action);

                log::info!("Interface Exit");

                action_vec.push(action);
            }
        }

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
