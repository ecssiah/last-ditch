use crate::simulation::{
    self,
    state::{
        world::{self},
        World,
    },
};
use glam::IVec3;

struct EntranceValidationCase {
    description: String,
    region1_coordinates: IVec3,
    region2_coordinates: IVec3,
    expected_number_of_entrances: usize,
    expected_transitions_per_entrance: usize,
}

impl EntranceValidationCase {
    pub fn check(&self, world: &World) {
        let region1_id = u32::from(
            world
                .grid
                .chunk_coordinates_to_chunk_id(self.region1_coordinates),
        );

        let region2_id = u32::from(
            world
                .grid
                .chunk_coordinates_to_chunk_id(self.region2_coordinates),
        );

        let graph_buffer = world.graph_buffer_lock.read().unwrap();
        let graph = graph_buffer.get();

        let test_entrance_vec: Vec<&world::graph::Entrance> = graph
            .entrance_vec
            .iter()
            .filter(|entrance| {
                let original_match =
                    entrance.region1_id == region1_id && entrance.region2_id == region2_id;

                let symmetric_match =
                    entrance.region1_id == region2_id && entrance.region2_id == region1_id;

                original_match || symmetric_match
            })
            .collect();

        assert_eq!(
            test_entrance_vec.len(),
            self.expected_number_of_entrances,
            "{:?}",
            self.description
        );

        for entrance in &test_entrance_vec {
            assert_eq!(
                entrance.transition_vec.len(),
                self.expected_transitions_per_entrance,
                "{:?}",
                self.description
            );
        }
    }
}

#[test]
fn validate_entrances() {
    let mode = simulation::Kind::GraphTest;

    let mut world = World::new(mode);
    world.setup();

    let test_cases = vec![
        EntranceValidationCase {
            description: "vertical entrance".to_string(),
            region1_coordinates: IVec3::new(0, 0, 1),
            region2_coordinates: IVec3::new(0, 1, 1),
            expected_number_of_entrances: 2,
            expected_transitions_per_entrance: 2,
        },
        EntranceValidationCase {
            description: "expanded entrance".to_string(),
            region1_coordinates: IVec3::new(0, 0, 0),
            region2_coordinates: IVec3::new(1, 0, 0),
            expected_number_of_entrances: 1,
            expected_transitions_per_entrance: 3,
        },
        EntranceValidationCase {
            description: "constricted entrance".to_string(),
            region1_coordinates: IVec3::new(0, 0, 0),
            region2_coordinates: IVec3::new(-1, 0, 0),
            expected_number_of_entrances: 1,
            expected_transitions_per_entrance: 1,
        },
        EntranceValidationCase {
            description: "multiple entrance".to_string(),
            region1_coordinates: IVec3::new(0, 0, 0),
            region2_coordinates: IVec3::new(0, 0, -1),
            expected_number_of_entrances: 2,
            expected_transitions_per_entrance: 2,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
