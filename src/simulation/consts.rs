use crate::{
    include_assets,
    simulation::{block, structure},
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, time::Duration};

pub const DEFAULT_SEED: u64 = 128;

pub const FIXED_TICK_RATE: u32 = 30;
pub const FIXED_DT: Duration = Duration::from_nanos(1_000_000_000 / FIXED_TICK_RATE as u64);
pub const SIMULATION_WAIT_DURATION: Duration = Duration::from_micros(500);
pub const SETTLEMENT_PERIOD: u32 = 10;

pub const GRAVITY_ACCELERATION: f32 = -22.8;

pub const DEFAULT_X_SPEED: f32 = 7.0;
pub const DEFAULT_Z_SPEED: f32 = 7.0;
pub const DEFAULT_ANGULAR_SPEED: f32 = 1.0;
pub const MAX_JUMP_TICKS: u32 = 8;
pub const JUMP_LAUNCH_VELOCITY: f32 = 8.0;
pub const JUMP_HOLD_VELOCITY: f32 = 1.0;
pub const JUMP_GRAVITY_DAMPING: f32 = 0.5;

pub const JUDGE_VIEW_RADIUS: usize = 12;
pub const AGENT_INITIAL_POPULATION: usize = 64;

pub const ENTITY_SIZE_X: f32 = 0.4;
pub const ENTITY_SIZE_Y: f32 = 1.4;
pub const ENTITY_SIZE_Z: f32 = 0.4;
pub const ENTITY_BORDER_RADIUS: f32 = 0.001;

pub const BLOCK_RADIUS: f32 = 0.5;
pub const BLOCK_SIZE: f32 = 2.0 * BLOCK_RADIUS;
pub const BLOCK_AREA: f32 = BLOCK_SIZE * BLOCK_SIZE;
pub const BLOCK_VOLUME: f32 = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE;

pub const CHUNK_RADIUS: usize = 4;
pub const CHUNK_SIZE: usize = 2 * CHUNK_RADIUS + 1;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_RADIUS: usize = 4;
pub const WORLD_SIZE: usize = 2 * WORLD_RADIUS + 1;
pub const WORLD_AREA: usize = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_VOLUME: usize = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub const WORLD_BOUNDARY: usize = CHUNK_RADIUS + WORLD_RADIUS * CHUNK_SIZE;

pub const JUDGE_CAMERA_HEIGHT: f32 = 1.6;
pub const WORLD_VIEW_RADIUS: i32 = 12;
pub const POPULATION_VIEW_RADIUS: f32 = (CHUNK_SIZE as i32 * (WORLD_VIEW_RADIUS - 2)) as f32;

pub const AMBIENT_LIGHT_LEVEL: [f32; 3] = [0.3, 0.8, 1.0];

const BLOCKS_CONFIG: &str = include_assets!("config/simulation/blocks.ron");
const STRUCTURES_CONFIG: &str = include_assets!("config/simulation/structures.ron");

pub static BLOCKS: Lazy<HashMap<block::Kind, block::Block>> = Lazy::new(|| {
    let list: Vec<block::Block> =
        ron::from_str::<Vec<block::Block>>(BLOCKS_CONFIG).expect("Failed to parse Blocks");

    list.into_iter().map(|block| (block.kind, block)).collect()
});

pub static STRUCTURES: Lazy<HashMap<structure::Kind, structure::Structure>> = Lazy::new(|| {
    let list: Vec<structure::Structure> =
        ron::from_str(STRUCTURES_CONFIG).expect("Failed to parse Structures");

    list.into_iter()
        .map(|structure| (structure.kind, structure))
        .collect()
});
