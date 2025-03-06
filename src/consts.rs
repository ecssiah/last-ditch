pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

pub const FOV: f32 = 45.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 100.0;

pub const DEFAULT_SEED: u64 = 101;
pub const SIMULATION_SLEEP: u64 = 16;

pub const WORLD_RADIUS: u64 = 2;
pub const WORLD_SIZE: u64 = 2 * WORLD_RADIUS + 1;
pub const WORLD_AREA: u64 = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_VOLUME: u64 = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub const CHUNK_RADIUS: u64 = 2;
pub const CHUNK_SIZE: u64 = 2 * CHUNK_RADIUS + 1;
pub const CHUNK_AREA: u64 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: u64 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
