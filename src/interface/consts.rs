use crate::{include_assets, interface::render::data::BlockData, simulation};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const FULLSCREEN: bool = false;

pub const FRAME_DURATION: std::time::Duration = std::time::Duration::from_micros(1_000_000 / 30);

pub const WINDOW_TITLE: &str = "Last Ditch";
pub const WINDOW_WIDTH: u32 = 2560;
pub const WINDOW_HEIGHT: u32 = 1440;
pub const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
pub const WINDOW_CLEAR_COLOR: [f64; 4] = [0.5, 0.5, 0.5, 1.0];

pub const FOV: f32 = 45.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 100.0;

pub const MOUSE_Y_SENSITIVITY: f32 = 0.009;
pub const MOUSE_X_SENSITIVITY: f32 = 0.006;

const BLOCK_DATA_MAP_CONFIG: &str = include_assets!("config/interface/block_data_map_config.ron");

pub static BLOCK_DATA_MAP: Lazy<HashMap<simulation::block::Kind, BlockData>> = Lazy::new(|| {
    let block_data_list: Vec<BlockData> = ron::from_str::<Vec<BlockData>>(BLOCK_DATA_MAP_CONFIG)
        .expect("Failed to parse block_data_map.ron");

    block_data_list
        .into_iter()
        .map(|block_data| (block_data.kind, block_data))
        .collect()
});
