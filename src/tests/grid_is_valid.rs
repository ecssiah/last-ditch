use crate::simulation::{consts::*, world::World};
use glam::IVec3;

struct IsValidCase {
    description: String,
    position: IVec3,
    expected_is_valid: bool,
}

impl IsValidCase {
    pub fn check(&self, world: &World) {
        let is_valid = world.grid.is_valid_position(self.position);

        assert_eq!(is_valid, self.expected_is_valid, "{:?}", self.description);
    }
}

#[test]
fn is_valid() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        IsValidCase {
            description: "Grid Position: (0, 0, 0)".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_is_valid: true,
        },
        IsValidCase {
            description: "Grid Position: (GRID_BOUNDARY, GRID_BOUNDARY, GRID_BOUNDARY)".to_string(),
            position: IVec3::new(boundary, boundary, boundary),
            expected_is_valid: true,
        },
        IsValidCase {
            description: "Grid Position: (-GRID_BOUNDARY, -GRID_BOUNDARY, -GRID_BOUNDARY)"
                .to_string(),
            position: IVec3::new(-boundary, -boundary, -boundary),
            expected_is_valid: true,
        },
        IsValidCase {
            description: "Grid Position: (GRID_BOUNDARY + 1, GRID_BOUNDARY + 1, GRID_BOUNDARY + 1)"
                .to_string(),
            position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_is_valid: false,
        },
        IsValidCase {
            description:
                "Grid Position: (-GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1)"
                    .to_string(),
            position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_is_valid: false,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
