use crate::{
    include_config,
    simulation::{block, structure},
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, time::Duration};

pub const SEED: u64 = 128;

pub const FIXED_TICK_RATE: u32 = 60;
pub const FIXED_DT: Duration = Duration::from_nanos(1_000_000_000 / FIXED_TICK_RATE as u64);
pub const SIMULATION_WAIT_DURATION: Duration = Duration::from_micros(500);

pub const GRAVITY_ACCELERATION: f32 = -60.0;

pub const DEFAULT_Z_SPEED: f32 = 22.0;
pub const DEFAULT_X_SPEED: f32 = 22.0;
pub const DEFAULT_ANGULAR_SPEED: f32 = 1.0;
pub const MAX_JUMP_DURATION: Duration = Duration::from_millis(500);
pub const JUMP_LAUNCH_VELOCITY: f32 = 28.0;
pub const JUMP_HOLD_FORCE: f32 = 6.0;

pub const BLOCK_RADIUS: f32 = 0.5;
pub const BLOCK_SIZE: f32 = 2.0 * BLOCK_RADIUS;
pub const BLOCK_AREA: f32 = BLOCK_SIZE * BLOCK_SIZE;
pub const BLOCK_VOLUME: f32 = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE;

pub const CHUNK_RADIUS: usize = 6;
pub const CHUNK_SIZE: usize = 2 * CHUNK_RADIUS + 1;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_RADIUS: usize = 6;
pub const WORLD_SIZE: usize = 2 * WORLD_RADIUS + 1;
pub const WORLD_AREA: usize = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_VOLUME: usize = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub const WORLD_BOUNDARY: usize = CHUNK_RADIUS + WORLD_RADIUS * CHUNK_SIZE;

pub const AMBIENT_OCCLUSION_LEVEL: [f32; 3] = [1.0, 0.6, 0.2];

const BLOCK_CONFIG: &str = include_config!("blocks.ron");
const STRUCTURE_CONFIG: &str = include_config!("structures.ron");

pub static BLOCKS: Lazy<HashMap<block::Kind, block::Block>> = Lazy::new(|| {
    let list: Vec<block::Block> =
        ron::from_str::<Vec<block::Block>>(BLOCK_CONFIG).expect("Failed to parse Blocks");

    list.into_iter().map(|block| (block.kind, block)).collect()
});

pub static STRUCTURES: Lazy<HashMap<structure::Kind, structure::Structure>> = Lazy::new(|| {
    let list: Vec<structure::Structure> =
        ron::from_str(STRUCTURE_CONFIG).expect("Failed to parse Structures");

    list.into_iter()
        .map(|structure| (structure.kind, structure))
        .collect()
});
