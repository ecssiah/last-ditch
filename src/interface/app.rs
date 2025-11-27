use crate::{
    interface::Interface,
    simulation::{manager::Message, viewer::View, Simulation},
};
use tokio::sync::mpsc::unbounded_channel;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

#[derive(Default)]
pub struct App<'window> {
    interface: Option<Interface<'window>>,
    simulation_thread: Option<tokio::task::JoinHandle<()>>,
}

impl<'window> App<'window> {
    pub fn start(
        event_loop: &ActiveEventLoop,
        interface: &mut Option<Interface<'window>>,
        simulation_thread: &mut Option<tokio::task::JoinHandle<()>>,
    ) {
        let (message_tx, message_rx) = unbounded_channel::<Message>();
        let (view_input, view_output) = triple_buffer::triple_buffer(&View::new());

        let mut simulation = Box::new(Simulation::new(message_rx, view_input));
        *interface = Some(Interface::new(event_loop, message_tx, view_output));

        *simulation_thread = Some(tokio::spawn(async move {
            Simulation::run(
                &mut simulation.manager,
                &mut simulation.state,
                &mut simulation.viewer,
            )
        }));
    }
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        Self::start(event_loop, &mut self.interface, &mut self.simulation_thread);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        Interface::update(event_loop, &mut self.interface);
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        Interface::handle_window_event(&event, &mut self.interface);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        Interface::handle_device_event(&event, &mut self.interface);
    }
}
