use crate::simulation::world::{self, chunk};
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Graph {
    pub(crate) node_map: HashMap<IVec3, world::Node>,
    pub(crate) chunk_graph_map: HashMap<IVec3, chunk::Graph>,
}

impl Graph {
    pub fn new() -> Self {
        let graph = Graph {
            node_map: HashMap::new(),
            chunk_graph_map: HashMap::new(),
        };

        graph
    }

    pub fn get_node(&self, chunk_position: IVec3) -> Option<&world::Node> {
        self.node_map.get(&chunk_position)
    }

    pub fn add_node(&mut self, chunk_position: IVec3, node: world::Node) -> Option<world::Node> {
        self.node_map.insert(chunk_position, node)
    }

    pub fn get_node_mut(&mut self, chunk_position: IVec3) -> Option<&mut world::Node> {
        self.node_map.get_mut(&chunk_position)
    }

    pub fn get_chunk_graph(&self, chunk_position: IVec3) -> Option<&chunk::Graph> {
        self.chunk_graph_map.get(&chunk_position)
    }

    pub fn get_chunk_graph_mut(&mut self, chunk_position: IVec3) -> Option<&mut chunk::Graph> {
        self.chunk_graph_map.get_mut(&chunk_position)
    }

    pub fn add_chunk_graph(
        &mut self,
        chunk_position: IVec3,
        chunk_graph: chunk::Graph,
    ) -> Option<chunk::Graph> {
        self.chunk_graph_map.insert(chunk_position, chunk_graph)
    }

    pub fn add_edge(&mut self, chunk_position: IVec3, edge: world::Edge) {
        if let Some(node) = self.node_map.get_mut(&chunk_position) {
            node.edge_map
                .entry(edge.from_position)
                .or_insert_with(Vec::new)
                .push(edge);
        }
    }

    pub fn create_edges(
        &mut self,
        from_chunk_position: IVec3,
        to_chunk_position: IVec3,
        from_position: IVec3,
        to_position: IVec3,
        clearance: i32,
        cost: f32,
    ) {
        let node1 = self.get_node(from_chunk_position);
        let node2 = self.get_node(to_chunk_position);

        if node1.is_some() && node2.is_some() {
            self.add_edge(
                from_chunk_position,
                world::Edge {
                    from_chunk_position: from_chunk_position,
                    to_chunk_position: to_chunk_position,
                    from_position: from_position,
                    to_position: to_position,
                    clearance: clearance as u32,
                    cost,
                },
            );

            self.add_edge(
                to_chunk_position,
                world::Edge {
                    from_chunk_position: to_chunk_position,
                    to_chunk_position: from_chunk_position,
                    from_position: to_position,
                    to_position: from_position,
                    clearance: clearance as u32,
                    cost,
                },
            );
        }
    }
}
