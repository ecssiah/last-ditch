pub mod edge;
pub mod entrance;
pub mod level;
pub mod node;
pub mod node_entry;
pub mod path;
pub mod transition;

pub use edge::Edge;
pub use entrance::Entrance;
pub use level::Level;
pub use node::Node;
pub use node_entry::NodeEntry;
pub use path::Path;
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
    pub level_0: Level,
    pub level_vec: Vec<Level>,
}

impl Graph {
    pub fn new(grid: &Grid, max_depth: usize) -> Self {
        Self {
            max_depth,
            level_0: Level::new(0, grid.chunk_size as usize, grid.world_limit as usize),
            level_vec: Vec::new(),
        }
    }

    pub fn find_region_path(
        start_position: IVec3,
        end_position: IVec3,
        level_0: &Level,
        search_level: &mut Level,
    ) -> Vec<Edge> {
        search_level.reset();

        let start_node = Self::create_node(start_position, search_level);
        let end_node = Self::create_node(end_position, search_level);

        Self::connect_node(start_node, level_0, search_level);
        Self::connect_node(end_node, level_0, search_level);

        Self::get_edge_path(start_node, end_node, &search_level)
    }

    pub fn find_local_path(
        start_position: IVec3,
        end_position: IVec3,
        level_0: &Level,
    ) -> Vec<Edge> {
        Level::get_node(start_position, level_0)
            .zip(Level::get_node(end_position, level_0))
            .map_or(Vec::new(), |(&start_node, &end_node)| {
                Self::get_edge_path(start_node, end_node, level_0)
            })
    }

    fn create_node(position: IVec3, level: &Level) -> Node {
        let region_position =
            Self::get_region_position(position, level.region_size, level.world_limit);

        let node = Node::new(position, region_position, level.depth);

        node
    }

    fn connect_node(node: Node, level_0: &Level, level_1: &mut Level) {
        let mut edge_vec = Vec::new();

        if let Some(node_map) = level_1.region_node_map.get(&node.region_position) {
            for (_, &region_node) in node_map {
                if node == region_node {
                    continue;
                }

                let cost = Self::get_path_cost(node, region_node, level_0);
                let edge = Edge::new(node, region_node, edge::Kind::Internal, cost, level_1.depth);

                edge_vec.push(edge);
            }
        }

        if !edge_vec.is_empty() {
            Level::attach_node(node, &mut level_1.region_node_map);
            Level::register_search_node(node, &mut level_1.search_node_keys_vec);

            for edge in edge_vec {
                Level::attach_edge(edge, &mut level_1.edge_map);
                Level::register_search_edge(edge, &mut level_1.search_edge_key_vec);
            }
        }
    }

    pub fn construct(grid: &Grid, chunk_vec_slice: &[Chunk], max_depth: usize) -> Self {
        let level_0 = Graph::setup_level_0(grid, chunk_vec_slice);
        let level_1 = Graph::setup_level_1(grid, chunk_vec_slice, &level_0);

        Self {
            max_depth,
            level_0,
            level_vec: Vec::from([level_1]),
        }
    }

    fn align_edge_vec(start_node: Node, end_node: Node, edge_vec: Vec<Edge>) -> Vec<Edge> {
        if edge_vec.is_empty() {
            return edge_vec;
        }

        let start_edge = if edge_vec[0].node2.position == start_node.position {
            Edge::flip(&edge_vec[0])
        } else {
            edge_vec[0]
        };

        let mut edge_vec_normalized = Vec::new();
        edge_vec_normalized.push(start_edge);

        if edge_vec.len() == 1 {
            return edge_vec_normalized;
        }

        for index in 1..edge_vec.len() {
            let edge1 = edge_vec[index - 1];
            let edge2 = edge_vec[index];

            if edge2.node2.position == edge1.node1.position {
                edge_vec_normalized.push(Edge::flip(&edge2));
            } else if edge2.node2.position == edge1.node2.position {
                edge_vec_normalized.push(Edge::flip(&edge2));
            } else if edge1.node1.position == edge2.node1.position {
                edge_vec_normalized.push(edge2);
            } else if edge1.node2.position == edge2.node1.position {
                edge_vec_normalized.push(edge2);
            } else {
                panic!("Disjoint path: \n{:?}\n\n |\n\n{:?}", edge1, edge2);
            }
        }

        let end_edge = edge_vec_normalized[edge_vec.len() - 1];

        if end_edge.node2.position != end_node.position {
            panic!("End Edge does not contain End Node");
        }

        edge_vec_normalized
    }

    fn setup_level_0(grid: &Grid, chunk_vec_slice: &[Chunk]) -> Level {
        let mut level_0 = Level::new(0, 1, grid.world_limit as usize);

        let chunk_radius = grid.chunk_radius as i32;
        let chunk_size = grid.chunk_size as i32;
        let world_radius = grid.world_radius as i32;
        let world_limit = grid.world_limit as i32;

        for x in -world_limit..world_limit {
            for y in -world_limit..world_limit {
                for z in -world_limit..world_limit {
                    let position = IVec3::new(x, y, z);
                    let clearance = World::get_clearance(position, grid, chunk_vec_slice);

                    if clearance >= MINIMUM_CLEARANCE {
                        let node1 = Self::create_node(position, &level_0);
                        Level::attach_node(node1, &mut level_0.region_node_map);
                    }
                }
            }
        }

        for cx in -world_radius..=world_radius {
            for cy in -world_radius..=world_radius {
                for cz in -world_radius..=world_radius {
                    let chunk_position = IVec3::new(cx, cy, cz) * chunk_size;

                    for bx in -chunk_radius..=chunk_radius {
                        for by in -chunk_radius..=chunk_radius {
                            for bz in -chunk_radius..=chunk_radius {
                                let node1_position = chunk_position + IVec3::new(bx, by, bz);

                                if let Some(node1) =
                                    Level::get_node(node1_position, &level_0).cloned()
                                {
                                    for direction_offset in grid::Direction::cardinal_offset_array()
                                    {
                                        for vertical_offset in [IVec3::NEG_Y, IVec3::ZERO, IVec3::Y]
                                        {
                                            let node2_position =
                                                node1_position + direction_offset + vertical_offset;

                                            if let Some(node2) =
                                                Level::get_node(node2_position, &level_0).cloned()
                                            {
                                                let chunk_id1 = Grid::position_to_chunk_id(
                                                    grid,
                                                    node1.position,
                                                );
                                                let chunk_id2 = Grid::position_to_chunk_id(
                                                    grid,
                                                    node2.position,
                                                );

                                                if chunk_id1 != chunk_id2 {
                                                    continue;
                                                }

                                                let cost = if node1.position.y == node2.position.y {
                                                    MOVEMENT_COST_STRAIGHT
                                                } else {
                                                    MOVEMENT_COST_DIAGONAL
                                                };

                                                let edge = Edge::new(
                                                    node1,
                                                    node2,
                                                    edge::Kind::External,
                                                    cost,
                                                    0,
                                                );

                                                Level::attach_edge(edge, &mut level_0.edge_map);

                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        level_0
    }

    fn setup_level_1(grid: &Grid, chunk_vec_slice: &[Chunk], level_0: &Level) -> Level {
        let mut level = Level::new(1, grid.chunk_size as usize, grid.world_limit as usize);

        let entrance_vec = Self::setup_entrance_vec(grid, chunk_vec_slice);

        Self::setup_external_edges(&mut level, &entrance_vec);
        Self::setup_internal_edges(&mut level, level_0);

        level
    }

    fn setup_entrance_vec(grid: &Grid, chunk_vec_slice: &[Chunk]) -> Vec<Entrance> {
        let mut entrance_vec = Vec::new();

        let world_radius = grid.world_radius as i32;

        for x in -world_radius..world_radius {
            for y in -world_radius..world_radius {
                for z in -world_radius..world_radius {
                    let chunk_coordinates = IVec3::new(x, y, z);
                    let chunk_position =
                        Grid::chunk_coordinates_to_position(grid, chunk_coordinates);

                    let x_entrance_vec =
                        Self::setup_x_entrance_vec(grid, chunk_position, chunk_vec_slice);

                    let y_entrance_vec =
                        Self::setup_y_entrance_vec(grid, chunk_position, chunk_vec_slice);

                    let z_entrance_vec =
                        Self::setup_z_entrance_vec(grid, chunk_position, chunk_vec_slice);

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
        chunk_position: IVec3,
        chunk_vec_slice: &[Chunk],
    ) -> Vec<Entrance> {
        let mut entrance_vec = Vec::new();

        let chunk_radius = grid.chunk_radius as i32;
        let chunk_size = grid.chunk_size as i32;
        let world_limit = grid.world_limit as i32;

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

                let clearance = World::get_clearance(block_position, grid, chunk_vec_slice);

                (1..=clearance).for_each(|level| {
                    visited_set.insert(block_position + IVec3::Y * level as i32);
                });

                if clearance >= MINIMUM_CLEARANCE {
                    let test_positions = [
                        block_position + IVec3::new(1, -1, 0),
                        block_position + IVec3::new(1, 0, 0),
                        block_position + IVec3::new(1, 1, 0),
                    ];

                    let mut matched = false;

                    for test_position in test_positions {
                        let neighbor_clearance =
                            World::get_clearance(test_position, grid, chunk_vec_slice);

                        if neighbor_clearance >= MINIMUM_CLEARANCE {
                            if !entrance_active {
                                let region_position1 = Self::get_region_position(
                                    block_position,
                                    chunk_size as usize,
                                    world_limit as usize,
                                );

                                let region_position2 = Self::get_region_position(
                                    test_position,
                                    chunk_size as usize,
                                    world_limit as usize,
                                );

                                let entrance = Entrance::new(region_position1, region_position2);

                                entrance_vec.push(entrance);
                                entrance_active = true;
                            }

                            let last_entrance_index = entrance_vec.len() - 1;

                            if let Some(entrance) = entrance_vec.get_mut(last_entrance_index) {
                                let transition = Transition::new(block_position, test_position);

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
        chunk_position: IVec3,
        chunk_vec_slice: &[Chunk],
    ) -> Vec<Entrance> {
        let mut entrance_vec = Vec::new();

        let chunk_radius = grid.chunk_radius as i32;
        let chunk_size = grid.chunk_size as i32;
        let world_limit = grid.world_limit as i32;

        let mut candidate_map = HashMap::new();

        for bz in -chunk_radius..=chunk_radius {
            for bx in -chunk_radius..=chunk_radius {
                let block_position = chunk_position + IVec3::new(bx, chunk_radius, bz);
                let block_clearance = World::get_clearance(block_position, grid, chunk_vec_slice);

                if block_clearance >= MINIMUM_CLEARANCE {
                    let mut neighbor_position_vec = Vec::new();

                    let test_position_array = [
                        block_position + IVec3::new(1, 1, 0),
                        block_position + IVec3::new(-1, 1, 0),
                        block_position + IVec3::new(0, 1, 1),
                        block_position + IVec3::new(0, 1, -1),
                    ];

                    for test_position in test_position_array {
                        let neighbor_clearance =
                            World::get_clearance(test_position, grid, chunk_vec_slice);

                        if neighbor_clearance >= MINIMUM_CLEARANCE {
                            neighbor_position_vec.push(test_position);
                        }
                    }

                    if !neighbor_position_vec.is_empty() {
                        candidate_map.insert(block_position, neighbor_position_vec);
                    }
                }
            }
        }

        let mut visited_set = HashSet::new();

        for &start_position in candidate_map.keys() {
            if visited_set.contains(&start_position) {
                continue;
            }

            let mut group = vec![start_position];
            let mut queue = vec![start_position];

            visited_set.insert(start_position);

            while let Some(position) = queue.pop() {
                let test_position_array = [
                    position + IVec3::X,
                    position + IVec3::NEG_X,
                    position + IVec3::Z,
                    position + IVec3::NEG_Z,
                ];

                for test_position in test_position_array {
                    if candidate_map.contains_key(&test_position)
                        && !visited_set.contains(&test_position)
                    {
                        visited_set.insert(test_position);

                        queue.push(test_position);
                        group.push(test_position);
                    }
                }
            }

            let region_position1 = Self::get_region_position(
                start_position,
                chunk_size as usize,
                world_limit as usize,
            );

            let region_position2 = Self::get_region_position(
                start_position + IVec3::Y,
                chunk_size as usize,
                world_limit as usize,
            );

            let mut entrance = Entrance::new(region_position1, region_position2);

            for position in group {
                let neighbor_position_vec = candidate_map.get(&position).unwrap();

                for &neighbor_position in neighbor_position_vec {
                    let transition = Transition::new(position, neighbor_position);

                    entrance.transition_vec.push(transition);
                }
            }

            entrance_vec.push(entrance);
        }

        entrance_vec
    }

    fn setup_z_entrance_vec(
        grid: &Grid,
        chunk_position: IVec3,
        chunk_vec_slice: &[Chunk],
    ) -> Vec<Entrance> {
        let mut entrance_vec = Vec::new();

        let chunk_radius = grid.chunk_radius as i32;
        let chunk_size = grid.chunk_size as i32;
        let world_limit = grid.world_limit as i32;

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

                let clearance = World::get_clearance(block_position, grid, chunk_vec_slice);

                (1..=clearance).for_each(|level| {
                    visited_set.insert(block_position + IVec3::Y * level as i32);
                });

                if clearance >= MINIMUM_CLEARANCE {
                    let test_position_array = [
                        block_position + IVec3::new(0, 1, 1),
                        block_position + IVec3::new(0, 0, 1),
                        block_position + IVec3::new(0, -1, 1),
                    ];

                    let mut matched = false;

                    for test_position in test_position_array {
                        let neighbor_clearance =
                            World::get_clearance(test_position, grid, chunk_vec_slice);

                        if neighbor_clearance >= MINIMUM_CLEARANCE {
                            if !entrance_active {
                                let region_position1 = Self::get_region_position(
                                    block_position,
                                    chunk_size as usize,
                                    world_limit as usize,
                                );

                                let region_position2 = Self::get_region_position(
                                    test_position,
                                    chunk_size as usize,
                                    world_limit as usize,
                                );

                                let entrance = Entrance::new(region_position1, region_position2);

                                entrance_vec.push(entrance);
                                entrance_active = true;
                            }

                            let last_entrance_index = entrance_vec.len() - 1;

                            if let Some(entrance) = entrance_vec.get_mut(last_entrance_index) {
                                let transition = Transition::new(block_position, test_position);

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

    fn setup_external_edges(level_1: &mut Level, entrance_vec_slice: &[Entrance]) {
        for entrance in entrance_vec_slice {
            for transition in entrance.representative_transitions() {
                let node1 = Self::create_node(transition.position1, level_1);
                let node2 = Self::create_node(transition.position2, level_1);

                Level::attach_node(node1, &mut level_1.region_node_map);
                Level::attach_node(node2, &mut level_1.region_node_map);

                let edge = Edge::new(node1, node2, edge::Kind::External, 10, 1);

                Level::attach_edge(edge, &mut level_1.edge_map);
            }
        }
    }

    fn setup_internal_edges(level_1: &mut Level, level_0: &Level) {
        for (_, node_map) in &level_1.region_node_map {
            for (index, (_, &node1)) in node_map.iter().enumerate() {
                for (_, &node2) in node_map.iter().skip(index + 1) {
                    let node1_level_0 = Level::get_node(node1.position, &level_0)
                        .expect("Level 0 Node 1 should exist");

                    let node2_level_0 = Level::get_node(node2.position, &level_0)
                        .expect("Level 0 Node 2 should exist");

                    let cost = Self::get_path_cost(*node1_level_0, *node2_level_0, level_0);

                    if cost < u32::MAX {
                        let edge = Edge::new(node1, node2, edge::Kind::Internal, cost, 1);

                        Level::attach_edge(edge, &mut level_1.edge_map);
                    }
                }
            }
        }
    }

    fn get_region_position(node_position: IVec3, region_size: usize, world_limit: usize) -> IVec3 {
        if region_size == 1 {
            node_position
        } else {
            let region_size = region_size as i32;
            let world_limit = world_limit as i32;

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
            if node.position == end_node.position {
                return cost;
            }

            for edge in Level::edge_vec(node.position, &level.edge_map) {
                if let Some(next_cost) = cost.checked_add(edge.weight) {
                    let neighbor_node = if node.position == edge.node1.position {
                        edge.node2
                    } else {
                        edge.node1
                    };

                    let neighbor_cost = *cost_so_far.get(&neighbor_node).unwrap_or(&u32::MAX);

                    if next_cost < neighbor_cost {
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

    pub fn get_path(start_node: Node, end_node: Node, level: &Level) -> Vec<Node> {
        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(NodeEntry::new(0, start_node));
        came_from.insert(start_node, None);
        cost_so_far.insert(start_node, 0);

        while let Some(NodeEntry { cost, node }) = heap.pop() {
            if node == end_node {
                let mut node_vec = Vec::new();
                let mut current = Some(node);

                while let Some(test_node) = current {
                    node_vec.push(test_node);
                    current = came_from.get(&test_node).cloned().flatten();
                }

                node_vec.reverse();

                return node_vec;
            }

            for edge in Level::edge_vec(node.position, &level.edge_map) {
                if let Some(next_cost) = cost.checked_add(edge.weight) {
                    let neighbor_node = if node.position == edge.node1.position {
                        edge.node2
                    } else {
                        edge.node1
                    };

                    let neighbor_cost = *cost_so_far.get(&neighbor_node).unwrap_or(&u32::MAX);

                    if next_cost < neighbor_cost {
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

    pub fn get_edge_path(start_node: Node, end_node: Node, level: &Level) -> Vec<Edge> {
        let mut heap = BinaryHeap::new();
        let mut came_from: HashMap<Node, Option<Node>> = HashMap::new();
        let mut cost_so_far = HashMap::new();

        heap.push(NodeEntry::new(0, start_node));
        came_from.insert(start_node, None);
        cost_so_far.insert(start_node, 0);

        while let Some(NodeEntry { cost, node }) = heap.pop() {
            if node == end_node {
                let mut edge_vec = Vec::new();
                let mut current = Some(node);

                while let Some(test_node) = current {
                    if let Some(Some(prev_node)) = came_from.get(&test_node) {
                        if let Some(test_edge) =
                            Level::edge_vec(prev_node.position, &level.edge_map)
                                .into_iter()
                                .find(|edge| {
                                    (edge.node1 == *prev_node && edge.node2 == test_node)
                                        || (edge.node2 == *prev_node && edge.node1 == test_node)
                                })
                        {
                            edge_vec.push(test_edge);
                        }
                        current = Some(*prev_node);
                    } else {
                        current = None;
                    }
                }

                edge_vec.reverse();

                return Self::align_edge_vec(start_node, end_node, edge_vec);
            }

            for edge in Level::edge_vec(node.position, &level.edge_map) {
                if let Some(next_cost) = cost.checked_add(edge.weight) {
                    let neighbor_node = if node.position == edge.node1.position {
                        edge.node2
                    } else {
                        edge.node1
                    };

                    let neighbor_cost = *cost_so_far.get(&neighbor_node).unwrap_or(&u32::MAX);

                    if next_cost < neighbor_cost {
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
