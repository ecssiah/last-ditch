use crate::simulation::{
    constructor,
    state::{
        self,
        physics::aabb::AABB,
        world::{cell::Cell, grid::Grid, World},
    },
};
use std::f32::EPSILON;
use ultraviolet::Vec3;

struct OverlappingAABBCase {
    description: String,
    aabb: AABB,
    expected_aabb_vec: Vec<AABB>,
}

impl OverlappingAABBCase {
    pub fn check(&self, world: &World) {
        let aabb_vec = Grid::cells_overlapping(self.aabb, &world.grid);

        let is_equal = AABB::approx_set_eq(&aabb_vec, &self.expected_aabb_vec, EPSILON);

        assert!(is_equal, "{:?}", self.description);
    }
}

#[test]
fn directions() {
    let state_template = state::Template::Empty;

    let mut world = World::new(state_template);
    constructor::world_template::construct(state_template, &mut world);

    let test_cases = vec![
        OverlappingAABBCase {
            description: String::from("(0.5, 0.5, 0.5)"),
            aabb: AABB::new(
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, 0, 0, &world.grid),
                Cell::aabb(1, 0, 0, &world.grid),
                Cell::aabb(0, 1, 0, &world.grid),
                Cell::aabb(1, 1, 0, &world.grid),
                Cell::aabb(0, 0, 1, &world.grid),
                Cell::aabb(1, 0, 1, &world.grid),
                Cell::aabb(0, 1, 1, &world.grid),
                Cell::aabb(1, 1, 1, &world.grid),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(0.5, 0.5, -0.5)"),
            aabb: AABB::new(
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, 0, -1, &world.grid),
                Cell::aabb(1, 0, -1, &world.grid),
                Cell::aabb(0, 1, -1, &world.grid),
                Cell::aabb(1, 1, -1, &world.grid),
                Cell::aabb(0, 0, 0, &world.grid),
                Cell::aabb(1, 0, 0, &world.grid),
                Cell::aabb(0, 1, 0, &world.grid),
                Cell::aabb(1, 1, 0, &world.grid),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(0.5, -0.5, 0.5)"),
            aabb: AABB::new(
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, -1, 0, &world.grid),
                Cell::aabb(1, -1, 0, &world.grid),
                Cell::aabb(0, 0, 0, &world.grid),
                Cell::aabb(1, 0, 0, &world.grid),
                Cell::aabb(0, -1, 1, &world.grid),
                Cell::aabb(1, -1, 1, &world.grid),
                Cell::aabb(0, 0, 1, &world.grid),
                Cell::aabb(1, 0, 1, &world.grid),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(0.5, -0.5, -0.5)"),
            aabb: AABB::new(
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, -1, -1, &world.grid),
                Cell::aabb(1, -1, -1, &world.grid),
                Cell::aabb(0, 0, -1, &world.grid),
                Cell::aabb(1, 0, -1, &world.grid),
                Cell::aabb(0, -1, 0, &world.grid),
                Cell::aabb(1, -1, 0, &world.grid),
                Cell::aabb(0, 0, 0, &world.grid),
                Cell::aabb(1, 0, 0, &world.grid),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(-0.5, 0.5, 0.5)"),
            aabb: AABB::new(
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(-1, 0, 0, &world.grid),
                Cell::aabb(0, 0, 0, &world.grid),
                Cell::aabb(-1, 1, 0, &world.grid),
                Cell::aabb(0, 1, 0, &world.grid),
                Cell::aabb(-1, 0, 1, &world.grid),
                Cell::aabb(0, 0, 1, &world.grid),
                Cell::aabb(-1, 1, 1, &world.grid),
                Cell::aabb(0, 1, 1, &world.grid),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(-0.5, 0.5, -0.5)"),
            aabb: AABB::new(
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(-1, 0, -1, &world.grid),
                Cell::aabb(0, 0, -1, &world.grid),
                Cell::aabb(-1, 1, -1, &world.grid),
                Cell::aabb(0, 1, -1, &world.grid),
                Cell::aabb(-1, 0, 0, &world.grid),
                Cell::aabb(0, 0, 0, &world.grid),
                Cell::aabb(-1, 1, 0, &world.grid),
                Cell::aabb(0, 1, 0, &world.grid),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(-0.5, -0.5, 0.5)"),
            aabb: AABB::new(
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(-1, -1, 0, &world.grid),
                Cell::aabb(0, -1, 0, &world.grid),
                Cell::aabb(-1, 0, 0, &world.grid),
                Cell::aabb(0, 0, 0, &world.grid),
                Cell::aabb(-1, -1, 1, &world.grid),
                Cell::aabb(0, -1, 1, &world.grid),
                Cell::aabb(-1, 0, 1, &world.grid),
                Cell::aabb(0, 0, 1, &world.grid),
            ],
        },
        OverlappingAABBCase {
            description: String::from("(-0.5, -0.5, -0.5)"),
            aabb: AABB::new(
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![
                Cell::aabb(0, 0, 0, &world.grid),
                Cell::aabb(-1, -1, -1, &world.grid),
                Cell::aabb(0, -1, -1, &world.grid),
                Cell::aabb(-1, 0, -1, &world.grid),
                Cell::aabb(0, 0, -1, &world.grid),
                Cell::aabb(-1, -1, 0, &world.grid),
                Cell::aabb(0, -1, 0, &world.grid),
                Cell::aabb(-1, 0, 0, &world.grid),
            ],
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
