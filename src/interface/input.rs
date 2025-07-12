//! User input processing

use crate::{interface::consts::*, simulation};
use glam::{Vec2, Vec3};
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
    pub action_vec: Vec<simulation::state::receiver::action::Action>,
    pub key_inputs: KeyInputs,
    pub mouse_inputs: MouseInputs,
}

impl Input {
    pub fn new() -> Self {
        let key_inputs = KeyInputs {
            key_w: 0.0,
            key_a: 0.0,
            key_s: 0.0,
            key_d: 0.0,
        };

        let mouse_inputs = MouseInputs { delta: Vec2::ZERO };

        let action_vec = Vec::new();

        Self {
            key_inputs,
            mouse_inputs,
            action_vec,
        }
    }

    pub fn get_actions(&mut self) -> Vec<simulation::state::receiver::action::Action> {
        let movement_action = self.get_movement_action();
        self.action_vec.push(movement_action);

        std::mem::take(&mut self.action_vec)
    }

    pub fn get_movement_action(&mut self) -> simulation::state::receiver::action::Action {
        let direction = Vec3::new(
            self.key_inputs.key_a + self.key_inputs.key_d,
            0.0,
            self.key_inputs.key_w + self.key_inputs.key_s,
        );

        let rotation = Vec3::new(
            0.0,
            self.mouse_inputs.delta.x * MOUSE_SENSITIVITY,
            self.mouse_inputs.delta.y * MOUSE_SENSITIVITY,
        );

        self.mouse_inputs.delta = Vec2::ZERO;

        let movement_data = simulation::state::receiver::action::MovementData {
            direction,
            rotation,
        };

        let judge_action =
            simulation::state::receiver::action::JudgeAction::Movement(movement_data);

        simulation::state::receiver::action::Action::Judge(judge_action)
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        if let Some(action) = match event {
            WindowEvent::CloseRequested => self.handle_close_requested(),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => self.handle_keyboard_input(device_id, event, is_synthetic),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => self.handle_mouse_input(device_id, state, button),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => self.handle_mouse_wheel(device_id, delta, phase),
            _ => None,
        } {
            self.action_vec.push(action);
        }
    }

    pub fn handle_device_event(&mut self, event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            self.handle_mouse_motion(*dx, *dy);
        }
    }

    fn handle_close_requested(&mut self) -> Option<simulation::state::receiver::action::Action> {
        let admin_action = simulation::state::receiver::action::AdminAction::Quit;
        let action = simulation::state::receiver::action::Action::Admin(admin_action);

        Some(action)
    }

    fn handle_keyboard_input(
        &mut self,
        _device_id: &DeviceId,
        key_event: &KeyEvent,
        _is_synthetic: &bool,
    ) -> Option<simulation::state::receiver::action::Action> {
        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                let admin_action = simulation::state::receiver::action::AdminAction::Quit;
                let action = simulation::state::receiver::action::Action::Admin(admin_action);

                Some(action)
            }
            PhysicalKey::Code(KeyCode::Digit1) => {
                if key_event.state == ElementState::Released {
                    let test_action = simulation::state::receiver::action::TestAction::Test1;
                    let action = simulation::state::receiver::action::Action::Test(test_action);

                    Some(action)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::Digit2) => {
                if key_event.state == ElementState::Released {
                    let test_action = simulation::state::receiver::action::TestAction::Test2;
                    let action = simulation::state::receiver::action::Action::Test(test_action);

                    Some(action)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::Digit3) => {
                if key_event.state == ElementState::Released {
                    let test_action = simulation::state::receiver::action::TestAction::Test3;
                    let action = simulation::state::receiver::action::Action::Test(test_action);

                    Some(action)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::Digit4) => {
                if key_event.state == ElementState::Released {
                    let test_action = simulation::state::receiver::action::TestAction::Test4;
                    let action = simulation::state::receiver::action::Action::Test(test_action);

                    Some(action)
                } else {
                    None
                }
            }
            PhysicalKey::Code(KeyCode::KeyW) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    self.key_inputs.key_w += 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_inputs.key_w -= 1.0;
                }

                None
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    self.key_inputs.key_s -= 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_inputs.key_s += 1.0;
                }

                None
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    self.key_inputs.key_a -= 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_inputs.key_a += 1.0;
                }

                None
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    self.key_inputs.key_d += 1.0;
                } else if key_event.state == ElementState::Released {
                    self.key_inputs.key_d -= 1.0;
                }

                None
            }
            PhysicalKey::Code(KeyCode::Space) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    let jump_action = simulation::state::receiver::action::JumpAction::Start;
                    let judge_action =
                        simulation::state::receiver::action::JudgeAction::Jump(jump_action);
                    let action = simulation::state::receiver::action::Action::Judge(judge_action);

                    Some(action)
                } else if key_event.state == ElementState::Released {
                    let jump_action = simulation::state::receiver::action::JumpAction::End;
                    let judge_action =
                        simulation::state::receiver::action::JudgeAction::Jump(jump_action);
                    let action = simulation::state::receiver::action::Action::Judge(judge_action);

                    Some(action)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn handle_mouse_input(
        &self,
        _device_id: &DeviceId,
        state: &ElementState,
        button: &MouseButton,
    ) -> Option<simulation::state::receiver::action::Action> {
        log::info!("{:?} {:?}", state, button);

        None
    }

    fn handle_mouse_wheel(
        &self,
        _device_id: &DeviceId,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
    ) -> Option<simulation::state::receiver::action::Action> {
        log::info!("{:?} {:?}", delta, phase);

        None
    }

    fn handle_mouse_motion(
        &mut self,
        dx: f64,
        dy: f64,
    ) -> Option<simulation::state::receiver::action::Action> {
        let delta = Vec2::new(dx as f32, dy as f32);

        self.mouse_inputs.delta += delta;

        None
    }
}
