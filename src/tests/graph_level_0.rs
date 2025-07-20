use crate::simulation::{
    self,
    consts::{MOVEMENT_COST_DIAGONAL, MOVEMENT_COST_STRAIGHT},
    state::{
        world::graph::{edge, Edge, Level, Node},
        World,
    },
};
use glam::IVec3;

struct NodeValidationCase {
    description: String,
    position: IVec3,
    expected_node: Option<Node>,
}

impl NodeValidationCase {
    pub fn check(&self, world: &World) {
        let graph_buffer = world.graph_buffer_lock.read().unwrap();
        let graph = graph_buffer.get();

        let level_0 = &graph.level_0;

        assert!(!level_0.region_node_map.is_empty());

        let node = Level::get_node(self.position, &level_0);

        assert_eq!(node, self.expected_node.as_ref(), "{:?}", self.description);
    }
}

#[test]
fn node_validation() {
    let kind = simulation::Kind::GraphTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        NodeValidationCase {
            description: "valid node 1".to_string(),
            position: IVec3::new(0, -3, 0),
            expected_node: Some(Node::new(IVec3::new(0, -3, 0), IVec3::new(0, -3, 0), 0)),
        },
        NodeValidationCase {
            description: "invalid node 1".to_string(),
            position: IVec3::new(0, -2, 0),
            expected_node: None,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct EdgeValidationCase {
    pub description: String,
    pub position1: IVec3,
    pub position2: IVec3,
    pub expected_edge: Option<Edge>,
}

impl EdgeValidationCase {
    pub fn check(&self, world: &World) {
        let graph_buffer = world.graph_buffer_lock.write().unwrap();
        let graph = graph_buffer.get();

        let level_0 = &graph.level_0;

        assert!(!level_0.edge_map.is_empty());

        let edge = Level::get_edge(self.position1, self.position2, &level_0.edge_map);

        assert_eq!(edge, self.expected_edge.as_ref(), "{:?}", self.description);
    }
}

#[test]
fn edge_validation() {
    let kind = simulation::Kind::GraphTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        EdgeValidationCase {
            description: "test case 1".to_string(),
            position1: IVec3::new(0, -3, 0),
            position2: IVec3::new(1, -3, 0),
            expected_edge: Some(Edge::new(
                Node::new(IVec3::new(0, -3, 0), IVec3::new(0, -3, 0), 0),
                Node::new(IVec3::new(1, -3, 0), IVec3::new(1, -3, 0), 0),
                edge::Kind::External,
                MOVEMENT_COST_STRAIGHT,
                0,
            )),
        },
        EdgeValidationCase {
            description: "test case 2".to_string(),
            position1: IVec3::new(-2, -2, 11),
            position2: IVec3::new(-1, -3, 11),
            expected_edge: Some(Edge::new(
                Node::new(IVec3::new(-2, -2, 11), IVec3::new(-2, -2, 11), 0),
                Node::new(IVec3::new(-1, -3, 11), IVec3::new(-1, -3, 11), 0),
                edge::Kind::External,
                MOVEMENT_COST_DIAGONAL,
                0,
            )),
        },
        EdgeValidationCase {
            description: "test case 3".to_string(),
            position1: IVec3::new(4, -3, 0),
            position2: IVec3::new(5, -4, 0),
            expected_edge: None,
        },
        EdgeValidationCase {
            description: "test case 4".to_string(),
            position1: IVec3::new(0, -3, 0),
            position2: IVec3::new(0, -2, 0),
            expected_edge: None,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
