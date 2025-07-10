use crate::simulation::state::world::graph;

pub struct Travel {
    pub regional_path_vec: Vec<graph::Node>,
    pub local_path_vec: Vec<graph::Node>,
}