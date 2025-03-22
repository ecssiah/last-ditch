use crate::{
    include_config,
    simulation::{block, structure},
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const SEED: u64 = 101;
pub const SIMULATION_WAIT: u64 = 16;
pub const UPDATE_WINDOW: u32 = 2;

pub const DEFAULT_Z_SPEED: f32 = 22.0;
pub const DEFAULT_X_SPEED: f32 = 22.0;
pub const DEFAULT_ANGULAR_SPEED: f32 = 1.0;

pub const BLOCK_RADIUS: f32 = 0.5;
pub const BLOCK_SIZE: f32 = 2.0 * BLOCK_RADIUS;
pub const BLOCK_AREA: f32 = BLOCK_SIZE * BLOCK_SIZE;
pub const BLOCK_VOLUME: f32 = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE;

pub const CHUNK_RADIUS: usize = 1;
pub const CHUNK_SIZE: usize = 2 * CHUNK_RADIUS + 1;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_RADIUS: usize = 1;
pub const WORLD_SIZE: usize = 2 * WORLD_RADIUS + 1;
pub const WORLD_AREA: usize = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_VOLUME: usize = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub const WORLD_BOUNDARY: usize = CHUNK_RADIUS + WORLD_RADIUS * CHUNK_SIZE;

const BLOCK_CONFIG: &str = include_config!("blocks.ron");
const STRUCTURE_CONFIG: &str = include_config!("structures.ron");

pub static BLOCKS: Lazy<HashMap<block::Kind, block::Block>> = Lazy::new(|| {
    let list: Vec<block::Block> =
        ron::from_str::<Vec<block::Block>>(BLOCK_CONFIG).expect("Failed to parse Blocks");

    list.into_iter()
        .map(|block| (block.kind, block))
        .collect()
});

pub static STRUCTURES: Lazy<HashMap<structure::Kind, structure::Structure>> = Lazy::new(|| {
    let list: Vec<structure::Structure> =
        ron::from_str(STRUCTURE_CONFIG).expect("Failed to parse Structures");

    list.into_iter()
        .map(|structure| (structure.kind, structure))
        .collect()
});
