//! Interface constants

use std::{f32::consts::PI, time::Duration};

pub const FULLSCREEN: bool = false;

pub const DEBUG_RENDER: bool = false;

pub const INTERFACE_FRAME_FREQUENCY: u64 = 60;
pub const INTERFACE_FRAME_DURATION: Duration =
    Duration::from_nanos(1_000_000_000 / INTERFACE_FRAME_FREQUENCY);

pub const WINDOW_WIDTH: u32 = 2560;
pub const WINDOW_HEIGHT: u32 = 1440;
pub const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
pub const WINDOW_CLEAR_COLOR: [f64; 4] = [0.349, 0.620, 0.969, 1.0];

pub const FOV: f32 = 45.0;
pub const FOV_RADIANS: f32 = FOV * PI / 180.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 200.0;

pub const MOUSE_SENSITIVITY: f32 = 0.004;
