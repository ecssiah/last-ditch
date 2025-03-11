use crate::simulation::{
    action::{Action, MoveActions, RotateActions, WorldAction},
    DEFAULT_LINEAR_SPEED, DEFAULT_STRAFE_SPEED,
};
use glam::Vec2;
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent,
    },
    keyboard::{KeyCode, PhysicalKey},
};

use super::{MOUSE_PITCH_SENSITIVITY, MOUSE_YAW_SENSITIVITY};

pub struct MouseState {
    last_position: Option<Vec2>,
    delta: Vec2,
}

pub struct Input {
    action_tx: UnboundedSender<Action>,
    move_actions: MoveActions,
    mouse_state: MouseState,
}

impl Input {
    pub fn new(action_tx: UnboundedSender<Action>) -> Input {
        let move_actions = MoveActions {
            forward: 0.0,
            backward: 0.0,
            left: 0.0,
            right: 0.0,
        };

        let mouse_state = MouseState {
            last_position: None,
            delta: Vec2::ZERO,
        };

        Input {
            action_tx,
            move_actions,
            mouse_state,
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.action_tx
                    .send(Action::World(WorldAction::Quit))
                    .unwrap();
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                self.handle_keyboard_input(device_id, event, is_synthetic);
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                self.handle_mouse_input(device_id, state, button);
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                self.handle_mouse_wheel(device_id, delta, phase);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                self.handle_cursor_moved(device_id, position);
            }
            _ => (),
        }
    }

    pub fn get_move_actions(&self) -> MoveActions {
        self.move_actions
    }

    pub fn get_rotate_actions(&mut self) -> RotateActions {
        let rotate_actions = RotateActions {
            yaw: -MOUSE_YAW_SENSITIVITY * self.mouse_state.delta.x,
            pitch: -MOUSE_PITCH_SENSITIVITY * self.mouse_state.delta.y,
        };

        self.reset_mouse_state();

        rotate_actions
    }

    pub fn reset_mouse_state(&mut self) {
        self.mouse_state.delta = Vec2::ZERO;
    }

    pub fn handle_keyboard_input(
        &mut self,
        _device_id: &DeviceId,
        key_event: &KeyEvent,
        _is_synthetic: &bool,
    ) {
        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                self.action_tx
                    .send(Action::World(WorldAction::Quit))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.move_actions.forward += DEFAULT_LINEAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.move_actions.forward -= DEFAULT_LINEAR_SPEED;
                }
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.move_actions.forward -= DEFAULT_LINEAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.move_actions.forward += DEFAULT_LINEAR_SPEED;
                }
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.move_actions.left += DEFAULT_STRAFE_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.move_actions.left -= DEFAULT_STRAFE_SPEED;
                }
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.move_actions.right -= DEFAULT_STRAFE_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.move_actions.right += DEFAULT_STRAFE_SPEED;
                }
            }
            _ => (),
        }
    }

    pub fn handle_mouse_input(
        &self,
        _device_id: &DeviceId,
        state: &ElementState,
        button: &MouseButton,
    ) {
        println!("{:?} {:?}", state, button);
    }

    pub fn handle_mouse_wheel(
        &self,
        _device_id: &DeviceId,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
    ) {
        println!("{:?} {:?}", delta, phase);
    }

    pub fn handle_cursor_moved(&mut self, _device_id: &DeviceId, position: &PhysicalPosition<f64>) {
        let current_position = Vec2::new(position.x as f32, position.y as f32);

        if let Some(last_position) = self.mouse_state.last_position {
            self.mouse_state.delta += current_position - last_position;
        }

        self.mouse_state.last_position = Some(current_position);
    }
}
