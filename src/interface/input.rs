use std::sync::Arc;

use winit::{
    dpi::PhysicalPosition, event::{
        DeviceId, ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase,
    },
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct Input {
    window: Arc<Window>,
}

impl Input {
    pub async fn new(window: Arc<Window>) -> Input {
        Input {
            window
        }
    }

    pub fn handle_event(&self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
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
        device_id: DeviceId,
        key_event: KeyEvent,
        is_synthetic: bool,
    ) {
        println!("{:?} {:?}", key_event.physical_key, key_event.state);
    }

    pub fn handle_mouse_input(
        &self,
        device_id: DeviceId,
        state: ElementState,
        button: MouseButton,
    ) {
        println!("{:?} {:?}", state, button);
    }

    pub fn handle_mouse_wheel(
        &self,
        device_id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    ) {
        println!("{:?} {:?}", delta, phase);
    }
    
    pub fn handle_cursor_moved(&self, device_id: DeviceId, position: PhysicalPosition<f64>) {
        println!("{:?}", position);
    }
}