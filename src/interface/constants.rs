//! Interface constants

use std::{f32::consts::PI, time::Duration};

pub const FULLSCREEN: bool = false;

pub const WORLD_TEXTURE_SIZE: usize = 64;
pub const POPULATION_TEXTURE_SIZE: usize = 128;

pub const INTERFACE_FRAME_FREQUENCY: u64 = 60;
pub const INTERFACE_FRAME_DURATION: Duration =
    Duration::from_nanos(1_000_000_000 / INTERFACE_FRAME_FREQUENCY);

pub const WINDOW_WIDTH: u32 = 2560;
pub const WINDOW_HEIGHT: u32 = 1440;
pub const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
pub const WINDOW_CLEAR_COLOR: [f64; 4] = [0.05, 0.3, 0.66, 1.0];

pub const TEXTURE_ATLAS_MAX: u32 = 4;
pub const TILE_ATLAS_WIDTH: u32 = 1024;
pub const TILE_ATLAS_HEIGHT: u32 = 1024;
pub const TILE_SIZE: u32 = 64;

pub const FOV: f32 = 45.0;
pub const FOV_RADIANS: f32 = FOV * PI / 180.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 200.0;

pub const MOUSE_SENSITIVITY: f32 = 0.2;

pub const OVERSEER_MESSAGE_LIMIT: usize = 500;
