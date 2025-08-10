use crate::simulation::state::physics::aabb::AABB;
use glam::Vec3;
use std::f32::EPSILON;

struct ApproxSetEqCase {
    description: String,
    aabb_vec1: Vec<AABB>,
    aabb_vec2: Vec<AABB>,
    expected_is_equal: bool,
}

impl ApproxSetEqCase {
    pub fn check(&self) {
        let is_equal = AABB::approx_set_eq(&self.aabb_vec1, &self.aabb_vec2, EPSILON);

        assert_eq!(is_equal, self.expected_is_equal, "{:?}", self.description);
    }
}

#[test]
fn approx_set_eq() {
    let test_cases = vec![
        ApproxSetEqCase {
            description: String::from("Equivalent AABB lists"),
            aabb_vec1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
            ],
            aabb_vec2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0)),
            ],
            expected_is_equal: true,
        },
        ApproxSetEqCase {
            description: String::from(
                "AABB lists that differ only by EPSILON in one center coordinate",
            ),
            aabb_vec1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(EPSILON, 0.0, 1.0), Vec3::splat(1.0)),
            ],
            aabb_vec2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0)),
            ],
            expected_is_equal: true,
        },
        ApproxSetEqCase {
            description: String::from(
                "AABB lists that differ only by 2.0 * EPSILON in one center coordinate",
            ),
            aabb_vec1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(2.0 * EPSILON, 0.0, 1.0), Vec3::splat(1.0)),
            ],
            aabb_vec2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0)),
            ],
            expected_is_equal: false,
        },
        ApproxSetEqCase {
            description: String::from(
                "AABB lists that differ only by 4.0 * EPSILON in one size coordinate",
            ),
            aabb_vec1: vec![
                AABB::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0 + 4.0 * EPSILON, 1.0, 1.0),
                ),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
            ],
            aabb_vec2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0)),
            ],
            expected_is_equal: false,
        },
        ApproxSetEqCase {
            description: String::from("AABB lists with different lengths"),
            aabb_vec1: vec![
                AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
            ],
            aabb_vec2: vec![
                AABB::new(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(1.0)),
                AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(1.0)),
            ],
            expected_is_equal: false,
        },
    ];

    for case in test_cases {
        case.check();
    }
}
