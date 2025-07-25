//! Simulation constants

use std::{f32::consts::FRAC_PI_2, time::Duration};

pub const TESTING: bool = true;

pub const PROJECT_TITLE: &str = "Last Ditch";

pub const SIMULATION_TICK_FREQUENCY: u64 = 20;
pub const SIMULATION_MAX_TICKS_PER_FRAME: u32 = 5;
pub const SIMULATION_TICK_DURATION: Duration =
    Duration::from_nanos(1_000_000_000 / SIMULATION_TICK_FREQUENCY);
pub const SIMULATION_TICK_IN_SECONDS: f32 = SIMULATION_TICK_DURATION.as_secs_f32();
pub const SIMULATION_TICK_IN_SECONDS_SQUARED: f32 =
    SIMULATION_TICK_IN_SECONDS * SIMULATION_TICK_IN_SECONDS;
pub const SIMULATION_MAX_ENTITIES: usize = 500;

pub const MOVEMENT_COST_STRAIGHT: u32 = 10;
pub const MOVEMENT_COST_DIAGONAL: u32 = 14;

pub const BLOCK_RADIUS: f32 = 0.5;
pub const BLOCK_SIZE: f32 = 2.0 * BLOCK_RADIUS;
pub const BLOCK_AREA: f32 = BLOCK_SIZE * BLOCK_SIZE;
pub const BLOCK_VOLUME: f32 = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE;

pub const AMBIENT_LIGHT_LEVELS: [f32; 3] = [0.3, 0.8, 1.0];

pub const GRAVITY_ACCELERATION: f32 = 16.0;
pub const EPSILON_COLLISION: f32 = 0.001;
pub const MAX_RESOLVE_ITERATIONS: usize = 40;
pub const MAXIMUM_CLEARANCE: u32 = 5;
pub const MINIMUM_CLEARANCE: u32 = 3;
pub const CONTACT_OFFSET: f32 = 0.001;
pub const MINIMUM_APPROACH_DISTANCE: f32 = f32::EPSILON * f32::EPSILON;

pub const JUDGE_SPEED_X: f32 = 6.0;
pub const JUDGE_SPEED_Y: f32 = 8.0;
pub const JUDGE_SPEED_Z: f32 = 8.0;
pub const JUDGE_SIZE_X: f32 = 0.6;
pub const JUDGE_SIZE_Y: f32 = 2.8;
pub const JUDGE_SIZE_Z: f32 = 0.6;
pub const JUDGE_VIEW_RADIUS: f32 = 256.0;
pub const JUDGE_VIEW_RADIUS_SQUARED: f32 = JUDGE_VIEW_RADIUS * JUDGE_VIEW_RADIUS;
pub const JUDGE_PITCH_LIMIT: f32 = FRAC_PI_2 + f32::EPSILON;
