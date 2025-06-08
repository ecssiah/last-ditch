use crate::simulation::{
    consts::*,
    world::{grid, World},
};
use glam::IVec3;

struct BoundaryContactDirectionsCase {
    description: String,
    position: IVec3,
    expected_boundary_contact_direction_list: Vec<grid::Direction>,
}

impl BoundaryContactDirectionsCase {
    pub fn check(&self, world: &World) {
        let boundary_contact_direction_list =
            world.grid.boundary_contact_direction_list(self.position);

        assert_eq!(
            boundary_contact_direction_list, self.expected_boundary_contact_direction_list,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn boundary_contact_direction_list() {
    let world = World::new(TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS);

    let chunk_radius = world.grid.chunk_radius as i32;
    let world_boundary = world.grid.world_boundary as i32;

    let test_cases = vec![
        BoundaryContactDirectionsCase {
            description: "origin chunk min".to_string(),
            position: IVec3::splat(-chunk_radius),
            expected_boundary_contact_direction_list: vec![
                grid::Direction::XnYoZo,
                grid::Direction::XoYnZo,
                grid::Direction::XoYoZn,
            ],
        },
        BoundaryContactDirectionsCase {
            description: "origin chunk min x direction".to_string(),
            position: IVec3::new(-chunk_radius, 0, 0),
            expected_boundary_contact_direction_list: vec![grid::Direction::XnYoZo],
        },
        BoundaryContactDirectionsCase {
            description: "origin chunk min x-y direction".to_string(),
            position: IVec3::new(-chunk_radius, -chunk_radius, 0),
            expected_boundary_contact_direction_list: vec![
                grid::Direction::XnYoZo,
                grid::Direction::XoYnZo,
            ],
        },
        BoundaryContactDirectionsCase {
            description: "origin chunk origin".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_boundary_contact_direction_list: vec![],
        },
        BoundaryContactDirectionsCase {
            description: "origin chunk max".to_string(),
            position: IVec3::splat(chunk_radius),
            expected_boundary_contact_direction_list: vec![
                grid::Direction::XpYoZo,
                grid::Direction::XoYpZo,
                grid::Direction::XoYoZp,
            ],
        },
        BoundaryContactDirectionsCase {
            description: "origin chunk max x direction".to_string(),
            position: IVec3::new(chunk_radius, 0, 0),
            expected_boundary_contact_direction_list: vec![grid::Direction::XpYoZo],
        },
        BoundaryContactDirectionsCase {
            description: "origin chunk max x-y direction".to_string(),
            position: IVec3::new(chunk_radius, chunk_radius, 0),
            expected_boundary_contact_direction_list: vec![
                grid::Direction::XpYoZo,
                grid::Direction::XoYpZo,
            ],
        },
        BoundaryContactDirectionsCase {
            description: "max chunk max".to_string(),
            position: IVec3::splat(world_boundary),
            expected_boundary_contact_direction_list: vec![],
        },
        BoundaryContactDirectionsCase {
            description: "min chunk min".to_string(),
            position: IVec3::splat(-world_boundary),
            expected_boundary_contact_direction_list: vec![],
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
