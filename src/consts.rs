pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

pub const FOV: f32 = 45.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 100.0;

pub const CHUNK_DIM: usize = 16;
pub const CHUNK_SIZE: usize = CHUNK_DIM * CHUNK_DIM * CHUNK_DIM;
pub const CHUNK_HALF: isize = (CHUNK_DIM / 2) as isize;