use std::sync::Arc;

use camera::Camera;
use input::Input;
use render::Render;

use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::simulation::state::SharedState;

pub struct Interface {
    _window: Arc<Window>,
    state: SharedState,
    camera: Camera,
    input: Input,
    render: Render,
}

pub mod camera;
pub mod input;
pub mod render;

impl Interface {
    pub async fn new(window: Arc<Window>, state: SharedState) -> Interface {
        let camera = Camera::new();
        let input = Input::new();
        let render = pollster::block_on(Render::new(window.clone(), state.clone()));

        Interface {
            _window: window,
            state,
            camera,
            input,
            render,
        }
    }

    pub fn handle_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: &WindowEvent,
    ) {
        if *event == WindowEvent::CloseRequested {
            event_loop.exit();
        } else {
            self.input.handle_event(&event);
            self.camera.handle_event(&event);
            self.render.handle_event(&event);
        }
    }
}
