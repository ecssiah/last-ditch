pub mod edge;
pub mod entrance;
pub mod level;
pub mod node;
pub mod node_entry;
pub mod transition;

pub use edge::Edge;
pub use entrance::Entrance;
pub use level::Level;
pub use node::Node;
pub use node_entry::NodeEntry;
pub use transition::Transition;

use crate::simulation::{
    consts::*,
    state::{
        world::{
            chunk::Chunk,
            grid::{self, Grid},
        },
        World,
    },
};
use glam::IVec3;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone)]
pub struct Graph {
    pub max_depth: usize,
    pub level_vec: Vec<Level>,
}

impl Graph {
    pub fn new(_grid: &Grid, max_depth: usize) -> Self {
        Self {
            max_depth,
            level_vec: Vec::new(),
        }
    }

    pub fn test_full_path(&mut self) {
        let path_vec = self.find_path(IVec3::new(0, -3, 0), IVec3::new(0, 6, 9));

        println!("Path: ");
        for node in &path_vec {
            println!("{:?}", node);
        }
        println!("\n");

        println!("Full Path: ");
        let level_0 = &mut self.level_vec[0];
        let mut full_path_vec = Vec::new();

        for index in 1..path_vec.len() {
            let node1 = path_vec[index - 1];
            let node2 = path_vec[index];

            let path = Self::get_path(node1, node2, level_0);

            full_path_vec.extend(path);
        }

        full_path_vec.dedup();

        for position in &full_path_vec {
            println!("{:?}", position);
        }
    }

    pub fn find_path(&mut self, start: IVec3, end: IVec3) -> Vec<Node> {
        let level = &mut self.level_vec[1];
        level.reset();

        let start_node = Self::create_node(start, level);
        let end_node = Self::create_node(end, level);

        Self::connect_node(start_node, level);
        Self::connect_node(end_node, level);

        Self::get_path(start_node, end_node, level)
    }

    fn create_node(position: IVec3, level: &mut Level) -> Node {
        let region_position = Self::get_region_position(position, level);

        let node_map = level
            .region_node_map
            .entry(region_position)
            .or_insert_with(HashMap::new);

        if let Some(existing_node) = node_map.get(&position) {
            *existing_node
        } else {
            let node = Node::new(position, region_position, level.depth);
            node_map.insert(position, node);

            node
        }
    }

    fn connect_node(node: Node, level: &mut Level) {
        let mut edge_vec = Vec::new();

        if let Some(node_map) = level.region_node_map.get(&node.region_position) {
            for (_, &region_node) in node_map {
                if node.position == region_node.position {
                    continue;
                }

                let cost = Self::get_path_cost(node, region_node, level);

                edge_vec.push(Edge::new(
                    node,
                    region_node,
                    level.depth,
                    cost,
                    edge::Kind::Internal,
                ));
            }
        }

        level.add_search_node(node);

        for edge in edge_vec {
            level.add_search_edge(edge);
        }
    }

    pub fn construct(grid: &Grid, chunk_vec_slice: &[Chunk], max_depth: usize) -> Self {
        let level_0 = Graph::setup_level_0(grid, chunk_vec_slice);
        let level_1 = Graph::setup_level_1(grid, chunk_vec_slice);

        Self {
            max_depth,
            level_vec: Vec::from([level_0, level_1]),
        }
    }

    fn setup_level_0(grid: &Grid, chunk_vec_slice: &[Chunk]) -> Level {
        let mut level = Level::new(0, 1, grid.world_limit as usize);

        let world_limit = grid.world_limit as i32;

        for x in -world_limit..world_limit {
            for y in -world_limit..world_limit {
                for z in -world_limit..world_limit {
                    let position = IVec3::new(x, y, z);

                    let node = Node::new(position, position, 0);
                    let clearance = World::get_clearance(position, grid, chunk_vec_slice);

                    if clearance >= MINIMUM_CLEARANCE {
                        for direction_offset in grid::Direction::axis_offset_array() {
                            for vertical_offset in [-IVec3::NEG_Y, IVec3::ZERO, IVec3::Y] {
                                let neighbor_position =
                                    position + direction_offset + vertical_offset;

                                let neighbor_clearance =
                                    World::get_clearance(neighbor_position, grid, chunk_vec_slice);

                                if neighbor_clearance >= MINIMUM_CLEARANCE {
                                    let neighbor_node =
                                        Node::new(neighbor_position, neighbor_position, 0);

                                    level
                                        .region_node_map
                                        .entry(position)
                                        .or_insert_with(HashMap::new)
                                        .insert(position, node);

                                    level
                                        .region_node_map
                                        .entry(position)
                                        .or_insert_with(HashMap::new)
                                        .insert(neighbor_position, neighbor_node);

                                    let cost = if vertical_offset.y == 0 {
                                        MOVEMENT_COST_STRAIGHT
                                    } else {
                                        MOVEMENT_COST_DIAGONAL
                                    };

                                    let edge = Edge::new(
                                        node,
                                        neighbor_node,
                                        0,
                                        cost,
                                        edge::Kind::External,
                                    );

                                    level.edge_map.insert((position, neighbor_position), edge);

                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        level
    }

    fn setup_level_1(grid: &Grid, chunk_vec_slice: &[Chunk]) -> Level {
        let mut level = Level::new(1, grid.chunk_size as usize, grid.world_limit as usize);

        let entrance_vec = Self::setup_entrance_vec(grid, chunk_vec_slice);

        Self::setup_external_edges(&entrance_vec, &mut level);
        Self::setup_internal_edges(&mut level);

        level
    }

    fn setup_entrance_vec(grid: &Grid, chunk_vec_slice: &[Chunk]) -> Vec<Entrance> {
        let mut entrance_vec = Vec::new();

        let world_radius = grid.world_radius as i32;

        for x in -world_radius..world_radius {
            for y in -world_radius..world_radius {
                for z in -world_radius..world_radius {
                    let chunk_coordinates = IVec3::new(x, y, z);

                    let chunk_index =
                        u32::from(grid.chunk_coordinates_to_chunk_id(chunk_coordinates));

                    let chunk_position = grid.chunk_coordinates_to_position(chunk_coordinates);

                    let x_entrance_vec = Self::setup_x_entrance_vec(
                        grid,
                        chunk_vec_slice,
                        chunk_index,
                        chunk_coordinates,
                        chunk_position,
                    );

                    let y_entrance_vec = Self::setup_y_entrance_vec(
                        grid,
                        chunk_vec_slice,
                        chunk_index,
                        chunk_coordinates,
                        chunk_position,
                    );

                    let z_entrance_vec = Self::setup_z_entrance_vec(
                        grid,
                        chunk_vec_slice,
                        chunk_index,
                        chunk_coordinates,
                        chunk_position,
                    );

                    entrance_vec.extend(x_entrance_vec);
                    entrance_vec.extend(y_entrance_vec);
                    entrance_vec.extend(z_entrance_vec);
                }
            }
        }

        entrance_vec
    }

    fn setup_x_entrance_vec(
        grid: &Grid,
        chunk_vec_slice: &[Chunk],
        chunk_index: u32,
        chunk_coordinates: IVec3,
        chunk_position: IVec3,
    ) -> Vec<Entrance> {
        let mut entrance_vec = Vec::new();

        let chunk_radius = grid.chunk_radius as i32;

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

                let block_clearance = World::get_clearance(block_position, grid, chunk_vec_slice);

                (1..=block_clearance).for_each(|level| {
                    visited_set.insert(block_position + IVec3::Y * level as i32);
                });

                if block_clearance >= MINIMUM_CLEARANCE {
                    let neighbor_chunk_coordinates = chunk_coordinates + IVec3::X;

                    let neighbor_positions = [
                        block_position + IVec3::new(1, 1, 0),
                        block_position + IVec3::new(1, 0, 0),
                        block_position + IVec3::new(1, -1, 0),
                    ];

                    let mut matched = false;

                    for neighbor_position in neighbor_positions {
                        let neighbor_clearance =
                            World::get_clearance(neighbor_position, grid, chunk_vec_slice);

                        if neighbor_clearance >= MINIMUM_CLEARANCE {
                            if !entrance_active {
                                let neighbor_chunk_index = u32::from(
                                    grid.chunk_coordinates_to_chunk_id(neighbor_chunk_coordinates),
                                );

                                let entrance = Entrance {
                                    region1_id: chunk_index,
                                    region2_id: neighbor_chunk_index,
                                    transition_vec: Vec::new(),
                                };

                                entrance_vec.push(entrance);

                                entrance_active = true;
                            }

                            let last_entrance_index = entrance_vec.len() - 1;

                            if let Some(entrance) = entrance_vec.get_mut(last_entrance_index) {
                                let transition = Transition {
                                    position1: block_position,
                                    position2: neighbor_position,
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

        entrance_vec
    }

    fn setup_y_entrance_vec(
        grid: &Grid,
        chunk_vec_slice: &[Chunk],
        chunk_index: u32,
        chunk_coordinates: IVec3,
        chunk_position: IVec3,
    ) -> Vec<Entrance> {
        let mut entrance_vec = Vec::new();

        let chunk_radius = grid.chunk_radius as i32;

        let mut candidate_map = HashMap::new();

        for bz in -chunk_radius..=chunk_radius {
            for bx in -chunk_radius..=chunk_radius {
                let block_position = chunk_position + IVec3::new(bx, chunk_radius, bz);
                let block_clearance = World::get_clearance(block_position, grid, chunk_vec_slice);

                if block_clearance >= MINIMUM_CLEARANCE {
                    let mut neighbor_position_vec = Vec::new();

                    let neighbor_position_array = [
                        block_position + IVec3::new(1, 1, 0),
                        block_position + IVec3::new(-1, 1, 0),
                        block_position + IVec3::new(0, 1, 1),
                        block_position + IVec3::new(0, 1, -1),
                    ];

                    for neighbor_position in neighbor_position_array {
                        let neighbor_clearance =
                            World::get_clearance(neighbor_position, grid, chunk_vec_slice);

                        if neighbor_clearance >= MINIMUM_CLEARANCE {
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

            let neighbor_chunk_index =
                u32::from(grid.chunk_coordinates_to_chunk_id(chunk_coordinates + IVec3::Y));

            let mut entrance = Entrance {
                region1_id: chunk_index,
                region2_id: neighbor_chunk_index,
                transition_vec: Vec::new(),
            };

            for position in group {
                let neighbor_position_vec = candidate_map.get(&position).unwrap();

                for neighbor_position in neighbor_position_vec {
                    let transition = Transition {
                        position1: position,
                        position2: *neighbor_position,
                    };

                    entrance.transition_vec.push(transition);
                }
            }

            entrance_vec.push(entrance);
        }

        entrance_vec
    }

    fn setup_z_entrance_vec(
        grid: &Grid,
        chunk_vec_slice: &[Chunk],
        chunk_index: u32,
        chunk_coordinates: IVec3,
        chunk_position: IVec3,
    ) -> Vec<Entrance> {
        let mut entrance_vec = Vec::new();

        let chunk_radius = grid.chunk_radius as i32;

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

                let block_clearance = World::get_clearance(block_position, grid, chunk_vec_slice);

                (1..=block_clearance).for_each(|level| {
                    visited_set.insert(block_position + IVec3::Y * level as i32);
                });

                if block_clearance >= MINIMUM_CLEARANCE {
                    let neighbor_chunk_coordinates = chunk_coordinates + IVec3::Z;

                    let neighbor_position_array = [
                        block_position + IVec3::new(0, 1, 1),
                        block_position + IVec3::new(0, 0, 1),
                        block_position + IVec3::new(0, -1, 1),
                    ];

                    let mut matched = false;

                    for neighbor_position in neighbor_position_array {
                        let neighbor_clearance =
                            World::get_clearance(neighbor_position, grid, chunk_vec_slice);

                        if neighbor_clearance >= MINIMUM_CLEARANCE {
                            if !entrance_active {
                                let neighbor_chunk_index = u32::from(
                                    grid.chunk_coordinates_to_chunk_id(neighbor_chunk_coordinates),
                                );

                                let entrance = Entrance {
                                    region1_id: chunk_index,
                                    region2_id: neighbor_chunk_index,
                                    transition_vec: Vec::new(),
                                };

                                entrance_vec.push(entrance);

                                entrance_active = true;
                            }

                            let last_entrance_index = entrance_vec.len() - 1;

                            if let Some(entrance) = entrance_vec.get_mut(last_entrance_index) {
                                let transition = Transition {
                                    position1: block_position,
                                    position2: neighbor_position,
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

        entrance_vec
    }

    fn setup_external_edges(entrance_vec_slice: &[Entrance], level: &mut Level) {
        for entrance in entrance_vec_slice {
            for transition in entrance.representative_transitions() {
                let node1_region_position = Self::get_region_position(transition.position1, level);

                let node1 = Node {
                    depth: 1,
                    region_position: node1_region_position,
                    position: transition.position1,
                };

                level
                    .region_node_map
                    .entry(node1_region_position)
                    .or_insert_with(HashMap::new)
                    .insert(transition.position1, node1);

                let node2_region_position = Self::get_region_position(transition.position2, level);

                let node2 = Node {
                    depth: 1,
                    region_position: node2_region_position,
                    position: transition.position2,
                };

                level
                    .region_node_map
                    .entry(node2_region_position)
                    .or_insert_with(HashMap::new)
                    .insert(transition.position2, node2);

                let edge_key = (node1.position, node2.position);
                let edge = Edge::new(node1, node2, 1, 10, edge::Kind::External);

                level.edge_map.insert(edge_key, edge);
            }
        }
    }

    fn setup_internal_edges(level: &mut Level) {
        for (_, node_map) in &level.region_node_map {
            for (index, (_, &node1)) in node_map.iter().enumerate() {
                for (_, &node2) in node_map.iter().skip(index + 1) {
                    let cost = Self::get_path_cost(node1, node2, level);

                    if cost < u32::MAX {
                        let edge_key = (node1.position, node2.position);

                        let edge = Edge::new(node1, node2, 1, cost, edge::Kind::Internal);

                        level.edge_map.insert(edge_key, edge);
                    }
                }
            }
        }
    }

    fn get_region_position(node_position: IVec3, level: &Level) -> IVec3 {
        if level.region_size == 1 {
            node_position
        } else {
            let region_size = level.region_size as i32;
            let world_limit = level.world_limit as i32;

            let node_position_indexable = node_position + world_limit;
            let region_coordinates = (node_position_indexable / region_size) * region_size;

            region_coordinates - world_limit
        }
    }

    fn get_path_cost(start_node: Node, end_node: Node, level: &Level) -> u32 {
        let mut heap = BinaryHeap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(NodeEntry::new(0, start_node));
        cost_so_far.insert(start_node, 0);

        while let Some(NodeEntry { cost, node }) = heap.pop() {
            if node == end_node {
                return cost;
            }

            for edge in Level::edge_vec(node.position, &level.edge_map) {
                if let Some(next_cost) = cost.checked_add(edge.weight) {
                    let neighbor_node = if node.position == edge.node1.position {
                        edge.node2
                    } else {
                        edge.node1
                    };

                    if next_cost < *cost_so_far.get(&neighbor_node).unwrap_or(&u32::MAX) {
                        cost_so_far.insert(neighbor_node, next_cost);

                        let priority = next_cost
                            + Self::manhattan_distance(neighbor_node.position, end_node.position);

                        heap.push(NodeEntry::new(priority, neighbor_node));
                    }
                }
            }
        }

        u32::MAX
    }

    fn get_path(start_node: Node, end_node: Node, level: &mut Level) -> Vec<Node> {
        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(NodeEntry::new(0, start_node));
        came_from.insert(start_node, None);
        cost_so_far.insert(start_node, 0);

        while let Some(NodeEntry { cost, node }) = heap.pop() {
            if node == end_node {
                let mut path = Vec::new();
                let mut current = Some(node);

                while let Some(test_node) = current {
                    path.push(test_node);
                    current = came_from.get(&test_node).cloned().flatten();
                }

                path.reverse();

                return path;
            }

            for edge in Level::edge_vec(node.position, &level.edge_map) {
                let neighbor_node = if node.position == edge.node1.position {
                    edge.node2
                } else {
                    edge.node1
                };

                if let Some(next_cost) = cost.checked_add(edge.weight) {
                    if next_cost < *cost_so_far.get(&neighbor_node).unwrap_or(&u32::MAX) {
                        cost_so_far.insert(neighbor_node, next_cost);
                        came_from.insert(neighbor_node, Some(node));

                        let priority = next_cost
                            + Self::manhattan_distance(neighbor_node.position, end_node.position);

                        heap.push(NodeEntry::new(priority, neighbor_node));
                    }
                }
            }
        }

        Vec::new()
    }

    fn manhattan_distance(position1: IVec3, position2: IVec3) -> u32 {
        (position1.x - position2.x).abs() as u32
            + (position1.y - position2.y).abs() as u32
            + (position1.z - position2.z).abs() as u32
    }
}
