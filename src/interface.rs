//! The Interface module manages user interaction with the Simulation. This includes
//! both presentation and input management.

pub mod camera;
pub mod consts;
pub mod input;
pub mod render;

use crate::{
    interface::{camera::Camera, consts::*, input::Input, render::Render},
    simulation::{self},
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    window::Window,
};

pub struct Interface {
    dt: Duration,
    instant: Instant,
    alpha: f32,
    action_tx: UnboundedSender<simulation::dispatch::Action>,
    window: Arc<Window>,
    device: wgpu::Device,
    observation: Arc<simulation::observation::Observation>,
    input: Input,
    queue: wgpu::Queue,
    render: Render,
    camera: Camera,
}

impl Interface {
    pub fn new(
        action_tx: UnboundedSender<simulation::dispatch::Action>,
        observation: Arc<simulation::observation::Observation>,
        window: Arc<Window>,
        instance: wgpu::Instance,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> Self {
        let dt = Duration::ZERO;
        let instant = Instant::now();
        let alpha = 0.0;

        let input = Input::new(action_tx.clone());
        let camera = Camera::new(&device);
        let render = Render::new(
            &device,
            &queue,
            window.clone(),
            &instance,
            &adapter,
            &camera,
        );

        let interface = Self {
            dt,
            instant,
            alpha,
            action_tx,
            observation,
            window,
            device,
            input,
            queue,
            camera,
            render,
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
        self.render.resize(&self.device, new_size);
    }

    pub fn handle_about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let view = self.observation.get_view();

        self.check_active(event_loop, &view);

        self.send_movement_actions();

        self.apply_view(&view);
    }

    fn apply_view(&mut self, view: &simulation::observation::view::View) {
        self.apply_time_view(&view.time_view);
        self.apply_population_view(&view.population_view);
        self.apply_world_view(&view.world_view);
    }

    fn apply_time_view(&mut self, time_view: &simulation::observation::view::TimeView) {
        let now = Instant::now();
        self.dt = now - self.instant;
        self.instant = now;

        let alpha = (now - time_view.instant.current).as_secs_f32();
        self.alpha = alpha.clamp(0.0, 1.0);

        log::info!("{:?}", time_view);
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

    pub fn handle_redraw_requested(&mut self) {
        self.render
            .redraw(&self.device, &self.queue, self.window.clone(), &self.camera);
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
