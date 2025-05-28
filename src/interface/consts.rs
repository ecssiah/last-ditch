//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

use crate::{include_assets, interface::render::data::BlockData, simulation};
use once_cell::sync::Lazy;
use std::{collections::HashMap, f32::consts::PI, time::Duration};

pub const FULLSCREEN: bool = false;

pub const INTERFACE_FRAME_FREQUENCY: u64 = 60;
pub const INTERFACE_FRAME_DURATION: Duration =
    Duration::from_nanos(1_000_000_000 / INTERFACE_FRAME_FREQUENCY);

pub const WINDOW_TITLE: &str = "Last Ditch";
pub const WINDOW_WIDTH: u32 = 2560;
pub const WINDOW_HEIGHT: u32 = 1440;
pub const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
pub const WINDOW_CLEAR_COLOR: [f64; 4] = [0.8, 0.8, 0.8, 1.0];

pub const FOV: f32 = 45.0;
pub const FOV_RADIANS: f32 = FOV * PI / 180.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 100.0;

pub const MOUSE_Y_SENSITIVITY: f32 = 0.009;
pub const MOUSE_X_SENSITIVITY: f32 = 0.006;

const BLOCK_DATA_MAP_CONFIG: &str = include_assets!("config/interface/block_data_map.ron");

pub static BLOCK_DATA_MAP: Lazy<HashMap<simulation::world::block::Kind, BlockData>> =
    Lazy::new(|| {
        let block_data_list: Vec<BlockData> =
            ron::from_str::<Vec<BlockData>>(BLOCK_DATA_MAP_CONFIG)
                .expect("Failed to parse BlockData");

        block_data_list
            .into_iter()
            .map(|block_data| (block_data.kind, block_data))
            .collect()
    });
