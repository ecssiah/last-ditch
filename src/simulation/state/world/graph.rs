pub mod edge;
pub mod node;
pub mod region;
pub mod transition;

pub use edge::Edge;
pub use node::Node;
pub use region::Region;
pub use transition::Transition;

use crate::simulation::state::world::{chunk::Chunk, grid::Grid};
use fixedbitset::FixedBitSet;
use glam::IVec3;
use std::collections::HashMap;

pub struct Level {
    pub nodes: HashMap<IVec3, Node>,
    pub edges: HashMap<(IVec3, IVec3), Edge>,
    pub regions: Vec<Region>,
    pub transitions: Vec<Transition>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            regions: Vec::new(),
            transitions: Vec::new(),
        }
    }
}

pub struct Graph {
    pub depth: usize,
    pub grid: Grid,
    pub solid_set_map: HashMap<IVec3, FixedBitSet>,
    pub levels: Vec<Level>,
}

impl Graph {
    pub fn new(grid: &Grid, depth: usize) -> Self {
        Self {
            depth,
            grid: *grid,
            solid_set_map: HashMap::new(),
            levels: Vec::with_capacity(depth),
        }
    }

    pub fn setup(&mut self, chunk_list: &Vec<Chunk>) {
        self.solid_set_map = Self::setup_solid_set_map(&self.grid, chunk_list);

        let base_level = Self::setup_base_level(&self.grid, chunk_list);

        self.levels.push(base_level);

        for level_number in 1..(self.depth - 1).max(0) {
            let level = Self::setup_level(&self.grid, level_number);

            self.levels.push(level);
        }
    }

    fn setup_solid_set_map(grid: &Grid, chunk_list: &Vec<Chunk>) -> HashMap<IVec3, FixedBitSet> {
        chunk_list
            .iter()
            .map(|chunk| {
                let mut chunk_solid_set = FixedBitSet::with_capacity(grid.chunk_volume as usize);

                for block in &chunk.block_list {
                    chunk_solid_set.set(usize::from(block.id), block.solid);
                }

                let chunk_coordinates = grid.position_to_chunk_coordinates(chunk.position).unwrap();

                (chunk_coordinates, chunk_solid_set)
            })
            .collect()
    }

    fn setup_base_level(grid: &Grid, chunk_list: &Vec<Chunk>) -> Level {
        let mut base_level = Level::new();
        base_level.regions = Self::setup_regions(grid, chunk_list);

        base_level
    }

    fn setup_level(grid: &Grid, level_number: usize) -> Level {
        let mut level = Level::new();

        level
    }

    fn setup_regions(grid: &Grid, chunk_list: &Vec<Chunk>) -> Vec<Region> {
        let chunk_radius = IVec3::splat(grid.chunk_radius as i32);

        chunk_list
            .iter()
            .map(|chunk| {
                let min = chunk.position - chunk_radius;
                let max = chunk.position + chunk_radius;

                Region { min, max }
            })
            .collect()
    }

    fn setup_transitions(&mut self) {}

    fn add_node(&mut self) {}

    fn add_edge(&mut self) {}

    fn add_region(&mut self) {}

    fn add_transition(&mut self) {}
}
