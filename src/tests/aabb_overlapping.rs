use crate::simulation::state::physics::aabb::AABB;
use glam::Vec3;

struct OverlapsAABBCase {
    description: String,
    aabb1: AABB,
    aabb2: AABB,
    expected_overlap_result: bool,
}

impl OverlapsAABBCase {
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
    let aabb1 = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0));

    let test_cases = vec![
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: true,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: true,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: true,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, -0.5, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: true,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::splat(1.0)),
            expected_overlap_result: true,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -0.5), Vec3::splat(1.0)),
            expected_overlap_result: true,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.5, 0.5, 0.5), Vec3::splat(1.0)),
            expected_overlap_result: true,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::splat(1.0)),
            expected_overlap_result: true,
        },
    ];

    for case in test_cases {
        case.check();
    }
}

#[test]
fn blocks_do_not_overlap_when_intersecting() {
    let aabb1 = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0));

    let test_cases = vec![
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
    ];

    for case in test_cases {
        case.check();
    }
}

#[test]
fn blocks_do_not_overlap_when_separated() {
    let aabb1 = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0));

    let test_cases = vec![
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(1.5, 0.0, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-1.5, 0.0, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 1.5, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, -1.5, 0.0), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, 1.5), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(0.0, 0.0, -1.5), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(1.5, 1.5, 1.5), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
        OverlapsAABBCase {
            description: "".to_string(),
            aabb1,
            aabb2: AABB::new(Vec3::new(-1.5, -1.5, -1.5), Vec3::splat(1.0)),
            expected_overlap_result: false,
        },
    ];

    for case in test_cases {
        case.check();
    }
}
