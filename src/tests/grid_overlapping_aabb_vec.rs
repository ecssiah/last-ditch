use crate::simulation::{constants::CELL_SIZE_IN_METERS, state::{
        self, constructor, physics::aabb::AABB, world::{World, cell::Cell, grid}
    }};
use std::f32::EPSILON;
use ultraviolet::Vec3;

struct OverlappingAABBCase {
    description: String,
    aabb: AABB,
    expected_aabb_vec: Vec<AABB>,
}

impl OverlappingAABBCase {
    pub fn check(&self) {
        let aabb_vec = grid::cells_overlapping(self.aabb);

        let is_equal = AABB::approx_set_eq(&aabb_vec, &self.expected_aabb_vec, EPSILON);

        assert!(is_equal, "{:?}", self.description);
    }
}

#[test]
fn directions() {
    let state_template = state::Template::Empty;

    let mut world = World::new(state_template, 0);
    constructor::world_template::construct(state_template, &mut world);

    let test_cases = vec![
        OverlappingAABBCase {
            description: String::from("(0.5, 0.5, 0.5)"),
            aabb: AABB::new(
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::broadcast(CELL_SIZE_IN_METERS),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, 0, 0),
                Cell::aabb(1, 0, 0),
                Cell::aabb(0, 1, 0),
                Cell::aabb(1, 1, 0),
                Cell::aabb(0, 0, 1),
                Cell::aabb(1, 0, 1),
                Cell::aabb(0, 1, 1),
                Cell::aabb(1, 1, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(0.5, 0.5, -0.5)"),
            aabb: AABB::new(
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::broadcast(CELL_SIZE_IN_METERS),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, 0, -1),
                Cell::aabb(1, 0, -1),
                Cell::aabb(0, 1, -1),
                Cell::aabb(1, 1, -1),
                Cell::aabb(0, 0, 0),
                Cell::aabb(1, 0, 0),
                Cell::aabb(0, 1, 0),
                Cell::aabb(1, 1, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(0.5, -0.5, 0.5)"),
            aabb: AABB::new(
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::broadcast(CELL_SIZE_IN_METERS),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, -1, 0),
                Cell::aabb(1, -1, 0),
                Cell::aabb(0, 0, 0),
                Cell::aabb(1, 0, 0),
                Cell::aabb(0, -1, 1),
                Cell::aabb(1, -1, 1),
                Cell::aabb(0, 0, 1),
                Cell::aabb(1, 0, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(0.5, -0.5, -0.5)"),
            aabb: AABB::new(
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::broadcast(CELL_SIZE_IN_METERS),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, -1, -1),
                Cell::aabb(1, -1, -1),
                Cell::aabb(0, 0, -1),
                Cell::aabb(1, 0, -1),
                Cell::aabb(0, -1, 0),
                Cell::aabb(1, -1, 0),
                Cell::aabb(0, 0, 0),
                Cell::aabb(1, 0, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(-0.5, 0.5, 0.5)"),
            aabb: AABB::new(
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::broadcast(CELL_SIZE_IN_METERS),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(-1, 0, 0),
                Cell::aabb(0, 0, 0),
                Cell::aabb(-1, 1, 0),
                Cell::aabb(0, 1, 0),
                Cell::aabb(-1, 0, 1),
                Cell::aabb(0, 0, 1),
                Cell::aabb(-1, 1, 1),
                Cell::aabb(0, 1, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(-0.5, 0.5, -0.5)"),
            aabb: AABB::new(
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::broadcast(CELL_SIZE_IN_METERS),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(-1, 0, -1),
                Cell::aabb(0, 0, -1),
                Cell::aabb(-1, 1, -1),
                Cell::aabb(0, 1, -1),
                Cell::aabb(-1, 0, 0),
                Cell::aabb(0, 0, 0),
                Cell::aabb(-1, 1, 0),
                Cell::aabb(0, 1, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(-0.5, -0.5, 0.5)"),
            aabb: AABB::new(
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::broadcast(CELL_SIZE_IN_METERS),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(-1, -1, 0),
                Cell::aabb(0, -1, 0),
                Cell::aabb(-1, 0, 0),
                Cell::aabb(0, 0, 0),
                Cell::aabb(-1, -1, 1),
                Cell::aabb(0, -1, 1),
                Cell::aabb(-1, 0, 1),
                Cell::aabb(0, 0, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(-0.5, -0.5, -0.5)"),
            aabb: AABB::new(
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::broadcast(CELL_SIZE_IN_METERS),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, 0, 0),
                Cell::aabb(-1, -1, -1),
                Cell::aabb(0, -1, -1),
                Cell::aabb(-1, 0, -1),
                Cell::aabb(0, 0, -1),
                Cell::aabb(-1, -1, 0),
                Cell::aabb(0, -1, 0),
                Cell::aabb(-1, 0, 0),
            ],
        },
    ];

    for case in test_cases {
        case.check();
    }
}
