use crate::{
    simulation::action::{Action, EntityAction, WorldAction},
    ActionSender,
};
use cgmath::{Vector2, Vector3, Vector4, Zero};
use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent,
    },
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Input {
    action_tx: ActionSender,
    movement: Vector3<f32>,
    rotation: Vector3<f32>,
}

impl Input {
    pub fn new(action_tx: ActionSender) -> Input {
        let movement = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let rotation = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        Input {
            action_tx,
            movement,
            rotation,
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
                    self.rotation.y = 1.0;
                } else if key_event.state == ElementState::Released {
                    self.rotation.y = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::Rotate(self.rotation)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyE) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.rotation.y = -1.0;
                } else if key_event.state == ElementState::Released {
                    self.rotation.y = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::Rotate(self.rotation)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.movement.z = -1.0;
                } else if key_event.state == ElementState::Released {
                    self.movement.z = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::Move(self.movement)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.movement.x = -1.0;
                } else if key_event.state == ElementState::Released {
                    self.movement.x = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::Move(self.movement)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.movement.z = 1.0;
                } else if key_event.state == ElementState::Released {
                    self.movement.z = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::Move(self.movement)))
                    .unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.movement.x = 1.0;
                } else if key_event.state == ElementState::Released {
                    self.movement.x = 0.0;
                }

                self.action_tx
                    .send(Action::Entity(EntityAction::Move(self.movement)))
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
