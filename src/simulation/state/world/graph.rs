pub mod edge;
pub mod entrance;
pub mod node;
pub mod region;
pub mod transition;

pub use edge::Edge;
pub use entrance::Entrance;
pub use node::Node;
pub use region::Region;
pub use transition::Transition;

use crate::simulation::state::world::{chunk::Chunk, grid::Grid};
use fixedbitset::FixedBitSet;
use glam::IVec3;
use std::collections::{HashMap, HashSet};

pub struct Level {
    pub node_vec: HashMap<IVec3, Node>,
    pub edge_vec: HashMap<(IVec3, IVec3), Edge>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            node_vec: HashMap::new(),
            edge_vec: HashMap::new(),
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Graph {
    pub depth: usize,
    pub grid: Grid,
    pub solid_set_map: HashMap<IVec3, FixedBitSet>,
    pub clearance_map: HashMap<IVec3, u32>,
    pub region_vec: Vec<Region>,
    pub entrance_vec: Vec<Entrance>,
    pub level_vec: Vec<Level>,
}

impl Graph {
    pub fn new(grid: &Grid, depth: usize) -> Self {
        Self {
            depth,
            grid: *grid,
            solid_set_map: HashMap::new(),
            clearance_map: HashMap::new(),
            region_vec: Vec::new(),
            entrance_vec: Vec::new(),
            level_vec: Vec::with_capacity(depth),
        }
    }

    pub fn setup(&mut self, chunk_vec: &[Chunk]) {
        self.solid_set_map = Self::setup_solid_set_map(&self.grid, chunk_vec);
        self.region_vec = Self::setup_regions(&self.grid, chunk_vec);

        self.setup_clearance_map();
        self.setup_nodes();
    }

    fn setup_solid_set_map(grid: &Grid, chunk_vec: &[Chunk]) -> HashMap<IVec3, FixedBitSet> {
        chunk_vec
            .iter()
            .map(|chunk| {
                let mut solid_set = FixedBitSet::with_capacity(grid.chunk_volume as usize);

                for block in &chunk.block_vec {
                    solid_set.set(usize::from(block.id), block.solid);
                }

                let chunk_coordinates = grid.position_to_chunk_coordinates(chunk.position).unwrap();

                (chunk_coordinates, solid_set)
            })
            .collect()
    }

    fn setup_clearance_map(&mut self) {
        let mut clearance_map = HashMap::new();

        let world_boundary = self.grid.world_boundary as i32;

        for x in -world_boundary..=world_boundary {
            for y in -world_boundary..=world_boundary {
                for z in -world_boundary..=world_boundary {
                    let position = IVec3::new(x, y, z);

                    clearance_map.insert(position, self.calculate_clearance(position));
                }
            }
        }

        self.clearance_map = clearance_map;
    }

    fn setup_regions(grid: &Grid, chunk_vec: &[Chunk]) -> Vec<Region> {
        let chunk_radius = IVec3::splat(grid.chunk_radius as i32);

        chunk_vec
            .iter()
            .map(|chunk| {
                let min = chunk.position - chunk_radius;
                let max = chunk.position + chunk_radius;

                Region { min, max }
            })
            .collect()
    }

    fn setup_nodes(&mut self) {
        let chunk_radius = self.grid.chunk_radius as i32;
        let world_radius = self.grid.world_radius as i32;

        struct CardinalNeighborPositions {
            up: IVec3,
            center: IVec3,
            down: IVec3,
        }

        for cx in -world_radius..=world_radius - 1 {
            for cy in -world_radius..=world_radius - 1 {
                for cz in -world_radius..=world_radius - 1 {
                    let chunk_coordinates = IVec3::new(cx, cy, cz);
                    let chunk_position = self
                        .grid
                        .chunk_coordinates_to_position(chunk_coordinates)
                        .unwrap();

                    let mut x_visited_set = HashSet::new();
                    let mut x_entrance_active = false;

                    for by in -chunk_radius..=chunk_radius {
                        for bz in -chunk_radius..=chunk_radius {
                            let block_coordinates = IVec3::new(chunk_radius, by, bz);
                            let block_position = chunk_position + block_coordinates;

                            if x_visited_set.contains(&block_position) {
                                x_entrance_active = false;
                                continue;
                            }

                            x_visited_set.insert(block_position);

                            let &block_clearance = self.clearance_map.get(&block_position).unwrap();

                            (1..=block_clearance).for_each(|level| {
                                x_visited_set.insert(block_position + IVec3::Y * level as i32);
                            });

                            if block_clearance >= 3 {
                                let x_neighbor_chunk_coordinates = chunk_coordinates + IVec3::X;

                                let neighbor_positions = CardinalNeighborPositions {
                                    up: block_position + IVec3::new(1, 1, 0),
                                    center: block_position + IVec3::new(1, 0, 0),
                                    down: block_position + IVec3::new(1, -1, 0),
                                };

                                let neighbor_clearance_down =
                                    self.get_clearance(neighbor_positions.down);

                                if neighbor_clearance_down >= 3 {
                                    if !x_entrance_active {
                                        let entrance = Entrance {
                                            region1_position: chunk_coordinates,
                                            region2_position: x_neighbor_chunk_coordinates,
                                            transitions: Vec::new(),
                                        };

                                        self.entrance_vec.push(entrance);

                                        x_entrance_active = true;
                                    };

                                    let last_entrance_index = self.entrance_vec.len() - 1;

                                    if let Some(entrance) =
                                        self.entrance_vec.get_mut(last_entrance_index)
                                    {
                                        let transition = Transition {
                                            region1_position: block_position,
                                            region2_position: neighbor_positions.down,
                                        };

                                        entrance.transitions.push(transition);
                                    }
                                } else {
                                    let neighbor_clearance_center =
                                        self.get_clearance(neighbor_positions.center);

                                    if neighbor_clearance_center >= 3 {
                                        if !x_entrance_active {
                                            let entrance = Entrance {
                                                region1_position: chunk_coordinates,
                                                region2_position: x_neighbor_chunk_coordinates,
                                                transitions: Vec::new(),
                                            };

                                            self.entrance_vec.push(entrance);

                                            x_entrance_active = true;
                                        };

                                        let last_entrance_index = self.entrance_vec.len() - 1;

                                        if let Some(entrance) =
                                            self.entrance_vec.get_mut(last_entrance_index)
                                        {
                                            let transition = Transition {
                                                region1_position: block_position,
                                                region2_position: neighbor_positions.center,
                                            };

                                            entrance.transitions.push(transition);
                                        }
                                    } else {
                                        let neighbor_clearance_up =
                                            self.get_clearance(neighbor_positions.up);

                                        if neighbor_clearance_up >= 3 {
                                            if !x_entrance_active {
                                                let entrance = Entrance {
                                                    region1_position: chunk_coordinates,
                                                    region2_position: x_neighbor_chunk_coordinates,
                                                    transitions: Vec::new(),
                                                };

                                                self.entrance_vec.push(entrance);

                                                x_entrance_active = true;
                                            };

                                            let last_entrance_index = self.entrance_vec.len() - 1;

                                            if let Some(entrance) =
                                                self.entrance_vec.get_mut(last_entrance_index)
                                            {
                                                let transition = Transition {
                                                    region1_position: block_position,
                                                    region2_position: neighbor_positions.up,
                                                };

                                                entrance.transitions.push(transition);
                                            }
                                        } else {
                                            x_entrance_active = false;
                                        }
                                    }
                                }
                            } else {
                                x_entrance_active = false;
                            }
                        }
                    }

                    let mut y_candidate_map = HashMap::new();

                    for bz in -chunk_radius..=chunk_radius {
                        for bx in -chunk_radius..=chunk_radius {
                            let block_position = chunk_position + IVec3::new(bx, chunk_radius, bz);

                            let block_clearance = self.get_clearance(block_position);

                            if block_clearance >= 3 {
                                let mut neighbor_position_vec = Vec::new();

                                let neighbor_offset_vec = [
                                    IVec3::new(1, 1, 0),
                                    IVec3::new(-1, 1, 0),
                                    IVec3::new(0, 1, 1),
                                    IVec3::new(0, 1, -1),
                                ];

                                for offset in neighbor_offset_vec {
                                    let neighbor_position = block_position + offset;
                                    let neighbor_clearance = self.get_clearance(neighbor_position);

                                    if neighbor_clearance >= 3 {
                                        neighbor_position_vec.push(neighbor_position);
                                    }
                                }

                                if !neighbor_position_vec.is_empty() {
                                    y_candidate_map.insert(block_position, neighbor_position_vec);
                                }
                            }
                        }
                    }

                    let mut y_visited_set = HashSet::new();

                    for &start in y_candidate_map.keys() {
                        if y_visited_set.contains(&start) {
                            continue;
                        }

                        let mut group = vec![start];
                        let mut queue = vec![start];

                        y_visited_set.insert(start);

                        while let Some(position) = queue.pop() {
                            for offset in [IVec3::X, -IVec3::X, IVec3::Z, -IVec3::Z] {
                                let neighbor_position = position + offset;

                                if y_candidate_map.contains_key(&neighbor_position)
                                    && !y_visited_set.contains(&neighbor_position)
                                {
                                    y_visited_set.insert(neighbor_position);

                                    queue.push(neighbor_position);
                                    group.push(neighbor_position);
                                }
                            }
                        }

                        let mut entrance = Entrance {
                            region1_position: chunk_coordinates,
                            region2_position: chunk_coordinates + IVec3::Y,
                            transitions: Vec::new(),
                        };

                        for position in group {
                            let neighbor_position_vec = y_candidate_map.get(&position).unwrap();

                            for neighbor_position in neighbor_position_vec {
                                let transition = Transition {
                                    region1_position: position,
                                    region2_position: *neighbor_position,
                                };

                                entrance.transitions.push(transition);
                            }
                        }

                        self.entrance_vec.push(entrance);
                    }

                    let mut z_visited_set = HashSet::new();
                    let mut z_entrance_active = false;

                    for by in -chunk_radius..=chunk_radius {
                        for bx in -chunk_radius..=chunk_radius {
                            let block_coordinates = IVec3::new(bx, by, chunk_radius);
                            let block_position = chunk_position + block_coordinates;

                            if z_visited_set.contains(&block_position) {
                                z_entrance_active = false;
                                continue;
                            }

                            z_visited_set.insert(block_position);

                            let &block_clearance = self.clearance_map.get(&block_position).unwrap();

                            (1..=block_clearance).for_each(|level| {
                                z_visited_set.insert(block_position + IVec3::Y * level as i32);
                            });

                            if block_clearance >= 3 {
                                let z_neighbor_chunk_coordinates = chunk_coordinates + IVec3::Z;

                                let neighbor_positions = CardinalNeighborPositions {
                                    up: block_position + IVec3::new(0, 1, 1),
                                    center: block_position + IVec3::new(0, 0, 1),
                                    down: block_position + IVec3::new(0, -1, 1),
                                };

                                let neighbor_clearance_down =
                                    self.get_clearance(neighbor_positions.down);

                                if neighbor_clearance_down >= 3 {
                                    if !z_entrance_active {
                                        let entrance = Entrance {
                                            region1_position: chunk_coordinates,
                                            region2_position: z_neighbor_chunk_coordinates,
                                            transitions: Vec::new(),
                                        };

                                        self.entrance_vec.push(entrance);

                                        z_entrance_active = true;
                                    };

                                    let last_entrance_index = self.entrance_vec.len() - 1;

                                    if let Some(entrance) =
                                        self.entrance_vec.get_mut(last_entrance_index)
                                    {
                                        let transition = Transition {
                                            region1_position: block_position,
                                            region2_position: neighbor_positions.down,
                                        };

                                        entrance.transitions.push(transition);
                                    }
                                } else {
                                    let neighbor_clearance_center =
                                        self.get_clearance(neighbor_positions.center);

                                    if neighbor_clearance_center >= 3 {
                                        if !z_entrance_active {
                                            let entrance = Entrance {
                                                region1_position: chunk_coordinates,
                                                region2_position: z_neighbor_chunk_coordinates,
                                                transitions: Vec::new(),
                                            };

                                            self.entrance_vec.push(entrance);

                                            z_entrance_active = true;
                                        };

                                        let last_entrance_index = self.entrance_vec.len() - 1;

                                        if let Some(entrance) =
                                            self.entrance_vec.get_mut(last_entrance_index)
                                        {
                                            let transition = Transition {
                                                region1_position: block_position,
                                                region2_position: neighbor_positions.center,
                                            };

                                            entrance.transitions.push(transition);
                                        }
                                    } else {
                                        let neighbor_clearance_up =
                                            self.get_clearance(neighbor_positions.up);

                                        if neighbor_clearance_up >= 3 {
                                            if !z_entrance_active {
                                                let entrance = Entrance {
                                                    region1_position: chunk_coordinates,
                                                    region2_position: z_neighbor_chunk_coordinates,
                                                    transitions: Vec::new(),
                                                };

                                                self.entrance_vec.push(entrance);

                                                z_entrance_active = true;
                                            };

                                            let last_entrance_index = self.entrance_vec.len() - 1;

                                            if let Some(entrance) =
                                                self.entrance_vec.get_mut(last_entrance_index)
                                            {
                                                let transition = Transition {
                                                    region1_position: block_position,
                                                    region2_position: neighbor_positions.up,
                                                };

                                                entrance.transitions.push(transition);
                                            }
                                        } else {
                                            z_entrance_active = false;
                                        }
                                    }
                                }
                            } else {
                                z_entrance_active = false;
                            }
                        }
                    }
                }
            }
        }

        let test_entrance_vec: Vec<&Entrance> = self
            .entrance_vec
            .iter()
            .filter(|entrance| {
                entrance.region1_position == IVec3::new(0, 0, -2)
                    || entrance.region2_position == IVec3::new(0, 0, -2)
            })
            .collect();

        for entrance in test_entrance_vec {
            println!("{:?}", entrance.region1_position);
            println!("{:?}", entrance.region2_position);

            for transition in &entrance.transitions {
                println!(" {:?}", transition);
            }
        }
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

    fn calculate_clearance(&self, position: IVec3) -> u32 {
        let chunk_size = self.grid.chunk_size as i32;
        let ground_is_solid = self.is_solid(position + IVec3::NEG_Y);

        let mut clearance = 0;

        if ground_is_solid {
            for level in 0..chunk_size {
                let level_position = position + IVec3::new(0, level, 0);

                if self.is_solid(level_position) {
                    break;
                } else {
                    clearance += 1;
                }
            }
        }

        clearance
    }

    fn get_clearance(&self, position: IVec3) -> u32 {
        if let Some(&clearance) = self.clearance_map.get(&position) {
            clearance
        } else {
            0
        }
    }
}
