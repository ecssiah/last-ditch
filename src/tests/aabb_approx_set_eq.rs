use glam::Vec3;
use std::f32::EPSILON;
use crate::simulation::{physics::aabb::AABB, consts::*};

struct ApproxSetEqCase {
    description: String,
    aabb_list1: Vec<AABB>,
    aabb_list2: Vec<AABB>,
    expected_is_equal: bool,
}

impl ApproxSetEqCase {
    pub fn check(&self) {
        let is_equal = AABB::approx_set_eq(&self.aabb_list1, &self.aabb_list2, EPSILON);

        assert_eq!(is_equal, self.expected_is_equal, "{:?}", self.description);
    }
}

#[test]
fn approx_set_eq() {
    let test_cases = vec![
        ApproxSetEqCase {
            description: String::from("Equivalent AABB lists"),
            aabb_list1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: true,
        },
        ApproxSetEqCase {
            description: String::from(
                "AABB lists that differ only by EPSILON in one center coordinate",
            ),
            aabb_list1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(EPSILON, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: true,
        },
        ApproxSetEqCase {
            description: String::from(
                "AABB lists that differ only by 2.0 * EPSILON in one center coordinate",
            ),
            aabb_list1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(2.0 * EPSILON, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: false,
        },
        ApproxSetEqCase {
            description: String::from(
                "AABB lists that differ only by 4.0 * EPSILON in one size coordinate",
            ),
            aabb_list1: vec![
                AABB::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(BLOCK_SIZE + 4.0 * EPSILON, BLOCK_SIZE, BLOCK_SIZE),
                ),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: false,
        },
        ApproxSetEqCase {
            description: String::from("AABB lists with different lengths"),
            aabb_list1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
            ],
            aabb_list2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(BLOCK_SIZE)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE)),
            ],
            expected_is_equal: false,
        },
    ];

    for test_case in test_cases {
        test_case.check();
    }
}
