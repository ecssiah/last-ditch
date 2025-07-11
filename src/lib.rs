//! # Last Ditch
//!
//! A Civilization Garden

pub mod interface;
pub mod simulation;

mod macros;
#[cfg(test)]
mod tests;

use crate::{
    interface::Interface,
    simulation::{consts::PROJECT_TITLE, Simulation},
};
use tokio::sync::mpsc::unbounded_channel;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

#[derive(Default)]
struct App<'window> {
    interface: Option<Interface<'window>>,
    simulation_thread: Option<tokio::task::JoinHandle<()>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (action_tx, action_rx) =
            unbounded_channel::<simulation::state::receiver::action::Action>();

        let mut simulation = Box::new(Simulation::new(action_rx));
        let interface = Interface::new(event_loop, action_tx, simulation.observation_arc.clone());

        self.simulation_thread = Some(tokio::spawn(async move {
            Simulation::run(
                &mut simulation.timing,
                &mut simulation.receiver,
                simulation.observation_arc.clone(),
                &mut simulation.state,
            )
        }));

        self.interface = Some(interface);
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

pub async fn run() {
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .log_to_file(flexi_logger::FileSpec::default().directory("logs"))
        .write_mode(flexi_logger::WriteMode::Direct)
        .start()
        .unwrap();

    std::env::set_var("RUST_LOG", "wgpu=debug");

    log::info!("{} {}\n", PROJECT_TITLE, env!("CARGO_PKG_VERSION"));

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
