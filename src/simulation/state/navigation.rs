use crate::simulation::state::{navigation, world::grid::Grid, World};
use std::collections::VecDeque;
use ultraviolet::IVec3;

pub mod graph;
pub mod path;

pub use graph::Graph;

pub struct Navigation {
    pub next_id: u64,
    pub graph: navigation::Graph,
    pub path_request_deque: VecDeque<path::Request>,
    pub path_result_vec: Vec<path::Result>,
    pub path_task_vec: Vec<path::Task>,
    pub path_solver: path::Solver,
}

impl Navigation {
    pub fn new(grid: &Grid) -> Self {
        let next_id = 0;
        let graph = navigation::Graph::new(grid.world_radius_in_cells);
        let path_request_deque = VecDeque::new();
        let path_result_vec = Vec::new();
        let path_task_vec = Vec::new();
        let path_solver = path::Solver::new();

        Self {
            next_id,
            graph,
            path_request_deque,
            path_result_vec,
            path_task_vec,
            path_solver,
        }
    }

    pub fn make_request(start: IVec3, end: IVec3, navigation: &mut Navigation) -> path::ID {
        let path_id = path::ID(navigation.next_id);

        navigation.next_id += 1;

        let path_request = path::Request {
            path_id,
            start,
            end,
        };

        navigation.path_request_deque.push_back(path_request);

        path_id
    }

    pub fn tick(world: &World, navigation: &mut Navigation) {}
}
