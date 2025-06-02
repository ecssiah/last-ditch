use glam::IVec3;
use last_ditch::simulation::{world::World, TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS};

struct IsValidTestCase {
    description: String,
    grid_position: IVec3,
    expected_is_valid: bool,
}

impl IsValidTestCase {
    pub fn check(&self, world: &World) {
        let is_valid = world.grid.is_valid_grid_position(self.grid_position);

        assert_eq!(is_valid, self.expected_is_valid, "{:?}", self.description);
    }
}

#[test]
fn is_valid() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        IsValidTestCase {
            description: "Grid Position: (0, 0, 0)".to_string(),
            grid_position: IVec3::new(0, 0, 0),
            expected_is_valid: true,
        },
        IsValidTestCase {
            description: "Grid Position: (GRID_BOUNDARY, GRID_BOUNDARY, GRID_BOUNDARY)".to_string(),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_is_valid: true,
        },
        IsValidTestCase {
            description: "Grid Position: (-GRID_BOUNDARY, -GRID_BOUNDARY, -GRID_BOUNDARY)"
                .to_string(),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_is_valid: true,
        },
        IsValidTestCase {
            description: "Grid Position: (GRID_BOUNDARY + 1, GRID_BOUNDARY + 1, GRID_BOUNDARY + 1)"
                .to_string(),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_is_valid: false,
        },
        IsValidTestCase {
            description:
                "Grid Position: (-GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1)"
                    .to_string(),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_is_valid: false,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
