//! User input processing

pub mod key_inputs;
pub mod mouse_inputs;

use crate::{
    interface::{
        Interface, constants::*, gpu::gpu_context::GPUContext, gui::GUI, input::{key_inputs::KeyInputs, mouse_inputs::MouseInputs}
    },
    simulation::manager::{
        self, Message, message::{move_data::MoveData, rotate_data::RotateData}
    },
};
use std::collections::VecDeque;
use tracing::info;
use ultraviolet::Vec2;
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
    pub message_deque: VecDeque<manager::Message>,
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

        let message_deque = VecDeque::new();

        Self {
            key_inputs,
            mouse_inputs,
            message_deque,
        }
    }

    pub fn get_message_deque(
        key_inputs: &KeyInputs,
        mouse_inputs: &mut MouseInputs,
        message_deque: &mut VecDeque<Message>,
    ) -> VecDeque<Message> {
        let move_message = Self::get_move_message(key_inputs);
        let rotate_message = Self::get_rotate_message(mouse_inputs);

        message_deque.push_back(move_message);
        message_deque.push_back(rotate_message);

        std::mem::take(message_deque)
    }

    pub fn get_move_message(key_inputs: &KeyInputs) -> Message {
        let move_data = MoveData {
            move_x: key_inputs.key_a + key_inputs.key_d,
            move_y: key_inputs.key_w + key_inputs.key_s,
            move_z: 0.0,
        };

        Message::Move(move_data)
    }

    fn get_rotate_message(mouse_inputs: &mut MouseInputs) -> Message {
        let rotation_xy = MOUSE_SENSITIVITY * -mouse_inputs.delta.x;
        let rotation_yz = MOUSE_SENSITIVITY * -mouse_inputs.delta.y;

        mouse_inputs.delta = Vec2::broadcast(0.0);

        let rotate_data = RotateData {
            rotate_xy: rotation_xy,
            rotate_yz: rotation_yz,
            rotate_zx: 0.0,
        };

        Message::Rotate(rotate_data)
    }

    pub fn handle_window_event(
        event: &WindowEvent,
        gui: &mut GUI,
        gpu_context: &mut GPUContext,
        input: &mut Input,
    ) -> bool {
        match event {
            WindowEvent::CloseRequested => Self::handle_close_requested(&mut input.message_deque),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => Self::handle_keyboard_input(
                device_id,
                event,
                is_synthetic,
                gui,
                gpu_context,
                &mut input.key_inputs,
                &mut input.message_deque,
            ),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => Self::handle_mouse_input(device_id, state, button, &mut input.message_deque),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => Self::handle_mouse_wheel(device_id, delta, phase, &mut input.message_deque),
            _ => false,
        }
    }

    pub fn handle_device_event(event: &DeviceEvent, input: &mut Input) -> bool {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            return Self::handle_mouse_motion(
                *dx,
                *dy,
                &mut input.mouse_inputs,
                &mut input.message_deque,
            );
        } else {
            return false;
        }
    }

    fn handle_close_requested(message_deque: &mut VecDeque<Message>) -> bool {
        message_deque.push_back(Message::Quit);

        true
    }

    fn handle_keyboard_input(
        _device_id: &DeviceId,
        key_event: &KeyEvent,
        _is_synthetic: &bool,
        gui: &mut GUI,
        gpu_context: &mut GPUContext,
        key_inputs: &mut KeyInputs,
        message_deque: &mut VecDeque<Message>,
    ) -> bool {
        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                message_deque.push_back(Message::Quit);

                true
            }
            PhysicalKey::Code(KeyCode::Tab) => {
                GUI::toggle_menu(gui, gpu_context);

                true
            },
            PhysicalKey::Code(KeyCode::Backquote) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Debug);

                    true
                } else {
                    false
                }
            }
            PhysicalKey::Code(KeyCode::Digit1) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Option1);

                    true
                } else {
                    false
                }
            }
            PhysicalKey::Code(KeyCode::Digit2) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Option2);

                    true
                } else {
                    false
                }
            }
            PhysicalKey::Code(KeyCode::Digit3) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Option3);

                    true
                } else {
                    false
                }
            }
            PhysicalKey::Code(KeyCode::Digit4) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Option4);

                    true
                } else {
                    false
                }
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_w += 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_w -= 1.0;
                }

                true
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_s -= 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_s += 1.0;
                }

                true
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_a -= 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_a += 1.0;
                }

                true
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_d += 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_d -= 1.0;
                }

                true
            }
            PhysicalKey::Code(KeyCode::Space) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    message_deque.push_back(Message::Jump);

                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn handle_mouse_input(
        _device_id: &DeviceId,
        state: &ElementState,
        button: &MouseButton,
        message_deque: &mut VecDeque<Message>,
    ) -> bool {
        if state == &ElementState::Pressed {
            if button == &MouseButton::Left {
                message_deque.push_back(Message::Interact1);

                return true;
            } else if button == &MouseButton::Right {
                message_deque.push_back(Message::Interact2);

                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn handle_mouse_wheel(
        _device_id: &DeviceId,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
        _message_deque: &mut VecDeque<Message>,
    ) -> bool {
        info!("{:?} {:?}", delta, phase);

        false
    }

    fn handle_mouse_motion(
        dx: f64,
        dy: f64,
        mouse_inputs: &mut MouseInputs,
        _message_deque: &mut VecDeque<Message>,
    ) -> bool {
        let delta = Vec2::new(dx as f32, dy as f32);

        mouse_inputs.delta += delta;

        false
    }
}
