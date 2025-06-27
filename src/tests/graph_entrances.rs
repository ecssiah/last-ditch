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
}

impl EntranceValidationCase {
    pub fn check(&self, world: &World) {
        let test_entrance_vec: Vec<&world::graph::Entrance> = world
            .graph
            .entrance_vec
            .iter()
            .filter(|entrance| {
                let original_match = entrance.region1_coordinates == self.region1_coordinates
                    && entrance.region2_coordinates == self.region2_coordinates;

                let symmetric_match = entrance.region1_coordinates == self.region2_coordinates
                    && entrance.region2_coordinates == self.region1_coordinates;

                original_match || symmetric_match
            })
            .collect();

        assert_eq!(
            test_entrance_vec.len(),
            self.expected_number_of_entrances,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn validate_entrances() {
    let mode = simulation::Mode::GraphTest;

    let mut world = World::new(mode);
    world.setup();

    let test_cases = vec![
        EntranceValidationCase {
            description: "expanded entrance".to_string(),
            region1_coordinates: IVec3::new(0, 0, 0),
            region2_coordinates: IVec3::new(1, 0, 0),
            expected_number_of_entrances: 1,
        },
        EntranceValidationCase {
            description: "constricted entrance".to_string(),
            region1_coordinates: IVec3::new(0, 0, 0),
            region2_coordinates: IVec3::new(-1, 0, 0),
            expected_number_of_entrances: 1,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
