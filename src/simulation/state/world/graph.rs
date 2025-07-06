pub mod edge;
pub mod entrance;
pub mod heap_entry;
pub mod level;
pub mod node;
pub mod region;
pub mod transition;

pub use edge::Edge;
pub use entrance::Entrance;
pub use heap_entry::HeapEntry;
pub use level::Level;
pub use node::Node;
pub use region::Region;
pub use transition::Transition;

use crate::simulation::{
    consts::*,
    state::world::{
        block,
        chunk::Chunk,
        graph::{self},
        grid::Grid,
    },
};
use fixedbitset::FixedBitSet;
use glam::IVec3;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone)]
pub struct Graph {
    pub depth: u32,
    pub grid: Grid,
    pub solid_set_map: HashMap<IVec3, FixedBitSet>,
    pub clearance_map: HashMap<IVec3, u32>,
    pub entrance_vec: Vec<Entrance>,
    pub level_vec: Vec<Level>,
}

impl Graph {
    pub fn new(grid: &Grid, depth: u32) -> Self {
        Self {
            depth,
            grid: grid.clone(),
            solid_set_map: HashMap::new(),
            clearance_map: HashMap::new(),
            entrance_vec: Vec::new(),
            level_vec: std::iter::repeat_with(Level::new)
                .take(depth as usize)
                .collect(),
        }
    }

    pub fn construct(grid: &Grid, depth: u32, chunk_vec_slice: &[Chunk]) -> Self {
        let mut graph = Self {
            depth,
            grid: grid.clone(),
            solid_set_map: HashMap::new(),
            clearance_map: HashMap::new(),
            entrance_vec: Vec::new(),
            level_vec: std::iter::repeat_with(Level::new)
                .take(depth as usize)
                .collect(),
        };

        graph.setup(chunk_vec_slice);

        graph
    }

    pub fn setup(&mut self, chunk_vec_slice: &[Chunk]) {
        self.setup_solid_set_map(chunk_vec_slice);
        self.setup_clearance_map();
        self.setup_entrances();

        self.build();

        self.test_full_path();
    }

    pub fn find_path(&mut self, start: IVec3, end: IVec3) -> Vec<Node> {
        let level = &mut self.level_vec[1];
        level.reset();

        let start_node = self.create_node(start, 1);
        let end_node = self.create_node(end, 1);

        self.connect_node(start_node);
        self.connect_node(end_node);

        let node_vec = self.get_node_path(start_node, end_node);

        node_vec
    }

    fn test_full_path(&mut self) {
        let path_vec = self.find_path(IVec3::new(0, -3, 0), IVec3::new(0, 6, 9));

        println!("Path: ");
        for node in &path_vec {
            println!("{:?}", node);
        }

        println!("\nFull Path: ");
        let mut full_path_vec = Vec::new();
        for index in 1..path_vec.len() {
            let node1 = path_vec[index - 1];
            let node2 = path_vec[index];

            let path = self.get_path(node1.position, node2.position);

            full_path_vec.extend(path);
        }

        full_path_vec.dedup();

        for position in &full_path_vec {
            println!("{:?}", position);
        }
    }

    fn connect_node(&mut self, node: Node) {
        let level = &self.level_vec[1];
        let mut edge_vec = Vec::new();

        for transition_node in level.get_region_node_vec(node.region_id) {
            if node.position == transition_node.position {
                continue;
            }

            let weight = self.get_path_cost(node.position, transition_node.position);

            let clearance = self
                .get_clearance(node.position)
                .min(self.get_clearance(transition_node.position));

            let edge = Edge {
                node1: node,
                node2: *transition_node,
                level: 1,
                weight,
                clearance,
                kind: graph::edge::Kind::Local,
            };

            edge_vec.push(edge);
        }

        self.level_vec[1].add_search_node(node);

        for edge in edge_vec.drain(..) {
            self.level_vec[1].add_search_edge(edge);
        }
    }

    fn setup_solid_set_map(&mut self, chunk_vec_slice: &[Chunk]) {
        self.solid_set_map = chunk_vec_slice
            .iter()
            .map(|chunk| {
                let mut solid_set = FixedBitSet::with_capacity(self.grid.chunk_volume as usize);

                for block in &chunk.block_vec {
                    solid_set.set(usize::from(block.id), block.solid);
                }

                let chunk_coordinates = self.grid.position_to_chunk_coordinates(chunk.position);

                (chunk_coordinates, solid_set)
            })
            .collect()
    }

    fn setup_clearance_map(&mut self) {
        let mut clearance_map = HashMap::new();

        let world_limit = self.grid.world_limit as i32;

        for x in -world_limit..=world_limit {
            for y in -world_limit..=world_limit {
                for z in -world_limit..=world_limit {
                    let position = IVec3::new(x, y, z);

                    clearance_map.insert(position, self.calculate_clearance(position));
                }
            }
        }

        self.clearance_map = clearance_map;
    }

    fn setup_entrances(&mut self) {
        let world_radius = self.grid.world_radius as i32;

        for cx in -world_radius..=world_radius - 1 {
            for cy in -world_radius..=world_radius - 1 {
                for cz in -world_radius..=world_radius - 1 {
                    let chunk_coordinates = IVec3::new(cx, cy, cz);

                    let chunk_index =
                        u32::from(self.grid.chunk_coordinates_to_chunk_id(chunk_coordinates));
                    let chunk_position = self.grid.chunk_coordinates_to_position(chunk_coordinates);

                    self.setup_x_entrances(chunk_index, chunk_coordinates, chunk_position);
                    self.setup_y_entrances(chunk_index, chunk_coordinates, chunk_position);
                    self.setup_z_entrances(chunk_index, chunk_coordinates, chunk_position);
                }
            }
        }
    }

    fn setup_x_entrances(
        &mut self,
        chunk_index: u32,
        chunk_coordinates: IVec3,
        chunk_position: IVec3,
    ) {
        let chunk_radius = self.grid.chunk_radius as i32;

        let mut entrance_active = false;
        let mut visited_set = HashSet::new();

        for by in -chunk_radius..=chunk_radius {
            for bz in -chunk_radius..=chunk_radius {
                let block_coordinates = IVec3::new(chunk_radius, by, bz);
                let block_position = chunk_position + block_coordinates;

                if visited_set.contains(&block_position) {
                    entrance_active = false;
                    continue;
                }

                visited_set.insert(block_position);

                let &block_clearance = self.clearance_map.get(&block_position).unwrap();

                (1..=block_clearance).for_each(|level| {
                    visited_set.insert(block_position + IVec3::Y * level as i32);
                });

                if block_clearance >= 3 {
                    let neighbor_chunk_coordinates = chunk_coordinates + IVec3::X;

                    let neighbor_positions = [
                        block_position + IVec3::new(1, 1, 0),
                        block_position + IVec3::new(1, 0, 0),
                        block_position + IVec3::new(1, -1, 0),
                    ];

                    let mut matched = false;

                    for neighbor_position in neighbor_positions {
                        if self.get_clearance(neighbor_position) >= 3 {
                            if !entrance_active {
                                let neighbor_chunk_index = u32::from(
                                    self.grid
                                        .chunk_coordinates_to_chunk_id(neighbor_chunk_coordinates),
                                );

                                let entrance = Entrance {
                                    region1_id: chunk_index,
                                    region2_id: neighbor_chunk_index,
                                    transition_vec: Vec::new(),
                                };

                                self.entrance_vec.push(entrance);

                                entrance_active = true;
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
                        entrance_active = false;
                    }
                } else {
                    entrance_active = false;
                }
            }
        }
    }

    fn setup_y_entrances(
        &mut self,
        chunk_index: u32,
        chunk_coordinates: IVec3,
        chunk_position: IVec3,
    ) {
        let chunk_radius = self.grid.chunk_radius as i32;

        let mut candidate_map = HashMap::new();

        for bz in -chunk_radius..=chunk_radius {
            for bx in -chunk_radius..=chunk_radius {
                let block_position = chunk_position + IVec3::new(bx, chunk_radius, bz);

                let block_clearance = self.get_clearance(block_position);

                if block_clearance >= 3 {
                    let mut neighbor_position_vec = Vec::new();

                    let neighbor_position_array = [
                        block_position + IVec3::new(1, 1, 0),
                        block_position + IVec3::new(-1, 1, 0),
                        block_position + IVec3::new(0, 1, 1),
                        block_position + IVec3::new(0, 1, -1),
                    ];

                    for neighbor_position in neighbor_position_array {
                        let neighbor_clearance = self.get_clearance(neighbor_position);

                        if neighbor_clearance >= 3 {
                            neighbor_position_vec.push(neighbor_position);
                        }
                    }

                    if !neighbor_position_vec.is_empty() {
                        candidate_map.insert(block_position, neighbor_position_vec);
                    }
                }
            }
        }

        let mut visited_set = HashSet::new();

        for &start in candidate_map.keys() {
            if visited_set.contains(&start) {
                continue;
            }

            let mut group = vec![start];
            let mut queue = vec![start];

            visited_set.insert(start);

            while let Some(position) = queue.pop() {
                let neighbor_position_array = [
                    position + IVec3::X,
                    position + IVec3::NEG_X,
                    position + IVec3::Z,
                    position + IVec3::NEG_Z,
                ];

                for neighbor_position in neighbor_position_array {
                    if candidate_map.contains_key(&neighbor_position)
                        && !visited_set.contains(&neighbor_position)
                    {
                        visited_set.insert(neighbor_position);

                        queue.push(neighbor_position);
                        group.push(neighbor_position);
                    }
                }
            }

            let neighbor_chunk_index = u32::from(
                self.grid
                    .chunk_coordinates_to_chunk_id(chunk_coordinates + IVec3::Y),
            );

            let mut entrance = Entrance {
                region1_id: chunk_index,
                region2_id: neighbor_chunk_index,
                transition_vec: Vec::new(),
            };

            for position in group {
                let neighbor_position_vec = candidate_map.get(&position).unwrap();

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

    fn setup_z_entrances(
        &mut self,
        chunk_index: u32,
        chunk_coordinates: IVec3,
        chunk_position: IVec3,
    ) {
        let chunk_radius = self.grid.chunk_radius as i32;

        let mut entrance_active = false;
        let mut visited_set = HashSet::new();

        for by in -chunk_radius..=chunk_radius {
            for bx in -chunk_radius..=chunk_radius {
                let block_coordinates = IVec3::new(bx, by, chunk_radius);
                let block_position = chunk_position + block_coordinates;

                if visited_set.contains(&block_position) {
                    entrance_active = false;
                    continue;
                }

                visited_set.insert(block_position);

                let &block_clearance = self.clearance_map.get(&block_position).unwrap();

                (1..=block_clearance).for_each(|level| {
                    visited_set.insert(block_position + IVec3::Y * level as i32);
                });

                if block_clearance >= 3 {
                    let neighbor_chunk_coordinates = chunk_coordinates + IVec3::Z;

                    let neighbor_position_array = [
                        block_position + IVec3::new(0, 1, 1),
                        block_position + IVec3::new(0, 0, 1),
                        block_position + IVec3::new(0, -1, 1),
                    ];

                    let mut matched = false;

                    for neighbor_position in neighbor_position_array {
                        if self.get_clearance(neighbor_position) >= 3 {
                            if !entrance_active {
                                let neighbor_chunk_index = u32::from(
                                    self.grid
                                        .chunk_coordinates_to_chunk_id(neighbor_chunk_coordinates),
                                );

                                let entrance = Entrance {
                                    region1_id: chunk_index,
                                    region2_id: neighbor_chunk_index,
                                    transition_vec: Vec::new(),
                                };

                                self.entrance_vec.push(entrance);

                                entrance_active = true;
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
                        entrance_active = false;
                    }
                } else {
                    entrance_active = false;
                }
            }
        }
    }

    fn build(&mut self) {
        let mut level = Level::new();

        self.setup_regions(&mut level);
        self.setup_regional_edges(&mut level);
        self.setup_local_edges(&mut level);

        self.level_vec.push(level);
    }

    fn setup_regions(&mut self, level: &mut Level) {
        let chunk_radius = self.grid.chunk_radius as i32;
        let world_radius = self.grid.world_radius as i32;

        let mut region_map = HashMap::new();

        for rx in -world_radius..=world_radius {
            for ry in -world_radius..=world_radius {
                for rz in -world_radius..=world_radius {
                    let region_coordinates = IVec3::new(rx, ry, rz);
                    let region_position =
                        self.grid.chunk_coordinates_to_position(region_coordinates);

                    let region_id =
                        u32::from(self.grid.chunk_coordinates_to_chunk_id(region_coordinates));

                    let min = region_position - IVec3::splat(chunk_radius);
                    let max = region_position + IVec3::splat(chunk_radius);

                    region_map.insert(
                        region_id,
                        Region {
                            id: region_id,
                            coordinates: region_coordinates,
                            min,
                            max,
                        },
                    );
                }
            }
        }

        level.region_map = region_map;
    }

    fn setup_regional_edges(&mut self, level: &mut Level) {
        for entrance in &self.entrance_vec {
            let transition_vec = entrance.representative_transitions();

            for transition in &transition_vec {
                let node1_region_id =
                    u32::from(self.grid.position_to_chunk_id(transition.region1_position));

                let node1 = level
                    .node_map
                    .entry(transition.region1_position)
                    .or_insert(Node {
                        level_number: 1,
                        region_id: node1_region_id,
                        position: transition.region1_position,
                    })
                    .clone();

                let node2_region_id =
                    u32::from(self.grid.position_to_chunk_id(transition.region2_position));

                let node2 = level
                    .node_map
                    .entry(transition.region2_position)
                    .or_insert(Node {
                        level_number: 1,
                        region_id: node2_region_id,
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
        for (_, region) in &level.region_map {
            let node_vec: Vec<Node> = level
                .node_map
                .iter()
                .filter(|(position, _)| {
                    let in_x_range = position.x >= region.min.x && position.x <= region.max.x;
                    let in_y_range = position.y >= region.min.y && position.y <= region.max.y;
                    let in_z_range = position.z >= region.min.z && position.z <= region.max.z;

                    in_x_range && in_y_range && in_z_range
                })
                .map(|(_, node)| node.clone())
                .collect();

            for (index, node1) in node_vec.iter().enumerate() {
                for node2 in node_vec.iter().skip(index + 1) {
                    let distance = self.get_path_cost(node1.position, node2.position);

                    let clearance = self
                        .get_clearance(node1.position)
                        .min(self.get_clearance(node2.position));

                    if distance < u32::MAX {
                        let edge_key = (node1.position, node2.position);

                        let edge = Edge {
                            node1: *node1,
                            node2: *node2,
                            level: 1,
                            weight: distance,
                            clearance,
                            kind: graph::edge::Kind::Local,
                        };

                        level.edge_map.insert(edge_key, edge);
                    }
                }
            }
        }
    }

    fn is_solid(&self, position: IVec3) -> bool {
        let chunk_coordinates = self.grid.position_to_chunk_coordinates(position);

        if chunk_coordinates != IVec3::MAX {
            if let Some(solid_set) = self.solid_set_map.get(&chunk_coordinates) {
                let block_id = self.grid.position_to_block_id(position);

                if block_id != block::ID::MAX {
                    return solid_set.contains(usize::from(block_id));
                }
            }
        }

        false
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

    fn create_node(&self, position: IVec3, level_number: u32) -> Node {
        let level = &self.level_vec[level_number as usize];

        let region_id = level
            .region_map
            .values()
            .find(|region| {
                let in_x_range = position.x >= region.min.x && position.x <= region.max.x;
                let in_y_range = position.y >= region.min.y && position.y <= region.max.y;
                let in_z_range = position.z >= region.min.z && position.z <= region.max.z;

                in_x_range && in_y_range && in_z_range
            })
            .map(|region| region.id)
            .unwrap()
            .clone();

        level
            .node_map
            .values()
            .find(|node| {
                node.region_id == region_id
                    && node.position == position
                    && node.level_number == level_number
            })
            .map(|node| *node)
            .unwrap_or(Node {
                level_number,
                region_id,
                position,
            })
    }

    fn get_node_path(&self, start_node: Node, end_node: Node) -> Vec<Node> {
        let level = &self.level_vec[1];

        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(HeapEntry::new(0, start_node.position));
        came_from.insert(start_node.position, None);
        cost_so_far.insert(start_node.position, 0);

        while let Some(HeapEntry { cost, position }) = heap.pop() {
            if position == end_node.position {
                let mut path = Vec::new();
                let mut current = Some(position);

                while let Some(test_position) = current {
                    if let Some(node) = level.node_map.get(&test_position) {
                        path.push(*node);
                    }

                    current = came_from.get(&test_position).cloned().flatten();
                }

                path.reverse();

                return path;
            }

            for (&(position1, position2), edge) in level.edge_map.iter() {
                if position1 == position || position2 == position {
                    let neighbor_position = if position1 == position {
                        position2
                    } else {
                        position1
                    };

                    let step_cost = edge.weight;

                    if let Some(next_cost) = cost.checked_add(step_cost) {
                        if next_cost < *cost_so_far.get(&neighbor_position).unwrap_or(&u32::MAX) {
                            cost_so_far.insert(neighbor_position, next_cost);

                            let priority = next_cost
                                + Self::manhattan_distance(neighbor_position, end_node.position);

                            heap.push(HeapEntry::new(priority, neighbor_position));
                            came_from.insert(neighbor_position, Some(position));
                        }
                    }
                }
            }
        }

        Vec::new()
    }

    fn get_path(&self, start: IVec3, goal: IVec3) -> Vec<IVec3> {
        if self.get_clearance(start) < 3 || self.get_clearance(goal) < 3 {
            return Vec::new();
        }

        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(HeapEntry::new(0, start));
        came_from.insert(start, None);
        cost_so_far.insert(start, 0);

        while let Some(HeapEntry { cost, position }) = heap.pop() {
            if position == goal {
                let mut path = Vec::new();
                let mut current = Some(position);

                while let Some(test_position) = current {
                    path.push(test_position);
                    current = came_from.get(&test_position).cloned().flatten();
                }

                path.reverse();

                return path;
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

                    if let Some(next_cost) = cost.checked_add(step_cost) {
                        if next_cost < *cost_so_far.get(&neighbor_position).unwrap_or(&u32::MAX) {
                            cost_so_far.insert(neighbor_position, next_cost);

                            let priority =
                                next_cost + Self::manhattan_distance(neighbor_position, goal);

                            heap.push(HeapEntry::new(priority, neighbor_position));
                            came_from.insert(neighbor_position, Some(position));
                        }
                    }
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

                    if let Some(next_cost) = cost.checked_add(step_cost) {
                        if next_cost < *cost_so_far.get(&neighbor_position).unwrap_or(&u32::MAX) {
                            heap.push(HeapEntry::new(next_cost, neighbor_position));
                            cost_so_far.insert(neighbor_position, next_cost);
                        }
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
