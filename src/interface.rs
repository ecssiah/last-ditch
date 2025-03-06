pub mod camera;
pub mod input;
pub mod render;

use camera::Camera;
use input::Input;
use render::Render;
use std::sync::Arc;
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::{simulation::state::State, ActionSender};

pub struct Interface {
    _window: Arc<Window>,
    state: Arc<State>,
    action_tx: ActionSender,
    camera: Camera,
    input: Input,
    render: Render,
}

impl Interface {
    pub async fn new(window: Arc<Window>, state: Arc<State>, action_tx: ActionSender) -> Interface {
        let camera = Camera::new();
        let input = Input::new(action_tx.clone());
        let render = pollster::block_on(Render::new(window.clone(), state.world.clone()));

        Interface {
            _window: window,
            state,
            action_tx,
            camera,
            input,
            render,
        }
    }

    pub fn update(&mut self, event_loop: &ActiveEventLoop) {
        let world = self.state.world.read().unwrap();

        println!("{:?}", world.active);

        if world.active == false {
            event_loop.exit();
        }
    }

    pub fn handle_window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: &WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {
                self.input.handle_window_event(&event);
                self.camera.handle_window_event(&event);
                self.render.handle_window_event(&event);
            }
        }
    }
}
