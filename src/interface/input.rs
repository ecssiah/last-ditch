//! User input processing

use crate::{
    interface::consts::{MOUSE_X_SENSITIVITY, MOUSE_Y_SENSITIVITY},
    simulation,
};
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

        Self {
            key_inputs,
            mouse_inputs,
        }
    }

    pub fn get_movement_actions(&mut self) -> simulation::state::receiver::action::Action {
        let direction = Vec3::new(
            self.key_inputs.key_a + self.key_inputs.key_d,
            0.0,
            self.key_inputs.key_w + self.key_inputs.key_s,
        );

        let pitch = MOUSE_X_SENSITIVITY * self.mouse_inputs.delta.y;
        let yaw = MOUSE_Y_SENSITIVITY * self.mouse_inputs.delta.x;

        self.mouse_inputs.delta = Vec2::ZERO;

        let movement_actions = simulation::state::receiver::action::MovementAction {
            direction,
            pitch,
            yaw,
        };

        let agent_action =
            simulation::state::receiver::action::AgentAction::Movement(movement_actions);
        let action = simulation::state::receiver::action::Action::Agent(agent_action);

        action
    }

    pub fn handle_window_event(
        &mut self,
        event: &WindowEvent,
    ) -> Option<simulation::state::receiver::action::Action> {
        match event {
            WindowEvent::CloseRequested => {
                let action = self.handle_close_requested();

                action
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let action = self.handle_keyboard_input(device_id, event, is_synthetic);

                action
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                let action = self.handle_mouse_input(device_id, state, button);

                action
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                let action = self.handle_mouse_wheel(device_id, delta, phase);

                action
            }
            _ => None,
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

    fn handle_close_requested(&self) -> Option<simulation::state::receiver::action::Action> {
        let world_action = simulation::state::receiver::action::WorldAction::Exit;
        let action = simulation::state::receiver::action::Action::World(world_action);

        Some(action)
    }

    fn handle_keyboard_input(
        &mut self,
        _device_id: &DeviceId,
        key_event: &KeyEvent,
        _is_synthetic: &bool,
    ) -> Option<simulation::state::receiver::action::Action> {
        let mut action_option = None;

        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                let world_action = simulation::state::receiver::action::WorldAction::Exit;
                let action = simulation::state::receiver::action::Action::World(world_action);

                action_option = Some(action);
            }
            PhysicalKey::Code(KeyCode::Digit1) => {
                if key_event.state == ElementState::Released {
                    let test_action = simulation::state::receiver::action::TestAction::Test1;
                    let action = simulation::state::receiver::action::Action::Test(test_action);

                    action_option = Some(action);
                }
            }
            PhysicalKey::Code(KeyCode::Digit2) => {
                if key_event.state == ElementState::Released {
                    let test_action = simulation::state::receiver::action::TestAction::Test2;
                    let action = simulation::state::receiver::action::Action::Test(test_action);

                    action_option = Some(action);
                }
            }
            PhysicalKey::Code(KeyCode::Digit3) => {
                if key_event.state == ElementState::Released {
                    let test_action = simulation::state::receiver::action::TestAction::Test3;
                    let action = simulation::state::receiver::action::Action::Test(test_action);

                    action_option = Some(action);
                }
            }
            PhysicalKey::Code(KeyCode::Digit4) => {
                if key_event.state == ElementState::Released {
                    let test_action = simulation::state::receiver::action::TestAction::Test4;
                    let action = simulation::state::receiver::action::Action::Test(test_action);

                    action_option = Some(action);
                }
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
                    let jump_action = simulation::state::receiver::action::JumpAction::Start;
                    let agent_action =
                        simulation::state::receiver::action::AgentAction::Jump(jump_action);
                    let action = simulation::state::receiver::action::Action::Agent(agent_action);

                    action_option = Some(action);
                } else if key_event.state == ElementState::Released {
                    let jump_action = simulation::state::receiver::action::JumpAction::End;
                    let agent_action =
                        simulation::state::receiver::action::AgentAction::Jump(jump_action);
                    let action = simulation::state::receiver::action::Action::Agent(agent_action);

                    action_option = Some(action);
                }
            }
            _ => (),
        }

        action_option
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
