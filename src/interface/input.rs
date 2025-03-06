use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent,
    },
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    simulation::action::{Action, WorldAction},
    ActionSender,
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
        &self,
        _device_id: &DeviceId,
        key_event: &KeyEvent,
        _is_synthetic: &bool,
    ) {
        match key_event.physical_key {
            PhysicalKey::Code(key_code) => match key_code {
                KeyCode::Escape => {
                    self.action_tx
                        .send(Action::World(WorldAction::Quit))
                        .unwrap();
                }
                _ => {}
            },
            _ => {}
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

    pub fn handle_cursor_moved(&self, _device_id: &DeviceId, position: &PhysicalPosition<f64>) {
        println!("{:?}", position);
    }
}
