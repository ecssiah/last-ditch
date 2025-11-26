use crate::simulation::state::navigation::{self, Graph, path};
use std::collections::{BinaryHeap, HashMap};
use ultraviolet::IVec3;

pub enum StepResult {
    Continue,
    Found(Vec<IVec3>),
    Impossible,
}

pub struct AStarNode {
    pub position: IVec3,
    pub g_cost: i32,
    pub f_cost: i32,
    pub parent: IVec3,
}

#[derive(Eq, PartialEq)]
struct OpenNode {
    pub position: IVec3,
    pub f_cost: i32,
}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct State {
    pub path_id: path::ID,
    pub finished: bool,
    start: IVec3,
    end: IVec3,
    open_heap: BinaryHeap<OpenNode>,
    origin: HashMap<IVec3, IVec3>,
    g_cost: HashMap<IVec3, i32>,
}

impl State {
    pub fn new(path_id: path::ID, start: IVec3, end: IVec3) -> Self {
        let mut open_heap = BinaryHeap::new();
        open_heap.push(OpenNode {
            position: start,
            f_cost: 0,
        });

        Self {
            path_id,
            finished: false,
            start,
            end,
            open_heap,
            origin: Default::default(),
            g_cost: Default::default(),
        }
    }

    pub fn step(&mut self, graph: &navigation::Graph) -> StepResult {
        let Some(open_node) = self.open_heap.pop() else {
            self.finished = true;

            return StepResult::Impossible;
        };

        if open_node.position == self.end {
            self.finished = true;

            let path = self.reconstruct(open_node.position);

            return StepResult::Found(path);
        }

        let current_g_cost = *self.g_cost.get(&open_node.position).unwrap_or(&0);

        for neighbor_position in Graph::get_neighbor_positions(open_node.position, graph) {
            let tentative_g_cost = current_g_cost + Graph::get_cost(neighbor_position, graph);

            if tentative_g_cost < *self.g_cost.get(&neighbor_position).unwrap_or(&i32::MAX) {
                self.origin.insert(neighbor_position, open_node.position);
                self.g_cost.insert(neighbor_position, tentative_g_cost);

                let f_cost =
                    tentative_g_cost + Self::manhattan_distance(neighbor_position, self.end);

                self.open_heap.push(OpenNode {
                    position: neighbor_position,
                    f_cost,
                });
            }
        }

        StepResult::Continue
    }

    fn reconstruct(&self, mut current: IVec3) -> Vec<IVec3> {
        let mut output_path = vec![current];

        while let Some(step_position) = self.origin.get(&current) {
            current = *step_position;
            output_path.push(current);
        }

        output_path.reverse();

        output_path
    }

    #[inline]
    fn manhattan_distance(a: IVec3, b: IVec3) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
    }
}
