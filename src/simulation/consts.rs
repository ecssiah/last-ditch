//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

use crate::{include_assets, simulation::world::block};
use once_cell::sync::Lazy;
use std::{collections::HashMap, time::Duration};

pub const DEFAULT_SEED: u64 = 128;

pub const PROJECT_TITLE: &str = "Last Ditch";
pub const PROJECT_VERSION: &str = "0.1.0";

pub const SIMULATION_TICK_FREQUENCY: u64 = 20;
pub const SIMULATION_TICK_DURATION: Duration =
    Duration::from_nanos(1_000_000_000 / SIMULATION_TICK_FREQUENCY);
pub const SIMULATION_TICK_IN_SECONDS: f32 = SIMULATION_TICK_DURATION.as_secs_f32();
pub const SIMULATION_TICK_IN_SECONDS_SQUARED: f32 =
    SIMULATION_TICK_IN_SECONDS * SIMULATION_TICK_IN_SECONDS;
pub const MAX_RESOLVE_ITERATIONS: usize = 40;

pub const GRAVITY_ACCELERATION: f32 = 16.0;
pub const EPSILON_COLLISION: f32 = 0.001;

pub const DEFAULT_X_SPEED: f32 = 11.0;
pub const DEFAULT_Z_SPEED: f32 = 11.0;
pub const DEFAULT_ANGULAR_SPEED: f32 = 1.0;
pub const MAX_JUMP_TICKS: u32 = 8;
pub const JUMP_LAUNCH_VELOCITY: f32 = 8.0;
pub const JUMP_HOLD_VELOCITY: f32 = 1.0;
pub const JUMP_GRAVITY_DAMPING: f32 = 0.5;

pub const AGENT_INITIAL_POPULATION: usize = 16;

pub const ENTITY_SIZE_X: f32 = 0.4;
pub const ENTITY_SIZE_Y: f32 = 1.4;
pub const ENTITY_SIZE_Z: f32 = 0.4;
pub const ENTITY_BORDER_RADIUS: f32 = 0.001;

pub const WORLD_CARDINAL_COST: f32 = 1.0;
pub const WORLD_EDGE_COST: f32 = 1.4142135;
pub const WORLD_CORNER_COST: f32 = 1.7320508;

pub const WORLD_RADIUS: usize = 4;
pub const CHUNK_RADIUS: usize = 8;

pub const BLOCK_RADIUS: f32 = 0.5;
pub const BLOCK_SIZE: f32 = 2.0 * BLOCK_RADIUS;
pub const BLOCK_AREA: f32 = BLOCK_SIZE * BLOCK_SIZE;
pub const BLOCK_VOLUME: f32 = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE;

pub const JUDGE_CAMERA_HEIGHT: f32 = 1.4;
pub const JUDGE_VIEW_RADIUS: i32 = 12;
pub const JUDGE_VIEW_X_LIMIT: f32 = 1.5533;

pub const POPULATION_VIEW_RADIUS: f32 = 120.0;
pub const POPULATION_VIEW_RADIUS_SQUARED: f32 = POPULATION_VIEW_RADIUS * POPULATION_VIEW_RADIUS;

pub const MAXIMUM_CLEARANCE_CHECK: i32 = 4;

pub const AMBIENT_LIGHT_LEVELS: [f32; 3] = [0.3, 0.8, 1.0];

const BLOCK_MAP_CONFIG: &str = include_assets!("config/simulation/block_map.ron");

pub static BLOCK_MAP: Lazy<HashMap<block::Kind, block::Block>> = Lazy::new(|| {
    let block_list: Vec<block::Block> = ron::from_str::<Vec<block::Block>>(BLOCK_MAP_CONFIG)
        .expect("Failed to parse block_map_config.ron");

    block_list
        .into_iter()
        .map(|block| (block.kind, block))
        .collect()
});
