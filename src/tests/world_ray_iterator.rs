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
        let epsilon = 1e-3;
        let mut block_sample_vec = Vec::new();

        if let Some(iter) =
            WorldRayIter::from_ray(&world, case.origin, case.direction, case.distance)
        {
            for block_sample in iter {
                block_sample_vec.push(block_sample);
            }
        }

        println!("{:?}", case.description);

        assert_eq!(block_sample_vec.len(), case.expected_block_info_vec.len());

        for (index, (t, position, enter_face_direction)) in
            case.expected_block_info_vec.iter().enumerate()
        {
            assert!(
                (block_sample_vec[index].t - *t).abs() <= epsilon,
                "i={}, got t={}, want {}",
                index,
                block_sample_vec[index].t,
                t
            );

            assert_eq!(block_sample_vec[index].position, *position);

            assert_eq!(
                block_sample_vec[index].enter_face_direction,
                *enter_face_direction
            );
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
