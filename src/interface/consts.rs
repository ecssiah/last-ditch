use crate::{include_assets, interface::render::RenderBlock, simulation::block};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const FULLSCREEN: bool = true;

pub const WINDOW_TITLE: &str = "Last Ditch";
pub const WINDOW_WIDTH: u32 = 2560;
pub const WINDOW_HEIGHT: u32 = 1440;
pub const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
pub const WINDOW_CLEAR_COLOR: [f64; 4] = [0.1, 0.2, 0.36, 1.0];

pub const FOV: f32 = 45.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 100.0;

pub const MOUSE_Y_SENSITIVITY: f32 = 0.009;
pub const MOUSE_X_SENSITIVITY: f32 = 0.006;

const RENDER_BLOCKS_CONFIG: &str = include_assets!("config/interface/render_blocks.ron");

pub static RENDER_BLOCKS: Lazy<HashMap<block::Kind, RenderBlock>> = Lazy::new(|| {
    let list: Vec<RenderBlock> =
        ron::from_str::<Vec<RenderBlock>>(RENDER_BLOCKS_CONFIG).expect("Failed to parse Blocks");

    list.into_iter().map(|block| (block.kind, block)).collect()
});
