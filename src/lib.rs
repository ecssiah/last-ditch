//! # Last Ditch
//!
//! A Civilizational Garden
//!
//! ## Systems
//! The Interface handles interactions between the User and the Simulation.
//!
//! The Simulation handles the evolution of the world.

pub mod interface;
pub mod macros;
pub mod simulation;

use crate::interface::Interface;
use crate::simulation::Simulation;
use simulation::action::Action;
use std::{sync::Arc, thread};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    interface: Option<Interface>,
    simulation_thread: Option<thread::JoinHandle<()>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        ));

        let window = self.window.as_ref().unwrap();

        let (action_tx, action_rx) = unbounded_channel();

        let mut simulation = Simulation::new(action_rx);
        simulation.generate();
        
        let state = simulation.get_state();

        let interface_future = Interface::new(action_tx, window.clone(), state);
        let interface = pollster::block_on(interface_future);

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
}

/// Application entrypoint
pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
