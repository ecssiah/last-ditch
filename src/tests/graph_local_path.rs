use crate::simulation::{
    self,
    state::{
        world::graph::{Edge, Graph, Level, Node},
        World,
    },
};
use glam::IVec3;

struct PathLocalCase {
    description: String,
    start_position: IVec3,
    end_position: IVec3,
    expected_edge_vec_len: usize,
}

impl PathLocalCase {
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
fn path_local() {
    let kind = simulation::Kind::GraphTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        PathLocalCase {
            description: "test case 1".to_string(),
            start_position: IVec3::new(0, 0, 0),
            end_position: IVec3::new(4, 0, 0),
            expected_edge_vec_len: 4,
        },
        PathLocalCase {
            description: "test case 2".to_string(),
            start_position: IVec3::new(0, 0, 0),
            end_position: IVec3::new(-4, 0, 0),
            expected_edge_vec_len: 4,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
