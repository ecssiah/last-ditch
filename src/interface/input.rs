use crate::{
    consts::*,
    simulation::action::{Action, InputActions, WorldAction},
    ActionSender,
};
use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent,
    },
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Input {
    action_tx: ActionSender,
    input_actions: InputActions,
}

impl Input {
    pub fn new(action_tx: ActionSender) -> Input {
        let input_actions = InputActions {
            forward: 0.0,
            back: 0.0,
            left: 0.0,
            right: 0.0,
            turn_left: 0.0,
            turn_right: 0.0,
        };

        Input {
            action_tx,
            input_actions,
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

    pub fn reset_input_actions(&mut self) {
        self.input_actions = InputActions {
            forward: 0.0,
            back: 0.0,
            left: 0.0,
            right: 0.0,
            turn_left: 0.0,
            turn_right: 0.0,
        };
    }

    pub fn get_input_actions(&self) -> InputActions {
        self.input_actions
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
            PhysicalKey::Code(KeyCode::KeyQ) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.input_actions.turn_left += DEFAULT_ANGULAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.input_actions.turn_left -= DEFAULT_ANGULAR_SPEED;
                }
            }
            PhysicalKey::Code(KeyCode::KeyE) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.input_actions.turn_right -= DEFAULT_ANGULAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.input_actions.turn_right += DEFAULT_ANGULAR_SPEED;
                }
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.input_actions.forward += DEFAULT_LINEAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.input_actions.forward -= DEFAULT_LINEAR_SPEED;
                }
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.input_actions.forward -= DEFAULT_LINEAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.input_actions.forward += DEFAULT_LINEAR_SPEED;
                }
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.input_actions.left += DEFAULT_STRAFE_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.input_actions.left -= DEFAULT_STRAFE_SPEED;
                }
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.input_actions.right -= DEFAULT_STRAFE_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.input_actions.right += DEFAULT_STRAFE_SPEED;
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

    pub fn handle_cursor_moved(&self, _device_id: &DeviceId, position: &PhysicalPosition<f64>) {}
}
