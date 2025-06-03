use glam::Vec3;
use last_ditch::simulation::{
    consts::*,
    physics::aabb::AABB,
    world::{block, grid, World},
};
use std::f32::EPSILON;

struct OverlappingAABBCase {
    description: String,
    aabb: AABB,
    expected_aabb_list: Vec<AABB>,
}

impl OverlappingAABBCase {
    pub fn check(&self, world: &World) {
        let aabb_list = world.grid.overlapping_aabb_list(self.aabb);

        let is_equal = AABB::approx_set_eq(&aabb_list, &self.expected_aabb_list, EPSILON);

        assert!(is_equal, "{:?}", self.description);
    }
}

#[test]
fn directions() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let test_cases = vec![
        OverlappingAABBCase {
            description: String::from("XoYoZo"),
            aabb: AABB::new(
                grid::Direction::XoYoZo.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![block::aabb(0, 0, 0)],
        },
        OverlappingAABBCase {
            description: String::from("XpYpZp"),
            aabb: AABB::new(
                grid::Direction::XpYpZp.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(0, 0, 0),
                block::aabb(1, 0, 0),
                block::aabb(0, 1, 0),
                block::aabb(1, 1, 0),
                block::aabb(0, 0, 1),
                block::aabb(1, 0, 1),
                block::aabb(0, 1, 1),
                block::aabb(1, 1, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XpYpZn"),
            aabb: AABB::new(
                grid::Direction::XpYpZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(0, 0, -1),
                block::aabb(1, 0, -1),
                block::aabb(0, 1, -1),
                block::aabb(1, 1, -1),
                block::aabb(0, 0, 0),
                block::aabb(1, 0, 0),
                block::aabb(0, 1, 0),
                block::aabb(1, 1, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XpYnZp"),
            aabb: AABB::new(
                grid::Direction::XpYnZp.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(0, -1, 0),
                block::aabb(1, -1, 0),
                block::aabb(0, 0, 0),
                block::aabb(1, 0, 0),
                block::aabb(0, -1, 1),
                block::aabb(1, -1, 1),
                block::aabb(0, 0, 1),
                block::aabb(1, 0, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XpYnZn"),
            aabb: AABB::new(
                grid::Direction::XpYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(0, -1, -1),
                block::aabb(1, -1, -1),
                block::aabb(0, 0, -1),
                block::aabb(1, 0, -1),
                block::aabb(0, -1, 0),
                block::aabb(1, -1, 0),
                block::aabb(0, 0, 0),
                block::aabb(1, 0, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XpYnZn"),
            aabb: AABB::new(
                grid::Direction::XpYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(0, -1, -1),
                block::aabb(1, -1, -1),
                block::aabb(0, 0, -1),
                block::aabb(1, 0, -1),
                block::aabb(0, -1, 0),
                block::aabb(1, -1, 0),
                block::aabb(0, 0, 0),
                block::aabb(1, 0, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XnYpZp"),
            aabb: AABB::new(
                grid::Direction::XnYpZp.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(-1, 0, 0),
                block::aabb(0, 0, 0),
                block::aabb(-1, 1, 0),
                block::aabb(0, 1, 0),
                block::aabb(-1, 0, 1),
                block::aabb(0, 0, 1),
                block::aabb(-1, 1, 1),
                block::aabb(0, 1, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XnYpZn"),
            aabb: AABB::new(
                grid::Direction::XnYpZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(-1, 0, -1),
                block::aabb(0, 0, -1),
                block::aabb(-1, 1, -1),
                block::aabb(0, 1, -1),
                block::aabb(-1, 0, 0),
                block::aabb(0, 0, 0),
                block::aabb(-1, 1, 0),
                block::aabb(0, 1, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XnYnZp"),
            aabb: AABB::new(
                grid::Direction::XnYnZp.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(-1, -1, 0),
                block::aabb(0, -1, 0),
                block::aabb(-1, 0, 0),
                block::aabb(0, 0, 0),
                block::aabb(-1, -1, 1),
                block::aabb(0, -1, 1),
                block::aabb(-1, 0, 1),
                block::aabb(0, 0, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XnYnZn"),
            aabb: AABB::new(
                grid::Direction::XnYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![
                block::aabb(0, 0, 0),
                block::aabb(-1, -1, -1),
                block::aabb(0, -1, -1),
                block::aabb(-1, 0, -1),
                block::aabb(0, 0, -1),
                block::aabb(-1, -1, 0),
                block::aabb(0, -1, 0),
                block::aabb(-1, 0, 0),
            ],
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
