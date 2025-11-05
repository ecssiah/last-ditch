use crate::simulation::{
    self,
    state::{
        world::grid::{self, WorldRayIterator},
        World,
    },
};
use glam::{IVec3, Vec3};

struct WorldRayIteratoratorCase {
    pub description: String,
    pub origin: Vec3,
    pub direction: Vec3,
    pub distance: f32,
    pub expected_cell_info_vec: Vec<(f32, IVec3, grid::Direction)>,
}

impl WorldRayIteratoratorCase {
    pub fn check(case: &WorldRayIteratoratorCase, world: &World) {
        let epsilon = 1e-3;
        let mut cell_sample_vec = Vec::new();

        if let Some(iter) =
            WorldRayIterator::from_ray(&world, case.origin, case.direction, case.distance)
        {
            for cell_sample in iter {
                cell_sample_vec.push(cell_sample);
            }
        }

        println!("{:?}", case.description);

        assert_eq!(cell_sample_vec.len(), case.expected_cell_info_vec.len());

        for (index, (t, position, enter_face_direction)) in
            case.expected_cell_info_vec.iter().enumerate()
        {
            assert!(
                (cell_sample_vec[index].t - *t).abs() <= epsilon,
                "i={}, got t={}, want {}",
                index,
                cell_sample_vec[index].t,
                t
            );

            assert_eq!(cell_sample_vec[index].position, *position);

            assert_eq!(
                cell_sample_vec[index].enter_face_direction,
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
        WorldRayIteratoratorCase {
            description: "case 1".to_string(),
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 1.0, 1.0),
            distance: 4.0,
            expected_cell_info_vec: vec![
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
        WorldRayIteratoratorCase {
            description: "case 2".to_string(),
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 1.0, 0.0),
            distance: 4.0,
            expected_cell_info_vec: vec![
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
        WorldRayIteratoratorCase {
            description: "case 3".to_string(),
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 1.0, 0.0),
            distance: 0.0,
            expected_cell_info_vec: vec![],
        },
    ];

    for case in &test_cases {
        WorldRayIteratoratorCase::check(case, &world);
    }
}
