pub mod edge;
pub mod node;
pub mod region;
pub mod transition;

pub use edge::Edge;
pub use node::Node;
pub use region::Region;
pub use transition::Transition;

use crate::simulation::world::World;
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
    pub depth: u32,
    pub solid_set_map: HashMap<IVec3, FixedBitSet>,
    pub levels: Vec<Level>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            depth: 1,
            solid_set_map: HashMap::new(),
            levels: Vec::new(),
        }
    }

    pub fn setup(&mut self, world: &World) {
        self.solid_set_map = Self::setup_solid_set_map(world);

        let mut level = Level::new();
        level.regions = Self::setup_regions(world);

        self.levels.push(level);
    }

    fn setup_solid_set_map(world: &World) -> HashMap<IVec3, FixedBitSet> {
        world
            .chunk_list
            .iter()
            .map(|chunk| {
                let mut chunk_solid_set =
                    FixedBitSet::with_capacity(world.grid.chunk_volume as usize);

                for block in &chunk.block_list {
                    chunk_solid_set.set(usize::from(block.id), block.solid);
                }

                let chunk_coordinates = world.grid.position_to_chunk_coordinates(chunk.position).unwrap();

                (chunk_coordinates, chunk_solid_set)
            })
            .collect()
    }

    fn setup_regions(world: &World) -> Vec<Region> {
        let chunk_radius = IVec3::splat(world.grid.chunk_radius as i32);

        world
            .chunk_list
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
