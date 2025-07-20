use crate::simulation::{
    self,
    consts::{MOVEMENT_COST_DIAGONAL, MOVEMENT_COST_STRAIGHT},
    state::{
        world::graph::{edge, Edge, Graph, Node},
        World,
    },
};
use glam::IVec3;

struct PathLocalEdgeCountCase {
    description: String,
    start_position: IVec3,
    end_position: IVec3,
    expected_edge_vec_len: usize,
}

impl PathLocalEdgeCountCase {
    pub fn check(&self, world: &World) {
        let graph_buffer = world.graph_buffer_lock.write().unwrap();
        let graph = graph_buffer.get();

        let edge_vec =
            Graph::find_local_path(self.start_position, self.end_position, &graph.level_0);

        assert_eq!(
            edge_vec.len(),
            self.expected_edge_vec_len,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn path_local_edge_count() {
    let kind = simulation::Kind::GraphTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        PathLocalEdgeCountCase {
            description: "test case 1".to_string(),
            start_position: IVec3::new(0, -3, 0),
            end_position: IVec3::new(4, -3, 0),
            expected_edge_vec_len: 4,
        },
        PathLocalEdgeCountCase {
            description: "test case 2".to_string(),
            start_position: IVec3::new(0, -3, 0),
            end_position: IVec3::new(-4, -3, 0),
            expected_edge_vec_len: 4,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PathLocalEdgeVecCase {
    description: String,
    start_position: IVec3,
    end_position: IVec3,
    expected_edge_vec: Vec<Edge>,
}

impl PathLocalEdgeVecCase {
    pub fn check(&self, world: &World) {
        let graph_buffer = world.graph_buffer_lock.write().unwrap();
        let graph = graph_buffer.get();

        let edge_vec =
            Graph::find_local_path(self.start_position, self.end_position, &graph.level_0);

        assert_eq!(edge_vec, self.expected_edge_vec, "{:?}", self.description);
    }
}

#[test]
fn path_local_edge_vec() {
    let kind = simulation::Kind::GraphTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        PathLocalEdgeVecCase {
            description: "test case 1".to_string(),
            start_position: IVec3::new(0, -3, 0),
            end_position: IVec3::new(4, -3, 0),
            expected_edge_vec: vec![
                Edge::new(
                    Node::new(IVec3::new(0, -3, 0), IVec3::new(0, -3, 0), 0),
                    Node::new(IVec3::new(1, -3, 0), IVec3::new(1, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(1, -3, 0), IVec3::new(1, -3, 0), 0),
                    Node::new(IVec3::new(2, -3, 0), IVec3::new(2, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(2, -3, 0), IVec3::new(2, -3, 0), 0),
                    Node::new(IVec3::new(3, -3, 0), IVec3::new(3, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(3, -3, 0), IVec3::new(3, -3, 0), 0),
                    Node::new(IVec3::new(4, -3, 0), IVec3::new(4, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
            ],
        },
        PathLocalEdgeVecCase {
            description: "test case 2".to_string(),
            start_position: IVec3::new(0, -3, 0),
            end_position: IVec3::new(-4, -3, 0),
            expected_edge_vec: vec![
                Edge::new(
                    Node::new(IVec3::new(0, -3, 0), IVec3::new(0, -3, 0), 0),
                    Node::new(IVec3::new(-1, -3, 0), IVec3::new(-1, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(-1, -3, 0), IVec3::new(-1, -3, 0), 0),
                    Node::new(IVec3::new(-2, -3, 0), IVec3::new(-2, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(-2, -3, 0), IVec3::new(-2, -3, 0), 0),
                    Node::new(IVec3::new(-3, -3, 0), IVec3::new(-3, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(-3, -3, 0), IVec3::new(-3, -3, 0), 0),
                    Node::new(IVec3::new(-4, -3, 0), IVec3::new(-4, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
            ],
        },
        PathLocalEdgeVecCase {
            description: "test case 2".to_string(),
            start_position: IVec3::new(9, -3, 0),
            end_position: IVec3::new(5, -4, 0),
            expected_edge_vec: vec![
                Edge::new(
                    Node::new(IVec3::new(9, -3, 0), IVec3::new(9, -3, 0), 0),
                    Node::new(IVec3::new(8, -3, 0), IVec3::new(8, -3, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(8, -3, 0), IVec3::new(8, -3, 0), 0),
                    Node::new(IVec3::new(7, -4, 0), IVec3::new(7, -4, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_DIAGONAL,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(7, -4, 0), IVec3::new(7, -4, 0), 0),
                    Node::new(IVec3::new(6, -4, 0), IVec3::new(6, -4, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
                Edge::new(
                    Node::new(IVec3::new(6, -4, 0), IVec3::new(6, -4, 0), 0),
                    Node::new(IVec3::new(5, -4, 0), IVec3::new(5, -4, 0), 0),
                    edge::Kind::External,
                    MOVEMENT_COST_STRAIGHT,
                    0,
                ),
            ],
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
