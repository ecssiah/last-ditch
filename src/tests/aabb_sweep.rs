use crate::simulation::{physics::aabb::AABB, consts::*};
use glam::Vec3;
use std::f32::EPSILON;

struct SweepCase {
    description: String,
    aabb1: AABB,
    aabb2: AABB,
    expected_aabb: AABB,
}

impl SweepCase {
    pub fn check(&self) {
        let aabb = AABB::sweep(self.aabb1, self.aabb2);

        let is_equal = aabb.approx_eq(self.expected_aabb, EPSILON);

        assert!(is_equal, "{:?}", self.description);
    }
}

#[test]
fn equal() {
    let test_cases = vec![
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (-1, -1, -1) to block at (-1, -1, -1)".to_string(),
            aabb1: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (1, 1, 1) to block at (1, 1, 1)".to_string(),
            aabb1: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}

#[test]
fn x_axis() {
    let test_cases = vec![
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0.5, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.25, 0.0, 0.0),
                Vec3::new(1.5 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (1, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.5, 0.0, 0.0),
                Vec3::new(2.0 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (2.0, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(2.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(3.0 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (-1, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(-0.5, 0.0, 0.0),
                Vec3::new(2.0 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}

#[test]
fn y_axis() {
    let test_cases = vec![
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 0.5, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.25, 0.0),
                Vec3::new(BLOCK_SIZE, 1.5 * BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 1, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.5, 0.0),
                Vec3::new(BLOCK_SIZE, 2.0 * BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 1, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 2.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(BLOCK_SIZE, 3.0 * BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, -1, 0)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, -0.5, 0.0),
                Vec3::new(BLOCK_SIZE, 2.0 * BLOCK_SIZE, BLOCK_SIZE),
            ),
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}

#[test]
fn z_axis() {
    let test_cases = vec![
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 0, 0.5)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 0.25),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 1.5 * BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 0, 1)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 0.5),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 2.0 * BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 0, 2)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 2.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 3.0 * BLOCK_SIZE),
            ),
        },
        SweepCase {
            description: "Sweep block at (0, 0, 0) to block at (0, 0, -1)".to_string(),
            aabb1: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_aabb: AABB::new(
                Vec3::new(0.0, 0.0, -0.5),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 2.0 * BLOCK_SIZE),
            ),
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}
