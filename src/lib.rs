//! # Last Ditch
//!
//! A Civilizational Garden
//!
//! ## Systems
//! The Interface handles interactions between the User and the Simulation.
//!
//! The Simulation handles the evolution of the world.

pub mod interface;
mod macros;
pub mod simulation;

use crate::{interface::Interface, simulation::Simulation};
use flexi_logger::{Logger, WriteMode};
use std::{sync::Arc, thread};
use tokio::sync::mpsc::unbounded_channel;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    interface: Option<Interface>,
    simulation_thread: Option<thread::JoinHandle<()>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(interface::consts::WINDOW_TITLE)
            .with_inner_size(PhysicalSize::new(
                interface::consts::WINDOW_WIDTH,
                interface::consts::WINDOW_HEIGHT,
            ));

        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));

        let window = self.window.as_ref().unwrap();

        window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect("Failed to grab cursor");
        window.set_cursor_visible(false);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter_future = instance.request_adapter(&wgpu::RequestAdapterOptions::default());
        let adapter = pollster::block_on(adapter_future).unwrap();

        let device_future = adapter.request_device(&wgpu::DeviceDescriptor::default(), None);
        let (device, queue) = pollster::block_on(device_future).unwrap();

        let (action_tx, action_rx) = unbounded_channel();

        let mut simulation = Simulation::new(action_rx);
        simulation.generate();

        let state = simulation.get_state();

        let interface = Interface::new(
            action_tx,
            window.clone(),
            instance,
            adapter,
            device,
            queue,
            state,
        );

        let simulation_thread = thread::spawn(move || simulation.run());

        self.simulation_thread = Some(simulation_thread);
        self.interface = Some(interface);

        window.request_redraw();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let interface = self.interface.as_mut().unwrap();

        interface.update(event_loop);
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let interface = self.interface.as_mut().unwrap();

        interface.handle_window_event(&event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let interface = self.interface.as_mut().unwrap();

        interface.handle_device_event(&event);
    }
}

/// Application entrypoint
pub async fn run() {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(flexi_logger::FileSpec::default().directory("logs"))
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    std::env::set_var("RUST_LOG", "wgpu=debug");

    log::info!("Last Ditch");
    log::info!("Version: {:?}", env!("CARGO_PKG_VERSION"));
    log::info!("");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
