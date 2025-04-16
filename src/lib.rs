//! # Last Ditch
//!
//! A Civilizational Garden
//!
//! ## Systems
//! The Interface handles interactions between the User and the Simulation.
//!
//! The Simulation handles the evolution of the world.

pub mod interface;
pub mod simulation;

mod macros;

use crate::{
    interface::{consts::*, Interface},
    simulation::Simulation,
};
use std::{sync::Arc, thread};
use tokio::sync::mpsc::unbounded_channel;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Fullscreen, WindowAttributes, WindowId},
};

#[derive(Default)]
struct App {
    interface: Option<Interface>,
    simulation_thread: Option<thread::JoinHandle<()>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let monitor = event_loop
            .primary_monitor()
            .expect("No primary monitor found");

        let window_attributes = if FULLSCREEN {
            WindowAttributes::default()
                .with_title(interface::consts::WINDOW_TITLE)
                .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
        } else {
            WindowAttributes::default()
                .with_title(interface::consts::WINDOW_TITLE)
                .with_inner_size(PhysicalSize::new(
                    interface::consts::WINDOW_WIDTH,
                    interface::consts::WINDOW_HEIGHT,
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

        let (action_tx, action_rx) = unbounded_channel();

        let mut simulation = Box::new(Simulation::new(action_rx));

        let observation_lock = simulation.get_observation_arc();

        let simulation_thread = thread::spawn(move || {
            simulation.run();
        });

        let interface = Interface::new(
            action_tx,
            Arc::clone(&observation_lock),
            Arc::clone(&window),
            instance,
            adapter,
            device,
            queue,
        );

        self.simulation_thread = Some(simulation_thread);
        self.interface = Some(interface);

        window.request_redraw();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let interface = self.interface.as_mut().unwrap();

        interface.handle_about_to_wait(event_loop);
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
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .log_to_file(flexi_logger::FileSpec::default().directory("logs"))
        .write_mode(flexi_logger::WriteMode::Direct)
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
