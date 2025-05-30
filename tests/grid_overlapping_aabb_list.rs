use glam::Vec3;
use last_ditch::simulation::{
    consts::*,
    physics::aabb::AABB,
    world::{block, grid, World},
};
use std::f32::EPSILON;

struct OverlappingAABBTestCase {
    description: String,
    aabb: AABB,
    expected_aabb_list: Vec<AABB>,
}

#[test]
fn directions() {
    let test_world = World::new(1, 2);

    let test_cases = vec![
        OverlappingAABBTestCase {
            description: String::from("XoYoZo"),
            aabb: AABB::new(
                grid::Direction::XoYoZo.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_list: vec![block::aabb(0, 0, 0)],
        },
        OverlappingAABBTestCase {
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
        OverlappingAABBTestCase {
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
        OverlappingAABBTestCase {
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
        OverlappingAABBTestCase {
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
        OverlappingAABBTestCase {
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
        OverlappingAABBTestCase {
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
        OverlappingAABBTestCase {
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
        OverlappingAABBTestCase {
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
        OverlappingAABBTestCase {
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
        let aabb_list = test_world.grid.overlapping_aabb_list(test_case.aabb);

        let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

        assert!(is_equal, "{:?}", test_case.description);
    }
}
