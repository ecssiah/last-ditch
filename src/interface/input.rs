use crate::{
    consts::{DEFAULT_ANGULAR_SPEED, DEFAULT_LINEAR_SPEED, DEFAULT_STRAFE_SPEED},
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

pub struct Input {
    action_tx: ActionSender,
}

impl Input {
    pub fn new(action_tx: ActionSender) -> Input {
        Input { action_tx }
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
                let mut angular_speed = 0.0;

                if key_event.state == ElementState::Pressed {
                    angular_speed = DEFAULT_ANGULAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    angular_speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetAngularSpeed(angular_speed)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyE) => {
                let mut angular_speed = 0.0;

                if key_event.state == ElementState::Pressed {
                    angular_speed = -DEFAULT_ANGULAR_SPEED;
                } else if key_event.state == ElementState::Released {
                    angular_speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetAngularSpeed(angular_speed)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
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
                    strafe_speed = -DEFAULT_STRAFE_SPEED;
                } else if key_event.state == ElementState::Released {
                    strafe_speed = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::SetStrafeSpeed(strafe_speed)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
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
            PhysicalKey::Code(KeyCode::KeyD) => {
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
