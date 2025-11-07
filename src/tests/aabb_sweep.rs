use crate::simulation::state::physics::aabb::AABB;
use std::f32::EPSILON;
use ultraviolet::Vec3;

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
            description: "Sweep cell at (0, 0, 0) to cell at (0, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::broadcast(0.0), Vec3::new(1.0, 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (-1, -1, -1) to cell at (-1, -1, -1)".to_string(),
            aabb1: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (1, 1, 1) to cell at (1, 1, 1)".to_string(),
            aabb1: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
        },
    ];

    for case in test_cases {
        case.check();
    }
}

#[test]
fn x_axis() {
    let test_cases = vec![
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::broadcast(0.0), Vec3::new(1.0, 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0.5, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.25, 0.0, 0.0), Vec3::new(1.5 * 1.0, 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (1, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::new(2.0 * 1.0, 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (2.0, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(2.0, 0.0, 0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(3.0 * 1.0, 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (-1, 0, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::new(2.0 * 1.0, 1.0, 1.0)),
        },
    ];

    for case in test_cases {
        case.check();
    }
}

#[test]
fn y_axis() {
    let test_cases = vec![
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, 0.5, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.0, 0.25, 0.0), Vec3::new(1.0, 1.5 * 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, 1, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::new(1.0, 2.0 * 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, 1, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.0, 2.0, 0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 3.0 * 1.0, 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, -1, 0)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(1.0, 2.0 * 1.0, 1.0)),
        },
    ];

    for case in test_cases {
        case.check();
    }
}

#[test]
fn z_axis() {
    let test_cases = vec![
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, 0, 0.5)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.0, 0.0, 0.25), Vec3::new(1.0, 1.0, 1.5 * 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, 0, 1)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::new(1.0, 1.0, 2.0 * 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, 0, 2)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 2.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 3.0 * 1.0)),
        },
        SweepCase {
            description: "Sweep cell at (0, 0, 0) to cell at (0, 0, -1)".to_string(),
            aabb1: AABB::new(Vec3::broadcast(0.0), Vec3::broadcast(1.0)),
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::broadcast(1.0)),
            expected_aabb: AABB::new(Vec3::new(0.0, 0.0, -0.5), Vec3::new(1.0, 1.0, 2.0 * 1.0)),
        },
    ];

    for case in test_cases {
        case.check();
    }
}
