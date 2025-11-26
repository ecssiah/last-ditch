//! User input processing

pub mod key_inputs;
pub mod mouse_inputs;

use crate::{
    interface::{
        constants::*,
        input::{key_inputs::KeyInputs, mouse_inputs::MouseInputs},
    },
    simulation::state::action::{act::MoveData, Act},
};
use std::collections::VecDeque;
use tracing::info;
use ultraviolet::{Vec2, Vec3};
use winit::{
    event::{
        DeviceEvent, DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase,
        WindowEvent,
    },
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Input {
    pub key_inputs: KeyInputs,
    pub mouse_inputs: MouseInputs,
    pub act_deque: VecDeque<Act>,
}

impl Input {
    pub fn new() -> Self {
        let key_inputs = KeyInputs {
            key_w: 0.0,
            key_a: 0.0,
            key_s: 0.0,
            key_d: 0.0,
        };

        let mouse_inputs = MouseInputs {
            delta: Vec2::broadcast(0.0),
        };

        let act_deque = VecDeque::new();

        Self {
            key_inputs,
            mouse_inputs,
            act_deque,
        }
    }

    pub fn get_act_deque(
        key_inputs: &KeyInputs,
        mouse_inputs: &mut MouseInputs,
        act_deque: &mut VecDeque<Act>,
    ) -> VecDeque<Act> {
        let move_act = Self::get_move_act(key_inputs, mouse_inputs);

        act_deque.push_back(move_act);

        std::mem::take(act_deque)
    }

    pub fn get_move_act(key_inputs: &KeyInputs, mouse_inputs: &mut MouseInputs) -> Act {
        let direction = Vec3::new(
            key_inputs.key_a + key_inputs.key_d,
            key_inputs.key_w + key_inputs.key_s,
            0.0,
        );

        let rotation =
            MOUSE_SENSITIVITY * Vec3::new(-mouse_inputs.delta.x, mouse_inputs.delta.y, 0.0);

        mouse_inputs.delta = Vec2::broadcast(0.0);

        let move_data = MoveData {
            direction,
            rotation,
        };

        Act::Move(move_data)
    }

    pub fn handle_window_event(
        event: &WindowEvent,
        key_inputs: &mut KeyInputs,
        act_deque: &mut VecDeque<Act>,
    ) {
        if let Some(act) = match event {
            WindowEvent::CloseRequested => Self::handle_close_requested(),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => Self::handle_keyboard_input(device_id, event, is_synthetic, key_inputs),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => Self::handle_mouse_input(device_id, state, button),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => Self::handle_mouse_wheel(device_id, delta, phase),
            _ => None,
        } {
            act_deque.push_back(act);
        }
    }

    pub fn process_device_event(event: &DeviceEvent, mouse_inputs: &mut MouseInputs) -> bool {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            Self::handle_mouse_motion(*dx, *dy, mouse_inputs);

            true
        } else {
            false
        }
    }

    fn handle_close_requested() -> Option<Act> {
        Some(Act::Quit)
    }

    fn handle_keyboard_input(
        _device_id: &DeviceId,
        key_event: &KeyEvent,
        _is_synthetic: &bool,
        key_inputs: &mut KeyInputs,
    ) -> Option<Act> {
        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => Some(Act::Quit),
            PhysicalKey::Code(KeyCode::Backquote) => {
                if key_event.state == ElementState::Released {
                    Some(Act::Debug)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::Digit1) => {
                if key_event.state == ElementState::Released {
                    Some(Act::Test1)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::Digit2) => {
                if key_event.state == ElementState::Released {
                    Some(Act::Test2)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::Digit3) => {
                if key_event.state == ElementState::Released {
                    Some(Act::Test3)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::Digit4) => {
                if key_event.state == ElementState::Released {
                    Some(Act::Test4)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_w += 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_w -= 1.0;
                }

                None
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_s -= 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_s += 1.0;
                }

                None
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_a -= 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_a += 1.0;
                }

                None
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_d += 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_d -= 1.0;
                }

                None
            }
            PhysicalKey::Code(KeyCode::Space) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    Some(Act::Jump)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn handle_mouse_input(
        _device_id: &DeviceId,
        state: &ElementState,
        button: &MouseButton,
    ) -> Option<Act> {
        info!("{:?} {:?}", state, button);

        None
    }

    fn handle_mouse_wheel(
        _device_id: &DeviceId,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
    ) -> Option<Act> {
        info!("{:?} {:?}", delta, phase);

        None
    }

    fn handle_mouse_motion(dx: f64, dy: f64, mouse_inputs: &mut MouseInputs) -> Option<Act> {
        let delta = Vec2::new(dx as f32, dy as f32);

        mouse_inputs.delta += delta;

        None
    }
}
