//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod action;
pub mod agent;
pub mod block;
pub mod chunk;
pub mod state;
pub mod structure;
pub mod world;

use crate::include_config;
use action::{Action, AgentAction, MoveActions, RotateActions, WorldAction};
use agent::Agent;
pub use block::Block;
use block::{Direction, NeighborMask};
pub use chunk::Chunk;
use glam::{IVec3, Quat, Vec3};
use log::info;
use once_cell::sync::Lazy;
use ron::from_str;
use state::State;
use std::collections::HashMap;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};
use structure::Structure;
use tokio::sync::mpsc::UnboundedReceiver;
use world::World;

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

pub const CHUNK_RADIUS: u32 = 4;
pub const CHUNK_SIZE: u32 = 2 * CHUNK_RADIUS + 1;
pub const CHUNK_AREA: u32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: u32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_RADIUS: u32 = 4;
pub const WORLD_SIZE: u32 = 2 * WORLD_RADIUS + 1;
pub const WORLD_AREA: u32 = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_VOLUME: u32 = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub const WORLD_BOUNDARY: u32 = CHUNK_RADIUS + WORLD_RADIUS * CHUNK_SIZE;

const BLOCK_CONFIG: &str = include_config!("blocks.ron");
const STRUCTURE_CONFIG: &str = include_config!("structures.ron");

pub static BLOCKS: Lazy<Vec<Block>> = Lazy::new(|| {
    let mut blocks: Vec<Block> = from_str(BLOCK_CONFIG).expect("Failed to parse Blocks");
    blocks.sort_by(|a, b| a.kind.cmp(&b.kind));

    blocks
});

pub static STRUCTURES: Lazy<HashMap<structure::Kind, Structure>> =
    Lazy::new(|| from_str(STRUCTURE_CONFIG).expect("Failed to parse Structures"));

pub struct Simulation {
    action_rx: UnboundedReceiver<Action>,
    state: Arc<State>,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let state = Arc::from(State {
            agent: Self::setup_agent(),
            world: Self::setup_world(),
            chunks: Self::setup_chunks(),
        });

        let simulation = Self { action_rx, state };

        info!("Simulation Initialized");

        simulation
    }

    fn setup_agent() -> Arc<RwLock<Agent>> {
        let agent = Arc::from(RwLock::from(Agent {
            id: 0,
            name: "Melchizedek",
            position: Vec3::new(0.0, 6.0, -12.0),
            x_speed: 0.0,
            z_speed: 0.0,
            look_x_axis: 0.0,
            look_y_axis: 0.0,
            look_rotation: Quat::IDENTITY,
        }));

        agent
    }

    fn setup_world() -> Arc<RwLock<World>> {
        let world = Arc::from(RwLock::from(World {
            active: true,
            seed: SEED,
            time: 0.0,
            ticks: 1,
            last_update: 1,
        }));

        world
    }

    fn setup_chunks() -> Arc<[Arc<RwLock<Chunk>>]> {
        let chunks: [Arc<RwLock<Chunk>>; WORLD_VOLUME as usize] =
            core::array::from_fn(|chunk_id| {
                Arc::from(RwLock::from(Chunk {
                    last_update: 1,
                    position: Self::chunk_id_to_position(chunk_id as u32),
                    palette: Vec::from([block::Kind::Air]),
                    palette_ids: Box::new([0; CHUNK_VOLUME as usize]),
                    meta: Box::new([block::Meta::default(); CHUNK_VOLUME as usize]),
                    light_map: Box::new([block::LightLevel::default(); CHUNK_VOLUME as usize]),
                }))
            });

        Arc::from(chunks)
    }

    pub fn generate(&mut self) {
        // self.generate_structure(IVec3::new(-12, 0, 0), structure::Kind::Mario);
        // self.generate_structure(IVec3::new(12, 0, 0), structure::Kind::Luigi);

        self.generate_structure(IVec3::new(0, 0, 0), structure::Kind::LightTest);

        // self.set_block_kind(IVec3::new(0, 0, 0), block::Kind::Concrete);
        // self.set_block_kind(IVec3::new(1, 0, 0), block::Kind::Concrete);
        // self.set_block_kind(IVec3::new(0, 0, 1), block::Kind::Concrete);
        // self.set_block_kind(IVec3::new(1, 0, 1), block::Kind::Concrete);
        // self.set_block_kind(IVec3::new(1, 1, 1), block::Kind::Concrete);
    }

    fn generate_structure(&mut self, world_position: IVec3, structure_kind: structure::Kind) {
        if let Some(structure) = STRUCTURES.get(&structure_kind) {
            for block_data in &structure.blocks[..] {
                let block_position = IVec3::new(
                    block_data.position[0],
                    block_data.position[1],
                    block_data.position[2],
                );

                let grid_position = world_position + block_position;

                self.set_block_kind(grid_position, block_data.kind);
            }
        }
    }

    pub fn get_chunk(&self, grid_position: IVec3) -> Option<Arc<RwLock<chunk::Chunk>>> {
        if let Some(chunk_id) = Self::grid_position_to_chunk_id(grid_position) {
            Some(self.state.chunks[chunk_id as usize].clone())
        } else {
            None
        }
    }

    pub fn get_block(&self, grid_position: IVec3) -> Option<&block::Block> {
        if let Some((chunk_id, block_id)) = Self::grid_position_to_ids(grid_position) {
            let chunk = self.state.chunks[chunk_id as usize].write().unwrap();

            let palette_id = chunk.palette_ids[block_id as usize];
            let kind = chunk.palette[palette_id as usize];

            Some(&BLOCKS[kind as usize])
        } else {
            None
        }
    }

    pub fn get_meta(&self, grid_position: IVec3) -> Option<block::Meta> {
        if let Some((chunk_id, block_id)) = Self::grid_position_to_ids(grid_position) {
            let chunk = self.state.chunks[chunk_id as usize].write().unwrap();

            let meta = chunk.meta[block_id as usize];

            Some(meta)
        } else {
            None
        }
    }

    fn set_block_kind(&mut self, grid_position: IVec3, kind: block::Kind) {
        if let Some((chunk_id, block_id)) = Self::grid_position_to_ids(grid_position) {
            {
                let mut chunk = self.state.chunks[chunk_id as usize].write().unwrap();

                let palette_id = self.get_and_insert_palette_id(&mut chunk, kind);
                chunk.palette_ids[block_id as usize] = palette_id;
            }

            self.update_block_meta(chunk_id, block_id, grid_position);
            self.update_light_map(chunk_id, block_id, grid_position);

            self.mark_chunk_update(chunk_id);
        }
    }

    fn mark_chunk_update(&mut self, chunk_id: u32) {
        let mut world = self.state.world.write().unwrap();
        let mut chunk = self.state.chunks[chunk_id as usize].write().unwrap();

        chunk.last_update = world.ticks;
        world.last_update = world.ticks;
    }

    fn get_and_insert_palette_id(&self, chunk: &mut Chunk, kind: block::Kind) -> u32 {
        match chunk
            .palette
            .iter()
            .position(|palette_kind| kind == *palette_kind)
        {
            Some(id) => id as u32,
            None => {
                chunk.palette.push(kind.clone());

                let id = chunk.palette.len() - 1;
                id as u32
            }
        }
    }

    fn update_block_meta(&mut self, chunk_id: u32, block_id: u32, grid_position: IVec3) {
        let neighbor_mask = self.compute_neighbor_mask(grid_position);

        {
            let mut chunk = self.state.chunks[chunk_id as usize].write().unwrap();

            chunk.meta[block_id as usize].neighbor_mask = neighbor_mask;
        }

        let mut neighbor_mask_updates = Vec::new();

        for offset in Direction::OFFSETS {
            if offset == Direction::CCC.offset() {
                continue;
            }

            let neighbor_grid_position = grid_position + offset;

            if let Some((chunk_id, block_id)) = Self::grid_position_to_ids(neighbor_grid_position) {
                let neighbor_mask = self.compute_neighbor_mask(neighbor_grid_position);

                neighbor_mask_updates.push((chunk_id, block_id, neighbor_mask));
            }
        }

        for (chunk_id, block_id, neighbor_mask) in neighbor_mask_updates {
            let mut chunk = self.state.chunks[chunk_id as usize].write().unwrap();

            chunk.meta[block_id as usize].neighbor_mask = neighbor_mask;
        }
    }

    fn compute_neighbor_mask(&mut self, grid_position: IVec3) -> NeighborMask {
        let mut neighbor_mask = NeighborMask::NONE;

        for index in 0..Direction::OFFSETS.len() {
            if index == Direction::CCC.index() {
                continue;
            }

            let offset = Direction::OFFSETS[index];

            let neighbor_mask_grid_position = grid_position + offset;

            if let Some(block) = self.get_block(neighbor_mask_grid_position) {
                if block.solid {
                    if let Some(direction) = Direction::bit(index) {
                        neighbor_mask.set_solid(direction, true);
                    }
                }
            }
        }

        neighbor_mask
    }

    fn update_light_map(&mut self, chunk_id: u32, block_id: u32, grid_position: IVec3) {}

    pub fn run(&mut self) {
        let mut previous_instant = Instant::now();

        loop {
            let now = Instant::now();

            let dt = now.duration_since(previous_instant).as_secs_f64();
            previous_instant = now;

            self.update(dt);

            thread::sleep(Duration::from_millis(SIMULATION_WAIT));
        }
    }

    fn update(&mut self, dt: f64) {
        self.process_actions();
        self.evolve(dt);
    }

    pub fn get_state(&self) -> Arc<State> {
        self.state.clone()
    }

    fn process_actions(&mut self) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::World(WorldAction::Quit) => {
                    self.process_quit_action();
                }
                Action::Agent(AgentAction::Move(move_actions)) => {
                    self.process_move_actions(&move_actions);
                }
                Action::Agent(AgentAction::Rotate(rotate_actions)) => {
                    self.process_rotate_actions(&rotate_actions);
                }
            }
        }
    }

    fn process_quit_action(&mut self) {
        let mut world = self.state.world.write().unwrap();

        world.active = false;
    }

    fn process_move_actions(&mut self, move_actions: &MoveActions) {
        let mut agent = self.state.agent.write().unwrap();

        agent.z_speed = move_actions.z_axis;
        agent.x_speed = move_actions.x_axis;
    }

    fn process_rotate_actions(&mut self, rotate_actions: &RotateActions) {
        let mut agent = self.state.agent.write().unwrap();

        agent.look_x_axis -= rotate_actions.x_axis;
        agent.look_y_axis += rotate_actions.y_axis;

        let limit = 89.0_f32.to_radians();

        agent.look_x_axis = agent.look_x_axis.clamp(-limit, limit);

        let y_axis_quat = Quat::from_rotation_y(agent.look_y_axis);
        let x_axis_quat = Quat::from_rotation_x(agent.look_x_axis);

        let target_rotation = y_axis_quat * x_axis_quat;

        agent.look_rotation = agent.look_rotation.slerp(target_rotation, 0.3);
    }

    fn evolve(&mut self, dt: f64) {
        self.evolve_world(dt);
        self.evolve_agents(dt);
    }

    fn evolve_world(&mut self, dt: f64) {
        let mut state = self.state.world.write().unwrap();

        state.time += dt;
        state.ticks += 1;
    }

    fn evolve_agents(&mut self, dt: f64) {
        let mut agent = self.state.agent.write().unwrap();

        let y_axis_quat = Quat::from_rotation_y(agent.look_y_axis);

        let x_axis = y_axis_quat * Vec3::X;
        let z_axis = y_axis_quat * Vec3::Z;

        let movement = agent.x_speed * x_axis + agent.z_speed * z_axis;

        agent.position += dt as f32 * movement;
    }

    fn chunk_id_to_position(chunk_id: u32) -> IVec3 {
        let chunk_position_shifted = IVec3::new(
            (chunk_id % WORLD_SIZE) as i32,
            (chunk_id / WORLD_SIZE % WORLD_SIZE) as i32,
            (chunk_id / WORLD_AREA) as i32,
        );

        let chunk_position = chunk_position_shifted - IVec3::splat(WORLD_RADIUS as i32);

        chunk_position
    }

    fn block_id_to_position(block_id: u32) -> IVec3 {
        let block_position_shifted = IVec3::new(
            (block_id % CHUNK_SIZE) as i32,
            (block_id / CHUNK_SIZE % CHUNK_SIZE) as i32,
            (block_id / CHUNK_AREA) as i32,
        );

        let block_position = block_position_shifted - IVec3::splat(CHUNK_RADIUS as i32);

        block_position
    }

    pub fn grid_position_to_chunk_id(grid_position: IVec3) -> Option<u32> {
        if Self::is_on_map(grid_position) {
            let chunk_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.div_euclid(WORLD_SIZE as i32)
            });

            let chunk_id = chunk_position_shifted.x
                + chunk_position_shifted.y * WORLD_SIZE as i32
                + chunk_position_shifted.z * WORLD_AREA as i32;

            Some(chunk_id as u32)
        } else {
            None
        }
    }

    pub fn grid_position_to_block_id(grid_position: IVec3) -> Option<u32> {
        if Self::is_on_map(grid_position) {
            let grid_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.rem_euclid(CHUNK_SIZE as i32)
            });

            let block_id = grid_position_shifted.x
                + grid_position_shifted.y * CHUNK_SIZE as i32
                + grid_position_shifted.z * CHUNK_AREA as i32;

            Some(block_id as u32)
        } else {
            None
        }
    }

    pub fn grid_position_to_ids(grid_position: IVec3) -> Option<(u32, u32)> {
        if Self::is_on_map(grid_position) {
            let chunk_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.div_euclid(WORLD_SIZE as i32)
            });

            let chunk_id = chunk_position_shifted.x
                + chunk_position_shifted.y * WORLD_SIZE as i32
                + chunk_position_shifted.z * WORLD_AREA as i32;

            let grid_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.rem_euclid(CHUNK_SIZE as i32)
            });

            let block_id = grid_position_shifted.x
                + grid_position_shifted.y * CHUNK_SIZE as i32
                + grid_position_shifted.z * CHUNK_AREA as i32;

            Some((chunk_id as u32, block_id as u32))
        } else {
            None
        }
    }

    pub fn get_grid_position(chunk_id: u32, block_id: u32) -> IVec3 {
        let chunk_position = Self::chunk_id_to_position(chunk_id);
        let block_position = Self::block_id_to_position(block_id);

        let grid_position = CHUNK_SIZE as i32 * chunk_position + block_position;

        grid_position
    }

    pub fn is_on_map(grid_position: IVec3) -> bool {
        let in_x_range = grid_position.x.abs() <= WORLD_BOUNDARY as i32;
        let in_y_range = grid_position.y.abs() <= WORLD_BOUNDARY as i32;
        let in_z_range = grid_position.z.abs() <= WORLD_BOUNDARY as i32;

        in_x_range && in_y_range && in_z_range
    }
}
