//! User input processing

pub mod key_inputs;
pub mod mouse_inputs;

use crate::{
    interface::{
        constants::*, debug_renderer::DebugRenderer, gpu::gpu_context::GPUContext, gui::GUI, input::{key_inputs::KeyInputs, mouse_inputs::MouseInputs}
    },
    simulation::manager::{
        self, Message, message::{movement_input_data::MovementInputData, rotation_input_data::RotationInputData}
    },
};
use std::collections::VecDeque;
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
            key_q: 0.0,
            key_e: 0.0,
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
        let movement_input_message = Self::get_movement_input_message(key_inputs);
        let rotation_input_message = Self::get_rotation_input_message(mouse_inputs);

        message_deque.push_back(movement_input_message);
        message_deque.push_back(rotation_input_message);

        std::mem::take(message_deque)
    }

    pub fn get_movement_input_message(key_inputs: &KeyInputs) -> Message {
        let movement_input_data = MovementInputData {
            input_x: key_inputs.key_a + key_inputs.key_d,
            input_y: key_inputs.key_w + key_inputs.key_s,
            input_z: key_inputs.key_q + key_inputs.key_e,
        };

        Message::MovementInput(movement_input_data)
    }

    fn get_rotation_input_message(mouse_inputs: &mut MouseInputs) -> Message {
        let rotation_input_data = RotationInputData {
            input_x: MOUSE_SENSITIVITY * -mouse_inputs.delta.y,
            input_y: 0.0,
            input_z: MOUSE_SENSITIVITY * -mouse_inputs.delta.x,
        };

        mouse_inputs.delta = Vec2::zero();

        Message::RotatationInput(rotation_input_data)
    }

    pub fn handle_window_event(
        event: &WindowEvent,
        gui: &mut GUI,
        debug_renderer: &mut DebugRenderer,
        gpu_context: &mut GPUContext,
        input: &mut Self,
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
                debug_renderer,
                gpu_context,
                &mut input.key_inputs,
                &mut input.message_deque,
            ),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => Self::handle_mouse_input(device_id, state, button, gui, &mut input.message_deque),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => Self::handle_mouse_wheel(device_id, delta, phase, &mut input.message_deque),
            _ => false,
        }
    }

    pub fn handle_device_event(event: &DeviceEvent, gui: &GUI, input: &mut Self) -> bool {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            return Self::handle_mouse_motion(
                *dx,
                *dy,
                gui,
                &mut input.mouse_inputs,
                &mut input.message_deque,
            );
        }

        false
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
        debug_renderer: &mut DebugRenderer,
        gpu_context: &mut GPUContext,
        key_inputs: &mut KeyInputs,
        message_deque: &mut VecDeque<Message>,
    ) -> bool {
        if key_event.physical_key == PhysicalKey::Code(KeyCode::Tab) {
            if key_event.state == ElementState::Released {
                GUI::toggle_menu_active(gui, gpu_context);
            }

            return true;
        }

        if gui.menu_active {
            return false;
        }

        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::Backquote) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Debug);

                    DebugRenderer::toggle_debug_active(debug_renderer);
                }

                true
            }
            PhysicalKey::Code(KeyCode::Digit1) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Option1);
                }

                true
            }
            PhysicalKey::Code(KeyCode::Digit2) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Option2);
                }

                true
            }
            PhysicalKey::Code(KeyCode::Digit3) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Option3);
                }

                true
            }
            PhysicalKey::Code(KeyCode::Digit4) => {
                if key_event.state == ElementState::Released {
                    message_deque.push_back(Message::Option4);
                }

                true
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
            PhysicalKey::Code(KeyCode::KeyQ) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_q -= 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_q += 1.0;
                }

                true
            }
            PhysicalKey::Code(KeyCode::KeyE) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    key_inputs.key_e += 1.0;
                } else if key_event.state == ElementState::Released {
                    key_inputs.key_e -= 1.0;
                }

                true
            }
            PhysicalKey::Code(KeyCode::Space) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    message_deque.push_back(Message::JumpInput);
                }

                true
            }
            _ => false,
        }
    }

    fn handle_mouse_input(
        _device_id: &DeviceId,
        state: &ElementState,
        button: &MouseButton,
        gui: &GUI,
        message_deque: &mut VecDeque<Message>,
    ) -> bool {
        if gui.menu_active {
            return false;
        }

        if state == &ElementState::Pressed {
            if button == &MouseButton::Left {
                message_deque.push_back(Message::Interact1);

                true
            } else if button == &MouseButton::Right {
                message_deque.push_back(Message::Interact2);

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn handle_mouse_wheel(
        _device_id: &DeviceId,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
        _message_deque: &mut VecDeque<Message>,
    ) -> bool {
        tracing::info!("{:?} {:?}", delta, phase);

        true
    }

    fn handle_mouse_motion(
        dx: f64,
        dy: f64,
        gui: &GUI,
        mouse_inputs: &mut MouseInputs,
        _message_deque: &mut VecDeque<Message>,
    ) -> bool {
        if gui.menu_active {
            return false;
        }

        let delta = Vec2::new(dx as f32, dy as f32);

        mouse_inputs.delta += delta;

        true
    }
}
