//! Simulation constants

pub const PROJECT_TITLE: &str = "Last Ditch";

pub const TESTING: bool = true;

pub const SQRT_2: f32 = std::f32::consts::SQRT_2;
pub const SQRT_3: f32 = 1.7320508;

pub const CELL_RADIUS: f32 = 0.5;

pub const SIMULATION_TICK_FREQUENCY: u64 = 20;
pub const SIMULATION_MAX_TICKS_PER_FRAME: u32 = 5;
pub const SIMULATION_TICK_DURATION: std::time::Duration =
    std::time::Duration::from_nanos(1_000_000_000 / SIMULATION_TICK_FREQUENCY);
pub const SIMULATION_TICK_IN_SECONDS: f32 = SIMULATION_TICK_DURATION.as_secs_f32();
pub const SIMULATION_TICK_IN_SECONDS_SQUARED: f32 =
    SIMULATION_TICK_IN_SECONDS * SIMULATION_TICK_IN_SECONDS;
pub const SIMULATION_MAX_ENTITIES: usize = 500;

pub const MOVEMENT_COST_STRAIGHT: u32 = 100;
pub const MOVEMENT_COST_DIAGONAL: u32 = 141;
pub const MOVEMENT_COST_CORNER: u32 = 173;

pub const GRAVITY_ACCELERATION: f32 = 60.0;
pub const EPSILON_COLLISION: f32 = 0.001;
pub const MAX_RESOLVE_ITERATIONS: usize = 40;
pub const MAXIMUM_CLEARANCE: u32 = 5;
pub const MINIMUM_CLEARANCE: u32 = 3;
pub const CONTACT_OFFSET: f32 = 0.001;
pub const MINIMUM_APPROACH_DISTANCE: f32 = f32::EPSILON * f32::EPSILON;

pub const JUDGE_DEFAULT_SPEED_X: f32 = 6.0;
pub const JUDGE_DEFAULT_SPEED_Y: f32 = 8.0;
pub const JUDGE_DEFAULT_SPEED_Z: f32 = 16.0;

pub const JUDGE_DEFAULT_SIZE_X: f32 = 0.6;
pub const JUDGE_DEFAULT_SIZE_Y: f32 = 0.6;
pub const JUDGE_DEFAULT_SIZE_Z: f32 = 2.2;

pub const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 + f32::EPSILON;

pub const AGENT_INITIAL_POPULATION: i32 = 16;

pub const AGENT_DEFAULT_SPEED_X: f32 = 6.0;
pub const AGENT_DEFAULT_SPEED_Y: f32 = 8.0;
pub const AGENT_DEFAULT_SPEED_Z: f32 = 8.0;

pub const AGENT_DEFAULT_SIZE_X: f32 = 0.6;
pub const AGENT_DEFAULT_SIZE_Y: f32 = 0.6;
pub const AGENT_DEFAULT_SIZE_Z: f32 = 1.73;
