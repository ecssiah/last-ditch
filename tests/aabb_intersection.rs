use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, BLOCK_SIZE};

struct IntersectsAABBTestCase {
    description: String,
    aabb1: AABB,
    aabb2: AABB,
    expected_intersects_result: bool,
}

impl IntersectsAABBTestCase {
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
fn blocks_intersect_when_overlapping() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, -0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -0.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.5, 0.5, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}

#[test]
fn blocks_intersect_when_intersecting() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: true,
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}

#[test]
fn blocks_do_not_intersect_when_separated() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(1.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-1.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 1.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, -1.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(1.5, 1.5, 1.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
        IntersectsAABBTestCase {
            description: "".to_string(),
            aabb1: aabb,
            aabb2: AABB::new(Vec3::new(-1.5, -1.5, -1.5), Vec3::splat(BLOCK_SIZE)),
            expected_intersects_result: false,
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}
