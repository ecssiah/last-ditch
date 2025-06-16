pub mod edge;
pub mod node;
pub mod region;
pub mod transition;

pub use edge::Edge;
pub use node::Node;
pub use region::Region;
pub use transition::Transition;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub regions: Vec<Region>,
    pub transitions: Vec<Transition>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            regions: Vec::new(),
            transitions: Vec::new(),
        }
    }
}
