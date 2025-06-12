use crate::simulation::{consts::*, physics::aabb::AABB};
use glam::Vec3;
use std::f32::EPSILON;

struct ApproxEqCase {
    description: String,
    aabb1: AABB,
    aabb2: AABB,
    expected_is_equal: bool,
}

impl ApproxEqCase {
    pub fn check(&self) {
        let is_equal = self.aabb1.approx_eq(self.aabb2, EPSILON);

        assert_eq!(is_equal, self.expected_is_equal, "{:?}", self.description);
    }
}

#[test]
fn approx_eq() {
    let aabb1 = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let test_cases = vec![
        ApproxEqCase {
            description: String::from("Equivalent AABBs"),
            aabb1,
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: true,
        },
        ApproxEqCase {
            description: "AABBs that differ only by 2.0 * EPSILON in center.x".to_string(),
            aabb1,
            aabb2: AABB::new(
                Vec3::new(2.0 * EPSILON, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqCase {
            description: "AABBs that differ only by 2.0 * EPSILON in center.y".to_string(),
            aabb1,
            aabb2: AABB::new(
                Vec3::new(0.0, 2.0 * EPSILON, 0.0),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqCase {
            description: "AABBs that differ only by 2.0 * EPSILON in center.z".to_string(),
            aabb1,
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 2.0 * EPSILON),
                Vec3::new(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqCase {
            description: "AABBs that differ only by 4.0 * EPSILON in size.x".to_string(),
            aabb1,
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE + 4.0 * EPSILON, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
        ApproxEqCase {
            description: "AABBs that differ only by -4.0 * EPSILON in size.x".to_string(),
            aabb1,
            aabb2: AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE - 4.0 * EPSILON, BLOCK_SIZE, BLOCK_SIZE),
            ),
            expected_is_equal: false,
        },
    ];

    for case in test_cases {
        case.check();
    }
}
