use crate::simulation::{
    self,
    state::{
        world::grid::{self, WorldRayIter},
        World,
    },
};
use glam::{IVec3, Vec3};

struct WorldRayIteratorCase {
    pub description: String,
    pub origin: Vec3,
    pub direction: Vec3,
    pub distance: f32,
    pub expected_block_info_vec: Vec<(f32, IVec3, grid::Direction)>,
}

impl WorldRayIteratorCase {
    pub fn check(case: &WorldRayIteratorCase, world: &World) {
        let eps = 1e-3;
        let mut got = Vec::new();

        if let Some(iter) =
            WorldRayIter::from_ray(&world, case.origin, case.direction, case.distance)
        {
            for s in iter {
                got.push(s);
            }
        }

        assert_eq!(got.len(), case.expected_block_info_vec.len());

        for (i, (t, position, enter_face_direction)) in
            case.expected_block_info_vec.iter().enumerate()
        {
            assert!(
                (got[i].t - *t).abs() <= eps,
                "i={i}, got t={}, want {t}",
                got[i].t
            );

            assert_eq!(got[i].position, *position);
            assert_eq!(got[i].enter_face_direction, *enter_face_direction);
        }
    }
}

#[test]
fn general_ray_tests() {
    let kind = simulation::Kind::WorldTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        WorldRayIteratorCase {
            description: "case 1".to_string(),
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 1.0, 1.0),
            distance: 4.0,
            expected_block_info_vec: vec![
                (0.5, IVec3::new(1, 0, 0), grid::Direction::XnYoZo),
                (0.5, IVec3::new(1, 1, 0), grid::Direction::XoYnZo),
                (0.5, IVec3::new(1, 1, 1), grid::Direction::XoYoZn),
                (1.5, IVec3::new(2, 1, 1), grid::Direction::XnYoZo),
                (1.5, IVec3::new(2, 2, 1), grid::Direction::XoYnZo),
                (1.5, IVec3::new(2, 2, 2), grid::Direction::XoYoZn),
                (2.5, IVec3::new(3, 2, 2), grid::Direction::XnYoZo),
                (2.5, IVec3::new(3, 3, 2), grid::Direction::XoYnZo),
                (2.5, IVec3::new(3, 3, 3), grid::Direction::XoYoZn),
                (3.5, IVec3::new(4, 3, 3), grid::Direction::XnYoZo),
                (3.5, IVec3::new(4, 4, 3), grid::Direction::XoYnZo),
                (3.5, IVec3::new(4, 4, 4), grid::Direction::XoYoZn),
            ],
        },
        WorldRayIteratorCase {
            description: "case 2".to_string(),
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 1.0, 0.0),
            distance: 4.0,
            expected_block_info_vec: vec![
                (0.5, IVec3::new(1, 0, 0), grid::Direction::XnYoZo),
                (0.5, IVec3::new(1, 1, 0), grid::Direction::XoYnZo),
                (1.5, IVec3::new(2, 1, 0), grid::Direction::XnYoZo),
                (1.5, IVec3::new(2, 2, 0), grid::Direction::XoYnZo),
                (2.5, IVec3::new(3, 2, 0), grid::Direction::XnYoZo),
                (2.5, IVec3::new(3, 3, 0), grid::Direction::XoYnZo),
                (3.5, IVec3::new(4, 3, 0), grid::Direction::XnYoZo),
                (3.5, IVec3::new(4, 4, 0), grid::Direction::XoYnZo),
            ],
        },
        WorldRayIteratorCase {
            description: "case 3".to_string(),
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 1.0, 0.0),
            distance: 0.0,
            expected_block_info_vec: vec![],
        },
    ];

    for case in &test_cases {
        WorldRayIteratorCase::check(case, &world);
    }
}
