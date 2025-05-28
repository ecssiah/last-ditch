use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, BLOCK_SIZE};
use std::f32::EPSILON;

struct SweepTestCase {
    description: String,
    aabb1: AABB,
    aabb2: AABB,
    expected_aabb: AABB,
}

#[test]
fn equal() {
    let test_cases = vec![
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 0, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (-1, -1, -1) to block at (-1, -1, -1)"),
            aabb1: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (1, 1, 1) to block at (1, 1, 1)"),
            aabb1: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
    ];

    for test_case in test_cases {
        let aabb = AABB::sweep(test_case.aabb1, test_case.aabb2);

        let is_equal = aabb.approx_eq(test_case.expected_aabb, EPSILON);

        assert!(is_equal, "{:?}", test_case.description);
    }
}

#[test]
fn x_axis() {
    let test_cases = vec![
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 0, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0.5, 0, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.25, 0.0, 0.0),
                Vec3::new(1.5 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (1, 0, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.5, 0.0, 0.0),
                Vec3::new(2.0 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (2.0, 0, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(2.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(3.0 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (-1, 0, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(-0.5, 0.0, 0.0),
                Vec3::new(2.0 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
    ];

    for test_case in test_cases {
        let aabb = AABB::sweep(test_case.aabb1, test_case.aabb2);

        let is_equal = aabb.approx_eq(test_case.expected_aabb, EPSILON);

        assert!(is_equal, "{:?}", test_case.description);
    }
}

#[test]
fn y_axis() {
    let test_cases = vec![
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 0.5, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.25, 0.0),
                Vec3::new(BLOCK_SIZE, 1.5 * BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 1, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.5, 0.0),
                Vec3::new(BLOCK_SIZE, 2.0 * BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 1, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 2.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(BLOCK_SIZE, 3.0 * BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, -1, 0)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, -0.5, 0.0),
                Vec3::new(BLOCK_SIZE, 2.0 * BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
    ];

    for test_case in test_cases {
        let aabb = AABB::sweep(test_case.aabb1, test_case.aabb2);

        let is_equal = aabb.approx_eq(test_case.expected_aabb, EPSILON);

        assert!(is_equal, "{:?}", test_case.description);
    }
}

#[test]
fn z_axis() {
    let test_cases = vec![
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 0, 0.5)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 0.25),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 1.5 * BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 0, 1)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 0.5),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 2.0 * BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 0, 2)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 2.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 3.0 * BLOCK_SIZE),
            ),
        },
        SweepTestCase {
            description: String::from("Sweep block at (0, 0, 0) to block at (0, 0, -1)"),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, -0.5),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 2.0 * BLOCK_SIZE),
            ),
        },
    ];

    for test_case in test_cases {
        let aabb = AABB::sweep(test_case.aabb1, test_case.aabb2);

        let is_equal = aabb.approx_eq(test_case.expected_aabb, EPSILON);

        assert!(is_equal, "{:?}", test_case.description);
    }
}
