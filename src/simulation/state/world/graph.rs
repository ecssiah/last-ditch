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
    pub node_list: HashMap<IVec3, Node>,
    pub edge_list: HashMap<(IVec3, IVec3), Edge>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            node_list: HashMap::new(),
            edge_list: HashMap::new(),
        }
    }
}

pub struct Graph {
    pub depth: usize,
    pub grid: Grid,
    pub solid_set_map: HashMap<IVec3, FixedBitSet>,
    pub region_list: Vec<Region>,
    pub level_list: Vec<Level>,
}

impl Graph {
    pub fn new(grid: &Grid, depth: usize) -> Self {
        Self {
            depth,
            grid: *grid,
            solid_set_map: HashMap::new(),
            region_list: Vec::new(),
            level_list: Vec::with_capacity(depth),
        }
    }

    pub fn setup(&mut self, chunk_list: &Vec<Chunk>) {
        self.solid_set_map = Self::setup_solid_set_map(&self.grid, chunk_list);
        self.region_list = Self::setup_regions(&self.grid, chunk_list);

        self.setup_nodes();
    }

    fn setup_solid_set_map(grid: &Grid, chunk_list: &Vec<Chunk>) -> HashMap<IVec3, FixedBitSet> {
        chunk_list
            .iter()
            .map(|chunk| {
                let mut solid_set = FixedBitSet::with_capacity(grid.chunk_volume as usize);

                for block in &chunk.block_list {
                    solid_set.set(usize::from(block.id), block.solid);
                }

                let chunk_coordinates = grid.position_to_chunk_coordinates(chunk.position).unwrap();

                (chunk_coordinates, solid_set)
            })
            .collect()
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

    fn is_solid(&self, position: IVec3) -> bool {
        if let Some(chunk_coordinates) = self.grid.position_to_chunk_coordinates(position) {
            if let Some(solid_set) = self.solid_set_map.get(&chunk_coordinates) {
                if let Some(block_id) = self.grid.position_to_block_id(position) {
                    return solid_set.contains(usize::from(block_id));
                }
            }
        }

        true
    }

    fn setup_nodes(&mut self) {
        let chunk_radius = self.grid.chunk_radius as i32;
        let world_radius = self.grid.world_radius as i32;

        for cx in -world_radius..=world_radius - 1 {
            for cy in -world_radius..=world_radius - 1 {
                for cz in -world_radius..=world_radius - 1 {
                    let chunk_coordinates = IVec3::new(cx, cy, cz);
                    let chunk_position = self
                        .grid
                        .chunk_coordinates_to_position(chunk_coordinates)
                        .unwrap();

                    for by in -chunk_radius..=chunk_radius {
                        for bz in -chunk_radius..=chunk_radius {
                            let block_coordinates = IVec3::new(chunk_radius, by, bz);
                            let block_position = chunk_position + block_coordinates;

                            
                        }
                    }

                    for bz in -chunk_radius..=chunk_radius {
                        for bx in -chunk_radius..=chunk_radius {
                            let block_coordinates = IVec3::new(bx, chunk_radius, bz);
                            let _block_position = chunk_position + block_coordinates;

                        }
                    }

                    for bx in -chunk_radius..=chunk_radius {
                        for by in -chunk_radius..=chunk_radius {
                            let block_coordinates = IVec3::new(bx, by, chunk_radius);
                            let _block_position = chunk_position + block_coordinates;

                        }
                    }
                }
            }
        }
    }
}
