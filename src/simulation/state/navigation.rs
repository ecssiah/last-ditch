pub mod graph;
pub mod path;

pub use graph::Graph;

use crate::simulation::{constants::*, state::World};
use std::collections::VecDeque;
use ultraviolet::IVec3;

pub struct Navigation {
    pub graph: Graph,
    pub path_request_deque: VecDeque<path::Request>,
    pub path_result_vec: Vec<path::Result>,
    pub path_task_vec: Vec<path::Task>,
    pub next_id: u64,
}

impl Navigation {
    pub fn new() -> Self {
        let graph = Graph::new();
        let path_request_deque = VecDeque::new();
        let path_result_vec = Vec::new();
        let path_task_vec = Vec::new();
        let next_id = 0;

        Self {
            graph,
            path_request_deque,
            path_result_vec,
            path_task_vec,
            next_id,
        }
    }
    
    pub fn get_id(navigation: &mut Navigation) -> u64 {
        let id = navigation.next_id;

        navigation.next_id += 1;

        id
    }

    pub fn init_graph(world: &World, graph: &mut Graph) {
        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;

        for z in -sector_radius_in_cells..=sector_radius_in_cells {
            for y in -sector_radius_in_cells..=sector_radius_in_cells {
                for x in -sector_radius_in_cells..=sector_radius_in_cells {
                    let cell = World::get_cell_at(IVec3::new(x, y, z), &world.sector_vec);

                    Graph::set_solid(cell.grid_position, cell.solid, graph);
                }
            }
        }
    }

    pub fn make_request(start: IVec3, end: IVec3, navigation: &mut Navigation) -> u64 {
        let id = Self::get_id(navigation);

        let path_request = path::Request {
            id,
            start,
            end,
        };

        navigation.path_request_deque.push_back(path_request);

        id
    }

    pub fn tick(_world: &World, _navigation: &mut Navigation) {}
}
