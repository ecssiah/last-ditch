use crate::simulation::state::physics::aabb::AABB;
use glam::Vec3;

struct IntersectsAABBCase {
    description: String,
    aabb1: AABB,
    aabb2: AABB,
    expected_intersects_result: bool,
}

impl IntersectsAABBCase {
    pub fn check(&self) {
        let intersection_result = self.aabb1.intersects(self.aabb2);

        assert_eq!(
            intersection_result, self.expected_intersects_result,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn cells_intersect_when_overlapping() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0));

    let test_cases = vec![
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, -0.5, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -0.5), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.5, 0.5, 0.5), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
    ];

    for case in test_cases {
        case.check();
    }
}

#[test]
fn cells_intersect_when_intersecting() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0));

    let test_cases = vec![
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(1.0)),
            expected_intersects_result: true,
        },
    ];

    for case in test_cases {
        case.check();
    }
}

#[test]
fn cells_do_not_intersect_when_separated() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0));

    let test_cases = vec![
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(1.5, 0.0, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: false,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-1.5, 0.0, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: false,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 1.5, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: false,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, -1.5, 0.0), Vec3::splat(1.0)),
            expected_intersects_result: false,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.5), Vec3::splat(1.0)),
            expected_intersects_result: false,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.5), Vec3::splat(1.0)),
            expected_intersects_result: false,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(1.5, 1.5, 1.5), Vec3::splat(1.0)),
            expected_intersects_result: false,
        },
        IntersectsAABBCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-1.5, -1.5, -1.5), Vec3::splat(1.0)),
            expected_intersects_result: false,
        },
    ];

    for case in test_cases {
        case.check();
    }
}
