use crate::{include_assets, interface::render::gpu_block::GPUBlock, simulation};
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

const GPU_BLOCKS_CONFIG: &str = include_assets!("config/interface/gpu_blocks.ron");

pub static GPU_BLOCKS: Lazy<HashMap<simulation::block::Kind, GPUBlock>> = Lazy::new(|| {
    let gpu_blocks: Vec<GPUBlock> =
        ron::from_str::<Vec<GPUBlock>>(GPU_BLOCKS_CONFIG).expect("Failed to parse Blocks");

    gpu_blocks
        .into_iter()
        .map(|block| (block.kind, block))
        .collect()
});
