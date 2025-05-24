use std::f32::EPSILON;

use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, BLOCK_SIZE};

struct ApproxEqTestCase {
    aabb1: AABB,
    aabb2: AABB,
    expected_is_equal: bool,
}

#[test]
fn approx_eq() {
    let test_cases = vec![
        ApproxEqTestCase {
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: true,
        },
        ApproxEqTestCase {
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(2.0 * EPSILON, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqTestCase {
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 2.0 * EPSILON, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqTestCase {
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 2.0 * EPSILON),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqTestCase {
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE + 4.0 * EPSILON, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqTestCase {
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE - 4.0 * EPSILON, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
    ];

    for test_case in test_cases {
        let is_equal = test_case.aabb1.approx_eq(&test_case.aabb2, EPSILON);

        assert_eq!(is_equal, test_case.expected_is_equal);
    }
}
