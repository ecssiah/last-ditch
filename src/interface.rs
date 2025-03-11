pub mod camera;
pub mod input;
pub mod render;

use crate::simulation::{
    action::{Action, AgentAction},
    state::State,
};
use camera::Camera;
use input::Input;
use render::Render;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

pub const FOV: f32 = 45.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 100.0;

pub const MOUSE_Y_SENSITIVITY: f32 = 0.009;
pub const MOUSE_X_SENSITIVITY: f32 = 0.006;

pub struct Interface {
    _window: Arc<Window>,
    state: Arc<State>,
    action_tx: UnboundedSender<Action>,
    camera: Camera,
    input: Input,
    render: Render,
}

impl Interface {
    pub async fn new(
        action_tx: UnboundedSender<Action>,
        window: Arc<Window>,
        state: Arc<State>,
    ) -> Interface {
        let camera = Camera::new();
        let input = Input::new(action_tx.clone());
        let render = pollster::block_on(Render::new(
            window.clone(),
            state.agent.clone(),
            state.world.clone(),
            state.blocks.clone(),
            state.chunks.clone(),
        ));

        window.set_cursor_visible(false);

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

        self.send_move_actions();
        self.send_rotate_actions();
    }

    fn check_active(&mut self, event_loop: &ActiveEventLoop) {
        let world = self.state.world.read().unwrap();

        if world.active == false {
            event_loop.exit();
        }
    }

    fn send_move_actions(&mut self) {
        let move_actions = self.input.get_move_actions();

        self.action_tx
            .send(Action::Agent(AgentAction::Move(move_actions)))
            .unwrap();
    }

    fn send_rotate_actions(&mut self) {
        let rotate_actions = self.input.get_rotate_actions();

        if rotate_actions.y_axis.abs() > 1e-6 || rotate_actions.x_axis.abs() > 1e-6 {
            self.action_tx
                .send(Action::Agent(AgentAction::Rotate(rotate_actions)))
                .unwrap();
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        self.input.handle_window_event(&event);
        self.camera.handle_window_event(&event);
        self.render.handle_window_event(&event);
    }
}
