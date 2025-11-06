//! User interaction

pub mod camera;
pub mod consts;
pub mod debug;
pub mod dispatch;
pub mod gpu_context;
pub mod hud;
pub mod input;
pub mod item_render;
pub mod mesh_data;
pub mod mesh_optimizer;
pub mod population_render;
pub mod texture_data;
pub mod vertex_data;
pub mod world_render;

use crate::{
    interface::{
        camera::Camera, consts::*, debug::DebugRender, dispatch::Dispatch, gpu_context::GPUContext,
        hud::HUD, input::Input, item_render::ItemRender, population_render::PopulationRender,
        world_render::WorldRender,
    },
    simulation::{
        self,
        observation::{view::View, Observation},
        state::{
            admin,
            receiver::action::{Action, AdminAction},
        },
    },
};
use std::{sync::Arc, time::Instant};
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Fullscreen, WindowAttributes},
};

pub struct Interface<'window> {
    pub last_instant: Instant,
    pub dispatch: Dispatch,
    pub input: Input,
    pub camera: Camera,
    pub hud: HUD,
    pub world_render: WorldRender,
    pub item_render: ItemRender,
    pub population_render: PopulationRender,
    pub debug_render: DebugRender,
    pub gpu_context: GPUContext<'window>,
    pub view_output: triple_buffer::Output<View>,
}

impl<'window> Interface<'window> {
    pub fn new(
        event_loop: &ActiveEventLoop,
        action_tx: UnboundedSender<Action>,
        view_output: triple_buffer::Output<View>,
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
        let debug_render = DebugRender::new(&gpu_context, &camera);

        gpu_context.window_arc.request_redraw();

        Self {
            last_instant,
            dispatch,
            input,
            camera,
            hud,
            world_render,
            item_render,
            population_render,
            debug_render,
            gpu_context,
            view_output,
        }
    }

    pub fn handle_about_to_wait(
        event_loop: &ActiveEventLoop,
        gpu_context: &GPUContext,
        dispatch: &Dispatch,
        last_instant: &mut Instant,
        camera: &mut Camera,
        input: &mut Input,
        hud: &mut HUD,
        world_render: &mut WorldRender,
        population_render: &mut PopulationRender,
        debug_render: &mut DebugRender,
        view_output: &mut triple_buffer::Output<View>,
    ) {
        let instant = Instant::now();
        let next_instant = *last_instant + INTERFACE_FRAME_DURATION;
        *last_instant = instant;

        Self::update(
            event_loop,
            dispatch,
            gpu_context,
            camera,
            hud,
            input,
            world_render,
            population_render,
            debug_render,
            view_output,
        );

        let instant = Instant::now();

        if next_instant > instant {
            event_loop.set_control_flow(ControlFlow::WaitUntil(next_instant));
        };

        gpu_context.window_arc.request_redraw();
    }

    pub fn handle_device_event(
        event: &DeviceEvent,
        gpu_context: &mut GPUContext,
        input: &mut Input,
        hud: &mut HUD,
    ) {
        let is_handled = HUD::handle_device_event(event, &hud.mode, gpu_context);

        if !is_handled {
            Input::handle_device_event(event, &mut input.mouse_inputs);
        }
    }

    pub fn handle_window_event(
        event: &WindowEvent,
        camera: &Camera,
        gpu_context: &mut GPUContext,
        input: &mut Input,
        hud: &mut HUD,
        world_render: &mut WorldRender,
        population_render: &mut PopulationRender,
        item_render: &mut ItemRender,
        debug_render: &mut DebugRender,
    ) {
        match event {
            WindowEvent::RedrawRequested => Self::handle_redraw_requested(
                camera,
                gpu_context,
                hud,
                world_render,
                item_render,
                population_render,
                debug_render,
            ),
            WindowEvent::Resized(size) => Self::handle_resized(*size, gpu_context),
            _ => {
                let is_handled = HUD::handle_window_event(event, &hud.mode, gpu_context);

                if !is_handled {
                    Input::handle_window_event(event, &mut input.key_inputs, &mut input.action_vec);
                }
            }
        }
    }

    fn handle_redraw_requested(
        camera: &Camera,
        gpu_context: &mut GPUContext,
        hud: &mut HUD,
        world_render: &mut WorldRender,
        item_render: &mut ItemRender,
        population_render: &mut PopulationRender,
        debug_render: &mut DebugRender,
    ) {
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

        WorldRender::render(
            &surface_texture_view,
            &depth_texture_view,
            &camera.uniform_bind_group,
            world_render,
            &mut encoder,
        );

        ItemRender::render(
            &surface_texture_view,
            &depth_texture_view,
            gpu_context,
            &camera.uniform_bind_group,
            item_render,
            &mut encoder,
        );

        PopulationRender::render(
            &surface_texture_view,
            &depth_texture_view,
            gpu_context,
            &camera.uniform_bind_group,
            population_render,
            &mut encoder,
        );

        HUD::render(
            &surface_texture_view,
            Arc::clone(&gpu_context.window_arc),
            &gpu_context.device,
            &gpu_context.queue,
            &gpu_context.egui_context,
            hud,
            &mut gpu_context.egui_winit_state,
            &mut gpu_context.egui_renderer,
            &mut encoder,
        );

        if debug_render.visible {
            DebugRender::render(
                &surface_texture_view,
                &depth_texture_view,
                gpu_context,
                debug_render,
                &mut encoder,
            );
        }

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

    fn update(
        event_loop: &ActiveEventLoop,
        dispatch: &Dispatch,
        gpu_context: &GPUContext,
        camera: &mut Camera,
        hud: &mut HUD,
        input: &mut Input,
        world_render: &mut WorldRender,
        population_render: &mut PopulationRender,
        debug_render: &mut DebugRender,
        view_output: &mut triple_buffer::Output<View>,
    ) {
        let view = Observation::get_view(view_output);

        if !Self::dispatch_action_vec(view, dispatch, hud, input) {
            let admin_action = AdminAction::Exit;
            let action = Action::Admin(admin_action);

            log::info!("Interface Exit");

            let _ = dispatch.send(action);
        } else {
            Self::apply_view(
                event_loop,
                view,
                gpu_context,
                camera,
                hud,
                world_render,
                population_render,
                debug_render,
            );
        }
    }

    fn apply_view(
        event_loop: &ActiveEventLoop,
        view: &View,
        gpu_context: &GPUContext,
        camera: &mut Camera,
        hud: &mut HUD,
        world_render: &mut WorldRender,
        population_render: &mut PopulationRender,
        debug_render: &mut DebugRender,
    ) {
        match view.admin_view.mode {
            admin::Mode::Menu => Self::apply_menu_view(view, gpu_context, hud),
            admin::Mode::Load => Self::apply_load_view(view, hud),
            admin::Mode::Simulate => Self::apply_simulate_view(
                view,
                gpu_context,
                camera,
                hud,
                world_render,
                population_render,
                debug_render,
            ),
            admin::Mode::Shutdown => Self::apply_shutdown_view(view, event_loop, hud),
        }
    }

    fn apply_menu_view(view: &View, gpu_context: &GPUContext, hud: &mut HUD) {
        gpu_context.window_arc.set_cursor_visible(true);
        gpu_context
            .window_arc
            .set_cursor_grab(winit::window::CursorGrabMode::None)
            .expect("Failed to grab cursor");

        HUD::apply_menu_view(view, &mut hud.mode);
    }

    fn apply_load_view(view: &View, hud: &mut HUD) {
        HUD::apply_load_view(view, &mut hud.mode);
    }

    fn apply_simulate_view(
        view: &View,
        gpu_context: &GPUContext,
        camera: &mut Camera,
        hud: &mut HUD,
        world_render: &mut WorldRender,
        population_render: &mut PopulationRender,
        debug_render: &mut DebugRender,
    ) {
        gpu_context.window_arc.set_cursor_visible(false);

        gpu_context
            .window_arc
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect("Failed to grab cursor");

        HUD::apply_simulate_view(view, &mut hud.mode);

        Camera::apply_judge_view(
            &gpu_context.queue,
            &view.population_view.judge_view,
            &camera.uniform_buffer,
        );

        WorldRender::apply_world_view(
            &gpu_context.device,
            &view.world_view,
            &world_render.cell_render_info,
            &world_render.cell_tile_coordinates_map,
            &mut world_render.sector_render_data_vec,
        );

        PopulationRender::apply_population_view(
            &view.population_view,
            &mut population_render.entity_instance_data_group_vec,
        );

        DebugRender::apply_debug_view(view, debug_render);
    }

    fn apply_shutdown_view(view: &View, event_loop: &ActiveEventLoop, hud: &mut HUD) {
        HUD::apply_shutdown_view(view, &mut hud.mode);

        event_loop.exit();
    }

    fn dispatch_action_vec(
        view: &View,
        dispatch: &Dispatch,
        hud: &mut HUD,
        input: &mut Input,
    ) -> bool {
        let mut action_vec = Vec::new();

        match view.admin_view.mode {
            admin::Mode::Menu => {
                let hud_action_vec = HUD::get_action_vec(&mut hud.action_vec);

                action_vec.extend(hud_action_vec);
            }
            admin::Mode::Load => {}
            admin::Mode::Simulate => {
                let input_action_vec = Input::get_action_vec(
                    &input.key_inputs,
                    &mut input.mouse_inputs,
                    &mut input.action_vec,
                );

                let hud_action_vec = HUD::get_action_vec(&mut hud.action_vec);

                action_vec.extend(input_action_vec);
                action_vec.extend(hud_action_vec);
            }
            admin::Mode::Shutdown => {
                let admin_action = AdminAction::Exit;
                let action = Action::Admin(admin_action);

                log::info!("Interface Exit");

                action_vec.push(action);
            }
        }

        for action in action_vec {
            match dispatch.send(action) {
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
