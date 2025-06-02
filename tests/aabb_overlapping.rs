use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, BLOCK_SIZE};

struct OverlapsAABBTestCase {
    description: String,
    aabb1: AABB,
    aabb2: AABB,
    expected_overlap_result: bool,
}

impl OverlapsAABBTestCase {
    pub fn check(&self) {
        let overlaps_result = self.aabb1.overlaps(self.aabb2);

        assert_eq!(
            overlaps_result, self.expected_overlap_result,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn blocks_overlap_when_overlapping() {
    let aabb1 = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, -0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -0.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.5, 0.5, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}

#[test]
fn blocks_do_not_overlap_when_intersecting() {
    let aabb1 = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}

#[test]
fn blocks_do_not_overlap_when_separated() {
    let aabb1 = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(1.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-1.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 1.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, -1.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(1.5, 1.5, 1.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-1.5, -1.5, -1.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}
