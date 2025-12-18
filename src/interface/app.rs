use crate::{
    interface::Interface,
    simulation::{
        manager::{viewer::view::View, Message},
        Simulation,
    },
};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

#[derive(Default)]
pub struct App<'window> {
    interface: Option<Interface<'window>>,
    simulation_handle: Option<std::thread::JoinHandle<()>>,
}

impl<'window> App<'window> {
    pub fn start(
        event_loop: &ActiveEventLoop,
        interface: &mut Option<Interface<'window>>,
        simulation_handle: &mut Option<std::thread::JoinHandle<()>>,
    ) {
        let (message_tx, message_rx) = crossbeam::channel::unbounded::<Message>();
        let (view_input, view_output) = triple_buffer::triple_buffer(&View::new());

        let mut simulation = Box::new(Simulation::new(message_rx, view_input));
        *interface = Some(Interface::new(message_tx, view_output, event_loop));

        *simulation_handle = Some(std::thread::spawn(move || {
            Simulation::run(&mut simulation.manager, &mut simulation.state)
        }));
    }
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        Self::start(event_loop, &mut self.interface, &mut self.simulation_handle);
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
