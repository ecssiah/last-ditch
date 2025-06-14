//! User input processing

use crate::{
    interface::{MOUSE_X_SENSITIVITY, MOUSE_Y_SENSITIVITY},
    simulation::dispatch::{Action, AgentAction, JumpAction, MovementAction, WorldAction},
};
use glam::{Vec2, Vec3};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use winit::{
    event::{
        DeviceEvent, DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase,
        WindowEvent,
    },
    keyboard::{KeyCode, PhysicalKey},
};

pub struct KeyInputs {
    key_w: f32,
    key_a: f32,
    key_s: f32,
    key_d: f32,
}

pub struct MouseInputs {
    delta: Vec2,
}

pub struct Input {
    pub action_tx: Arc<UnboundedSender<Action>>,
    pub key_inputs: KeyInputs,
    pub mouse_inputs: MouseInputs,
}

impl Input {
    pub fn new(action_tx: Arc<UnboundedSender<Action>>) -> Self {
        let key_inputs = KeyInputs {
            key_w: 0.0,
            key_a: 0.0,
            key_s: 0.0,
            key_d: 0.0,
        };

        let mouse_inputs = MouseInputs { delta: Vec2::ZERO };

        Self {
            action_tx,
            key_inputs,
            mouse_inputs,
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                let world_action = WorldAction::Exit;
                let action = Action::World(world_action);

                self.action_tx.send(action).unwrap();
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
            _ => (),
        }
    }

    pub fn handle_device_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                self.handle_mouse_motion(*dx, *dy);
            }
            _ => (),
        }
    }

    pub fn get_movement_actions(&mut self) -> MovementAction {
        let direction = Vec3::new(
            self.key_inputs.key_a + self.key_inputs.key_d,
            0.0,
            self.key_inputs.key_w + self.key_inputs.key_s,
        );

        let pitch = MOUSE_X_SENSITIVITY * self.mouse_inputs.delta.y;
        let yaw = MOUSE_Y_SENSITIVITY * self.mouse_inputs.delta.x;

        self.mouse_inputs.delta = Vec2::ZERO;

        let movement_actions = MovementAction {
            direction,
            pitch,
            yaw,
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
                let world_action = WorldAction::Exit;
                let action = Action::World(world_action);

                self.action_tx.send(action).unwrap();
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.key_inputs.key_w += 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_inputs.key_w -= 1.0;
                }
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.key_inputs.key_s -= 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_inputs.key_s += 1.0;
                }
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.key_inputs.key_a -= 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_inputs.key_a += 1.0;
                }
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    self.key_inputs.key_d += 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_inputs.key_d -= 1.0;
                }
            }
            PhysicalKey::Code(KeyCode::Space) => {
                if key_event.state == ElementState::Pressed && key_event.repeat == false {
                    let jump_action = JumpAction::Start;
                    let agent_action = AgentAction::Jump(jump_action);
                    let action = Action::Agent(agent_action);

                    self.action_tx.send(action).unwrap();
                } else if key_event.state == ElementState::Released {
                    let jump_action = JumpAction::End;
                    let agent_action = AgentAction::Jump(jump_action);
                    let action = Action::Agent(agent_action);

                    self.action_tx.send(action).unwrap();
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
        log::info!("{:?} {:?}", state, button);
    }

    pub fn handle_mouse_wheel(
        &self,
        _device_id: &DeviceId,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
    ) {
        log::info!("{:?} {:?}", delta, phase);
    }

    pub fn handle_mouse_motion(&mut self, dx: f64, dy: f64) {
        let delta = Vec2::new(dx as f32, dy as f32);

        self.mouse_inputs.delta += delta;
    }
}
