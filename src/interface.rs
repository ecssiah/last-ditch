pub mod camera;
pub mod input;
pub mod render;

use crate::{simulation::{action::{Action, EntityAction, InputActions}, state::State}, ActionSender};
use camera::Camera;
use input::Input;
use render::Render;
use std::sync::Arc;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

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
        let render = pollster::block_on(Render::new(
            window.clone(),
            state.judge.clone(),
            state.world.clone(),
        ));

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
        self.check_active(event_loop);

        self.send_input_actions();
    }

    fn check_active(&mut self, event_loop: &ActiveEventLoop) {
        let world = self.state.world.read().unwrap();

        if world.active == false {
            event_loop.exit();
        }
    }

    fn send_input_actions(&mut self) {
        self.action_tx
            .send(Action::Entity(EntityAction::Input(self.input.get_input_actions())))
            .unwrap();
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        self.input.handle_window_event(&event);
        self.camera.handle_window_event(&event);
        self.render.handle_window_event(&event);
    }
}
