use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, BLOCK_SIZE};

struct OverlapsAABBTestCase {
    description: String,
    aabb: AABB,
    expected_overlap_result: bool,
}

#[test]
fn blocks_overlap_when_overlapping() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, -0.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, -0.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.5, 0.5, 0.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: true,
        },
    ];

    for test_case in test_cases {
        let overlaps_result = aabb.overlaps(&test_case.aabb);

        assert_eq!(
            overlaps_result, test_case.expected_overlap_result,
            "{:?}",
            test_case.description
        );
    }
}

#[test]
fn blocks_do_not_overlap_when_intersecting() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
    ];

    for test_case in test_cases {
        let overlaps_result = aabb.overlaps(&test_case.aabb);

        assert_eq!(
            overlaps_result, test_case.expected_overlap_result,
            "{:?}",
            test_case.description
        );
    }
}

#[test]
fn blocks_do_not_overlap_when_separated() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(1.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-1.5, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 1.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, -1.5, 0.0), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, 1.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(0.0, 0.0, -1.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(1.5, 1.5, 1.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
        OverlapsAABBTestCase {
            description: String::from(""),
            aabb: AABB::new(Vec3::new(-1.5, -1.5, -1.5), Vec3::splat(BLOCK_SIZE)),
            expected_overlap_result: false,
        },
    ];

    for test_case in test_cases {
        let overlaps_result = aabb.intersects(&test_case.aabb);

        assert_eq!(
            overlaps_result, test_case.expected_overlap_result,
            "{:?}",
            test_case.description
        );
    }
}
