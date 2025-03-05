use winit::event::WindowEvent;

pub struct Camera {}

impl Camera {
    pub async fn new() -> Camera {
        Camera {}
    }

    pub fn handle_event(&mut self, _event: &WindowEvent) {

    }
}
