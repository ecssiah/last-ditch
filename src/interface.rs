use std::sync::Arc;

use camera::Camera;
use input::Input;
use render::Render;
use winit::{
    dpi::PhysicalPosition, event::{
        DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase,
    },
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct Interface {
    window: Arc<Window>,
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
        let input = pollster::block_on(Input::new(window.clone()));
        let render = pollster::block_on(Render::new(window.clone()));

        Interface {
            window,
            camera,
            input,
            render,
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }
}
