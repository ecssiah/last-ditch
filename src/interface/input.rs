use crate::{
    interface::{MOUSE_X_SENSITIVITY, MOUSE_Y_SENSITIVITY},
    simulation::{
        action::{Action, MovementActions, WorldAction},
        DEFAULT_X_SPEED, DEFAULT_Z_SPEED,
    },
};
use glam::{Vec2, Vec3};
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent,
    },
    keyboard::{KeyCode, PhysicalKey},
};

pub struct KeyState {
    key_w: f32,
    key_a: f32,
    key_s: f32,
    key_d: f32,
    key_space: f32,
}

pub struct MouseState {
    last_position: Option<Vec2>,
    delta: Vec2,
}

pub struct Input {
    action_tx: UnboundedSender<Action>,
    key_state: KeyState,
    mouse_state: MouseState,
}

impl Input {
    pub fn new(action_tx: UnboundedSender<Action>) -> Self {
        let key_state = KeyState {
            key_w: 0.0,
            key_a: 0.0,
            key_s: 0.0,
            key_d: 0.0,
            key_space: 0.0,
        };

        let mouse_state = MouseState {
            last_position: None,
            delta: Vec2::ZERO,
        };

        let input = Self {
            action_tx,
            key_state,
            mouse_state,
        };

        input
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

    pub fn get_movement_actions(&mut self) -> MovementActions {
        let direction = Vec3::new(
            DEFAULT_X_SPEED * (self.key_state.key_a + self.key_state.key_d),
            0.0,
            DEFAULT_Z_SPEED * (self.key_state.key_w + self.key_state.key_s),
        );

        let rotation = Vec3::new(
            -MOUSE_X_SENSITIVITY * self.mouse_state.delta.y,
            -MOUSE_Y_SENSITIVITY * self.mouse_state.delta.x,
            0.0,
        );

        self.mouse_state.delta = Vec2::ZERO;

        let is_jumping = self.key_state.key_space > 0.0;

        let movement_actions = MovementActions {
            direction,
            rotation,
            is_jumping,
        };

        movement_actions
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
                    self.key_state.key_w += 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_state.key_w -= 1.0;
                }
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.key_state.key_s -= 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_state.key_s += 1.0;
                }
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.key_state.key_a += 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_state.key_a -= 1.0;
                }
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.key_state.key_d -= 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_state.key_d += 1.0;
                }
            }
            PhysicalKey::Code(KeyCode::Space) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.key_state.key_space += 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_state.key_space -= 1.0;
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
            let current_delta = current_position - last_position;

            self.mouse_state.delta += current_delta;
        }

        self.mouse_state.last_position = Some(current_position);
    }
}
