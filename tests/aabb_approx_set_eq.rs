use std::f32::EPSILON;

use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, BLOCK_SIZE};

struct ApproxSetEqTestCase {
    description: String,
    aabb_list1: Vec<AABB>,
    aabb_list2: Vec<AABB>,
    expected_is_equal: bool,
}

#[test]
fn approx_set_eq() {
    let test_cases = vec![
        ApproxSetEqTestCase {
            description: String::from("Equivalent AABB lists"),
            aabb_list1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: true,
        },
        ApproxSetEqTestCase {
            description: String::from(
                "AABB lists that differ only by EPSILON in one center coordinate",
            ),
            aabb_list1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(EPSILON, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: true,
        },
        ApproxSetEqTestCase {
            description: String::from(
                "AABB lists that differ only by 2.0 * EPSILON in one center coordinate",
            ),
            aabb_list1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(2.0 * EPSILON, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: false,
        },
        ApproxSetEqTestCase {
            description: String::from(
                "AABB lists that differ only by 4.0 * EPSILON in one size coordinate",
            ),
            aabb_list1: vec![
                AABB::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(BLOCK_SIZE + 4.0 * EPSILON, BLOCK_SIZE, BLOCK_SIZE),
                ),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: false,
        },
        ApproxSetEqTestCase {
            description: String::from("AABB lists with different lengths"),
            aabb_list1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: false,
        },
    ];

    for test_case in test_cases {
        let is_equal = AABB::approx_set_eq(&test_case.aabb_list1, &test_case.aabb_list2, EPSILON);

        assert_eq!(
            is_equal, test_case.expected_is_equal,
            "{:?}",
            test_case.description
        );
    }
}
