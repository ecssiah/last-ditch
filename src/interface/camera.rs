use winit::event::WindowEvent;

pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        let camera = Self {};

        camera
    }

    pub fn handle_window_event(&mut self, _event: &WindowEvent) {}
}
