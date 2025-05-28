use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, BLOCK_SIZE};

struct IntersectsAABBTestCase {
    description: String,
    aabb: AABB,
    expected_intersects_result: bool,
}

#[test]
fn blocks_intersect_when_overlapping() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, -0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, -0.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.5, 0.5, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
    ];

    for test_case in test_cases {
        let intersection_result = aabb.intersects(test_case.aabb);

        assert_eq!(
            intersection_result, test_case.expected_intersects_result,
            "{:?}",
            test_case.description
        );
    }
}

#[test]
fn blocks_intersect_when_intersecting() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
    ];

    for test_case in test_cases {
        let intersection_result = aabb.intersects(test_case.aabb);

        assert_eq!(
            intersection_result, test_case.expected_intersects_result,
            "{:?}",
            test_case.description
        );
    }
}

#[test]
fn blocks_do_not_intersect_when_separated() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(1.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-1.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 1.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, -1.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, 1.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, -1.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(1.5, 1.5, 1.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-1.5, -1.5, -1.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
    ];

    for test_case in test_cases {
        let intersection_result = aabb.intersects(test_case.aabb);

        assert_eq!(
            intersection_result, test_case.expected_intersects_result,
            "{:?}",
            test_case.description
        );
    }
}
