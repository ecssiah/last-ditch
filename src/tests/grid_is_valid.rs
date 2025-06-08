use crate::simulation::{consts::*, world::World};
use glam::IVec3;

struct IsValidCase {
    description: String,
    grid_position: IVec3,
    expected_valid: bool,
}

impl IsValidCase {
    pub fn check(&self, world: &World) {
        let valid = world.grid.valid_position(self.grid_position);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn is_valid() {
    let test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    let world_boundary = test_world.grid.world_boundary as i32;

    let test_cases = vec![
        IsValidCase {
            description: "Grid Position: (0, 0, 0)".to_string(),
            grid_position: IVec3::new(0, 0, 0),
            expected_valid: true,
        },
        IsValidCase {
            description: "Grid Position: (GRID_BOUNDARY, GRID_BOUNDARY, GRID_BOUNDARY)".to_string(),
            grid_position: IVec3::new(world_boundary, world_boundary, world_boundary),
            expected_valid: true,
        },
        IsValidCase {
            description: "Grid Position: (-GRID_BOUNDARY, -GRID_BOUNDARY, -GRID_BOUNDARY)"
                .to_string(),
            grid_position: IVec3::new(-world_boundary, -world_boundary, -world_boundary),
            expected_valid: true,
        },
        IsValidCase {
            description: "Grid Position: (GRID_BOUNDARY + 1, GRID_BOUNDARY + 1, GRID_BOUNDARY + 1)"
                .to_string(),
            grid_position: IVec3::new(world_boundary + 1, world_boundary + 1, world_boundary + 1),
            expected_valid: false,
        },
        IsValidCase {
            description:
                "Grid Position: (-GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1)"
                    .to_string(),
            grid_position: IVec3::new(-world_boundary - 1, -world_boundary - 1, -world_boundary - 1),
            expected_valid: false,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
