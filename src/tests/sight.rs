use crate::simulation::state::population::entity::Sight;
use approx::assert_abs_diff_eq;
use glam::Vec3;

struct SightCase {
    pub description: String,
    pub fov_angle: f32,
    pub ray_count: usize,
    pub expected_view_ray_vec: Vec<Vec3>,
}

impl SightCase {
    pub fn check(case: &SightCase) {
        let view_ray_vec_reference =
            Sight::generate_view_ray_vec_reference(case.fov_angle, case.ray_count);

        for (&ray, &expected_ray) in view_ray_vec_reference
            .iter()
            .zip(case.expected_view_ray_vec.iter())
        {
            let dot_product = ray.normalize().dot(expected_ray.normalize());

            assert_abs_diff_eq!(dot_product, 1.0, epsilon = 1e-3);
        }
    }
}

#[test]
fn fibonacci_cone_direction_tests() {
    let test_cases = vec![SightCase {
        description: "case 1".to_string(),
        fov_angle: 90.0,
        ray_count: 4,
        expected_view_ray_vec: vec![
            Vec3::new(0.669, 0.000, 0.744),
            Vec3::new(-0.428, -0.396, 0.817),
            Vec3::new(0.039, 0.454, 0.890),
            Vec3::new(0.208, -0.185, 0.963),
        ],
    }];

    for case in &test_cases {
        SightCase::check(case);
    }
}
