use std::{sync::Arc, thread};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod interface;
mod simulation;

use crate::interface::Interface;
use crate::simulation::Simulation;

#[derive(Default)]
struct App {
    interface: Option<Interface>,
    simulation_thread: Option<thread::JoinHandle<()>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut simulation = Simulation::new();
        let state = simulation.get_shared_state();

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let interface = pollster::block_on(Interface::new(window.clone(), state));

        let simulation_thread = thread::spawn(move || simulation.run());

        self.simulation_thread = Some(simulation_thread);
        self.interface = Some(interface);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let interface = self.interface.as_mut().unwrap();

        interface.handle_event(event_loop, _id, &event);
    }
}

pub fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
