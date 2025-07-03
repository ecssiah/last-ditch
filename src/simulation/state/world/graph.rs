pub mod edge;
pub mod entrance;
pub mod heap_entry;
pub mod level;
pub mod node;
pub mod region;
pub mod transition;

pub use edge::Edge;
pub use entrance::Entrance;
pub use level::Level;
pub use node::Node;
pub use region::Region;
pub use transition::Transition;

use crate::simulation::{
    consts::*,
    state::world::{
        chunk::Chunk,
        graph::{self, heap_entry::HeapEntry},
        grid::Grid,
    },
};
use fixedbitset::FixedBitSet;
use glam::IVec3;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Graph {
    pub depth: u32,
    pub grid: Grid,
    pub solid_set_map: HashMap<IVec3, FixedBitSet>,
    pub clearance_map: HashMap<IVec3, u32>,
    pub region_vec: Vec<Region>,
    pub entrance_vec: Vec<Entrance>,
    pub level_vec: Vec<Level>,
}

impl Graph {
    pub fn new(grid: &Grid, depth: u32) -> Self {
        Self {
            depth,
            grid: *grid,
            solid_set_map: HashMap::new(),
            clearance_map: HashMap::new(),
            region_vec: Vec::new(),
            entrance_vec: Vec::new(),
            level_vec: std::iter::repeat_with(Level::new)
                .take(depth as usize)
                .collect(),
        }
    }

    pub fn setup(&mut self, chunk_vec: &[Chunk]) {
        self.solid_set_map = Self::setup_solid_set_map(&self.grid, chunk_vec);
        self.region_vec = Self::setup_regions(&self.grid, chunk_vec);

        self.setup_clearance_map();
        self.setup_entrances();

        self.build();

        let path = self.get_path(IVec3::new(0, -3, 0), IVec3::new(-9, -3, 0));

        println!("Path: ");
        for position in path {
            println!("{:?}", position);
        }
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
                let coordinates = grid.position_to_chunk_coordinates(chunk.position).unwrap();

                let min = chunk.position - chunk_radius;
                let max = chunk.position + chunk_radius;

                Region {
                    coordinates,
                    min,
                    max,
                }
            })
            .collect()
    }

    fn setup_entrances(&mut self) {
        let world_radius = self.grid.world_radius as i32;

        for cx in -world_radius..=world_radius - 1 {
            for cy in -world_radius..=world_radius - 1 {
                for cz in -world_radius..=world_radius - 1 {
                    let chunk_coordinates = IVec3::new(cx, cy, cz);
                    let chunk_position = self
                        .grid
                        .chunk_coordinates_to_position(chunk_coordinates)
                        .unwrap();

                    self.setup_x_entrances(chunk_coordinates, chunk_position);
                    self.setup_y_entrances(chunk_coordinates, chunk_position);
                    self.setup_z_entrances(chunk_coordinates, chunk_position);
                }
            }
        }
    }

    fn setup_x_entrances(&mut self, chunk_coordinates: IVec3, chunk_position: IVec3) {
        let chunk_radius = self.grid.chunk_radius as i32;

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

                    let directions = [
                        ("up", block_position + IVec3::new(1, 1, 0)),
                        ("center", block_position + IVec3::new(1, 0, 0)),
                        ("down", block_position + IVec3::new(1, -1, 0)),
                    ];

                    let mut matched = false;

                    for &(_, neighbor_position) in &directions {
                        if self.get_clearance(neighbor_position) >= 3 {
                            if !x_entrance_active {
                                let entrance = Entrance {
                                    region1_coordinates: chunk_coordinates,
                                    region2_coordinates: x_neighbor_chunk_coordinates,
                                    transition_vec: Vec::new(),
                                };

                                self.entrance_vec.push(entrance);

                                x_entrance_active = true;
                            }

                            let last_entrance_index = self.entrance_vec.len() - 1;

                            if let Some(entrance) = self.entrance_vec.get_mut(last_entrance_index) {
                                let transition = Transition {
                                    region1_position: block_position,
                                    region2_position: neighbor_position,
                                };

                                entrance.transition_vec.push(transition);
                            }

                            matched = true;
                            break;
                        }
                    }

                    if !matched {
                        x_entrance_active = false;
                    }
                } else {
                    x_entrance_active = false;
                }
            }
        }
    }

    fn setup_y_entrances(&mut self, chunk_coordinates: IVec3, chunk_position: IVec3) {
        let chunk_radius = self.grid.chunk_radius as i32;

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
                region1_coordinates: chunk_coordinates,
                region2_coordinates: chunk_coordinates + IVec3::Y,
                transition_vec: Vec::new(),
            };

            for position in group {
                let neighbor_position_vec = y_candidate_map.get(&position).unwrap();

                for neighbor_position in neighbor_position_vec {
                    let transition = Transition {
                        region1_position: position,
                        region2_position: *neighbor_position,
                    };

                    entrance.transition_vec.push(transition);
                }
            }

            self.entrance_vec.push(entrance);
        }
    }

    fn setup_z_entrances(&mut self, chunk_coordinates: IVec3, chunk_position: IVec3) {
        let chunk_radius = self.grid.chunk_radius as i32;

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

                    let directions = [
                        ("up", block_position + IVec3::new(0, 1, 1)),
                        ("center", block_position + IVec3::new(0, 0, 1)),
                        ("down", block_position + IVec3::new(0, -1, 1)),
                    ];

                    let mut matched = false;

                    for &(_, neighbor_position) in &directions {
                        if self.get_clearance(neighbor_position) >= 3 {
                            if !z_entrance_active {
                                let entrance = Entrance {
                                    region1_coordinates: chunk_coordinates,
                                    region2_coordinates: z_neighbor_chunk_coordinates,
                                    transition_vec: Vec::new(),
                                };

                                self.entrance_vec.push(entrance);

                                z_entrance_active = true;
                            }

                            let last_entrance_index = self.entrance_vec.len() - 1;

                            if let Some(entrance) = self.entrance_vec.get_mut(last_entrance_index) {
                                let transition = Transition {
                                    region1_position: block_position,
                                    region2_position: neighbor_position,
                                };

                                entrance.transition_vec.push(transition);
                            }

                            matched = true;
                            break;
                        }
                    }

                    if !matched {
                        z_entrance_active = false;
                    }
                } else {
                    z_entrance_active = false;
                }
            }
        }
    }

    fn build(&mut self) {
        let mut level = Level::new();

        self.setup_regional_edges(&mut level);
        self.setup_local_edges(&mut level);

        self.level_vec.push(level);
    }

    fn setup_regional_edges(&mut self, level: &mut Level) {
        for entrance in &self.entrance_vec {
            let transition_vec = entrance.representative_transitions();

            for transition in &transition_vec {
                let node1 = level
                    .node_map
                    .entry(transition.region1_position)
                    .or_insert(Node {
                        level: 1,
                        region_coordinates: entrance.region1_coordinates,
                        position: transition.region1_position,
                    })
                    .clone();

                let node2 = level
                    .node_map
                    .entry(transition.region2_position)
                    .or_insert(Node {
                        level: 1,
                        region_coordinates: entrance.region2_coordinates,
                        position: transition.region2_position,
                    })
                    .clone();

                let clearance = self
                    .get_clearance(transition.region1_position)
                    .min(self.get_clearance(transition.region2_position));

                let edge = Edge {
                    node1,
                    node2,
                    level: 1,
                    weight: 10,
                    clearance,
                    kind: graph::edge::Kind::Regional,
                };

                level
                    .edge_map
                    .insert((node1.position, node2.position), edge);
            }
        }
    }

    fn setup_local_edges(&mut self, level: &mut Level) {
        for region in &self.region_vec {
            let node_vec: Vec<Node> = level
                .node_map
                .iter()
                .filter(|(_, node)| node.region_coordinates == region.coordinates)
                .map(|(_, node)| node.clone())
                .collect();

            for (index, node1) in node_vec.iter().enumerate() {
                for node2 in node_vec.iter().skip(index + 1) {
                    let distance = self.get_path_cost(node1.position, node2.position);

                    let clearance = self
                        .get_clearance(node1.position)
                        .min(self.get_clearance(node2.position));

                    if distance < u32::MAX {
                        let edge = Edge {
                            node1: *node1,
                            node2: *node2,
                            level: 1,
                            weight: distance,
                            clearance,
                            kind: graph::edge::Kind::Local,
                        };

                        level
                            .edge_map
                            .insert((node1.position, node2.position), edge);
                    }
                }
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
        let world_boundary = self.grid.world_boundary as i32;

        let is_bottom_layer = position.y == -world_boundary;
        let ground_is_solid = self.is_solid(position + IVec3::NEG_Y);

        let mut clearance = 0;

        if !is_bottom_layer && ground_is_solid {
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

    fn get_path(&self, start: IVec3, end: IVec3) -> Vec<Node> {
        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(HeapEntry::new(0, start));
        came_from.insert(start, None);
        cost_so_far.insert(start, 0);

        while let Some(HeapEntry { cost, position }) = heap.pop() {
            if position == end {
                let mut path = Vec::new();
                let mut current = Some(position);

                while let Some(pos) = current {
                    if let Some(node) = self.level_vec[1].node_map.get(&pos) {
                        path.push(*node);
                    }

                    current = came_from.get(&pos).cloned().flatten();
                }

                path.reverse();

                return path;
            }

            for neighbor_node in self.level_vec[1].neighbors(position) {
                let step_cost = self.level_vec[1]
                    .edge_map
                    .get(&(position, neighbor_node.position))
                    .map_or(MOVEMENT_COST_STRAIGHT, |edge| edge.weight);

                let next_cost = cost + step_cost;

                if next_cost
                    < *cost_so_far
                        .get(&neighbor_node.position)
                        .unwrap_or(&u32::MAX)
                {
                    cost_so_far.insert(neighbor_node.position, next_cost);

                    let priority = next_cost + Graph::manhattan_distance(position, end);

                    heap.push(HeapEntry::new(priority, neighbor_node.position));
                    came_from.insert(neighbor_node.position, Some(position));
                }
            }
        }

        Vec::new()
    }

    fn get_path_cost(&self, start: IVec3, goal: IVec3) -> u32 {
        if self.get_clearance(start) < 3 || self.get_clearance(goal) < 3 {
            return u32::MAX;
        }

        let mut heap = BinaryHeap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(HeapEntry::new(0, start));
        cost_so_far.insert(start, 0);

        while let Some(HeapEntry { cost, position }) = heap.pop() {
            if position == goal {
                return cost;
            }

            let direction_array = [IVec3::X, IVec3::NEG_X, IVec3::Z, IVec3::NEG_Z];

            for direction in direction_array {
                for y_offset in -1..=1 {
                    let offset = direction + IVec3::Y * y_offset;
                    let neighbor_position = position + offset;

                    if self.get_clearance(neighbor_position) < 3 {
                        continue;
                    }

                    let step_cost = if y_offset == 0 {
                        MOVEMENT_COST_STRAIGHT
                    } else {
                        MOVEMENT_COST_DIAGONAL
                    };

                    let next_cost = cost + step_cost;

                    if next_cost < *cost_so_far.get(&neighbor_position).unwrap_or(&u32::MAX) {
                        heap.push(HeapEntry::new(next_cost, neighbor_position));
                        cost_so_far.insert(neighbor_position, next_cost);
                    }
                }
            }
        }

        u32::MAX
    }

    fn manhattan_distance(position1: IVec3, position2: IVec3) -> u32 {
        (position1.x - position2.x).abs() as u32
            + (position1.y - position2.y).abs() as u32
            + (position1.z - position2.z).abs() as u32
    }
}
