use std::collections::HashMap;

use crate::{
    consts::*,
    simulation::action::{Action, EntityAction, WorldAction},
    ActionSender,
};
use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent,
    },
    keyboard::{KeyCode, PhysicalKey},
};

pub struct InputTypes {
    forward: bool,
    back: bool,
    left: bool,
    right: bool,
    turn_left: bool,
    turn_right: bool,
}

pub struct Input {
    action_tx: ActionSender,
    input_types: InputTypes,
    speed: f32,
    strafe_speed: f32,
    angular_speed: f32,
}

impl Input {
    pub fn new(action_tx: ActionSender) -> Input {
        let input_types = InputTypes {
            forward: false,
            back: false,
            left: false,
            right: false,
            turn_left: false,
            turn_right: false,
        };

        Input {
            action_tx,
            input_types,
            speed: 0.0,
            strafe_speed: 0.0,
            angular_speed: 0.0,
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
                    self.angular_speed += DEFAULT_ANGULAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.angular_speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetAngularSpeed(self.angular_speed)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyE) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.angular_speed -= DEFAULT_ANGULAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    self.angular_speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetAngularSpeed(self.angular_speed)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                let mut speed = 0.0;

                if key_event.state == ElementState::Pressed {
                    speed = DEFAULT_LINEAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetLinearSpeed(speed)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                let mut speed = 0.0;

                if key_event.state == ElementState::Pressed {
                    speed = -DEFAULT_LINEAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetLinearSpeed(speed)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                let mut strafe_speed = 0.0;

                if key_event.state == ElementState::Pressed {
                    strafe_speed = DEFAULT_STRAFE_SPEED;
                } else if key_event.state == ElementState::Released {
                    strafe_speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetStrafeSpeed(strafe_speed)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                let mut strafe_speed = 0.0;

                if key_event.state == ElementState::Pressed {
                    strafe_speed = -DEFAULT_STRAFE_SPEED;
                } else if key_event.state == ElementState::Released {
                    strafe_speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetStrafeSpeed(strafe_speed)))
                    .unwrap();
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
