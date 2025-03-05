use std::sync::Arc;

use camera::Camera;
use input::Input;
use render::Render;

use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct Interface {
    _window: Arc<Window>,
    camera: Camera,
    input: Input,
    render: Render,
}

pub mod camera;
pub mod input;
pub mod render;

impl Interface {
    pub async fn new(window: Arc<Window>) -> Interface {
        let camera = pollster::block_on(Camera::new());
        let input = pollster::block_on(Input::new());
        let render = pollster::block_on(Render::new(window.clone()));

        Interface {
            _window: window,
            camera,
            input,
            render,
        }
    }

    pub fn handle_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        } else {
            self.input.handle_event(&event);
            self.camera.handle_event(&event);
            self.render.handle_event(&event);
        }
    }
}
