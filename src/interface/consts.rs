use crate::{include_assets, interface::render, simulation::block};
use once_cell::sync::Lazy;
use ron::from_str;
use std::collections::HashMap;

pub const WINDOW_TITLE: &str = "Last Ditch";

pub const FULLSCREEN: bool = true;

pub const WINDOW_WIDTH: u32 = 2560;
pub const WINDOW_HEIGHT: u32 = 1440;
pub const ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

pub const FOV: f32 = 45.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 100.0;

pub const MOUSE_Y_SENSITIVITY: f32 = 0.009;
pub const MOUSE_X_SENSITIVITY: f32 = 0.006;

pub const DEBUG_COLOR: bool = false;
pub const CLEAR_COLOR: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

const BLOCK_UVS_CONFIG: &str = include_assets!("config/interface/block_uvs.ron");

pub static BLOCK_UVS: Lazy<HashMap<block::Kind, render::BlockUV>> = Lazy::new(|| {
    let blocks: Vec<render::BlockUVConfig> =
        from_str(BLOCK_UVS_CONFIG).expect("Failed to parse block_uvs.ron");

    blocks
        .into_iter()
        .map(|block_uv_config| {
            let block_uv =
                render::BlockUV::try_from(block_uv_config).expect("Invalid block UV definition");
            (block_uv.kind.clone(), block_uv)
        })
        .collect()
});
