use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, BLOCK_SIZE};
use std::f32::EPSILON;

struct ApproxEqTestCase {
    description: String,
    aabb1: AABB,
    aabb2: AABB,
    expected_is_equal: bool,
}

#[test]
fn approx_eq() {
    let test_cases = vec![
        ApproxEqTestCase {
            description: String::from("Equivalent AABBs"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: true,
        },
        ApproxEqTestCase {
            description: String::from("AABBs that differ only by 2.0 * EPSILON in center.x"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(2.0 * EPSILON, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqTestCase {
            description: String::from("AABBs that differ only by 2.0 * EPSILON in center.y"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 2.0 * EPSILON, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqTestCase {
            description: String::from("AABBs that differ only by 2.0 * EPSILON in center.z"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 2.0 * EPSILON),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqTestCase {
            description: String::from("AABBs that differ only by 4.0 * EPSILON in size.x"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE + 4.0 * EPSILON, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqTestCase {
            description: String::from("AABBs that differ only by -4.0 * EPSILON in size.x"),
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

        assert_eq!(
            is_equal, test_case.expected_is_equal,
            "{:?}",
            test_case.description
        );
    }
}
