pub mod graph;
pub mod path;

pub use graph::Graph;

use crate::simulation::{constants::*, state::World};
use std::collections::{HashMap, VecDeque};
use ultraviolet::IVec3;

pub struct Navigation {
    pub active: bool,
    pub graph: Graph,
    pub path_request_deque: VecDeque<path::Request>,
    pub path_result_vec: Vec<path::Result>,
    pub path_result_map: HashMap<u64, path::Result>,
    pub path_task_vec: Vec<path::Task>,
    pub next_path_request_id: u64,
}

impl Navigation {
    pub fn new() -> Self {
        let active = false;
        let graph = Graph::new();
        let path_request_deque = VecDeque::new();
        let path_result_vec = Vec::new();
        let path_result_map = HashMap::new();
        let path_task_vec = Vec::new();
        let next_path_request_id = 0;

        Self {
            active,
            graph,
            path_request_deque,
            path_result_vec,
            path_result_map,
            path_task_vec,
            next_path_request_id,
        }
    }

    pub fn get_next_path_request_id(navigation: &mut Navigation) -> u64 {
        let path_id = navigation.next_path_request_id;

        navigation.next_path_request_id += 1;

        path_id
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
        let path_request_id = Self::get_next_path_request_id(navigation);

        let path_request = path::Request {
            path_request_id,
            start,
            end,
        };

        navigation.path_request_deque.push_back(path_request);

        path_request_id
    }

    pub fn poll_result(path_request_id: u64, navigation: &mut Navigation) -> bool {
        navigation
            .path_result_vec
            .iter()
            .any(|result| result.path_request_id == path_request_id)
    }

    pub fn take_result(path_request_id: u64, navigation: &mut Navigation) -> Option<path::Result> {
        let index = navigation
            .path_result_vec
            .iter()
            .position(|result| result.path_request_id == path_request_id);

        index.map(|index| navigation.path_result_vec.remove(index))
    }

    pub fn tick(_world: &World, navigation: &mut Navigation) {
        let _ = tracing::info_span!("navigation_tick");

        if !navigation.active {
            return;
        }
    }
}
