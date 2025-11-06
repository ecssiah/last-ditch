use crate::simulation::{
    self,
    state::{
        physics::aabb::AABB,
        world::{
            cell::Cell,
            grid::{self, Grid},
            World,
        },
    },
};
use glam::Vec3;
use std::f32::EPSILON;

struct OverlappingAABBCase {
    description: String,
    aabb: AABB,
    expected_aabb_vec: Vec<AABB>,
}

impl OverlappingAABBCase {
    pub fn check(&self, world: &World) {
        let aabb_vec = Grid::cells_overlapping(&world.grid, self.aabb);

        let is_equal = AABB::approx_set_eq(&aabb_vec, &self.expected_aabb_vec, EPSILON);

        assert!(is_equal, "{:?}", self.description);
    }
}

#[test]
fn directions() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    World::setup(simulation_kind, &mut world);

    let test_cases = vec![
        OverlappingAABBCase {
            description: String::from("XoYoZo"),
            aabb: AABB::new(
                grid::Direction::XoYoZo.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![Cell::aabb(0, 0, 0, &world.grid)],
        },
        OverlappingAABBCase {
            description: String::from("XpYpZp"),
            aabb: AABB::new(
                grid::Direction::XpYpZp.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
            description: String::from("XpYpZn"),
            aabb: AABB::new(
                grid::Direction::XpYpZn.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
            description: String::from("XpYnZp"),
            aabb: AABB::new(
                grid::Direction::XpYnZp.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
            description: String::from("XpYnZn"),
            aabb: AABB::new(
                grid::Direction::XpYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
            description: String::from("XpYnZn"),
            aabb: AABB::new(
                grid::Direction::XpYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
            description: String::from("XnYpZp"),
            aabb: AABB::new(
                grid::Direction::XnYpZp.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
            description: String::from("XnYpZn"),
            aabb: AABB::new(
                grid::Direction::XnYpZn.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
            description: String::from("XnYnZp"),
            aabb: AABB::new(
                grid::Direction::XnYnZp.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
            description: String::from("XnYnZn"),
            aabb: AABB::new(
                grid::Direction::XnYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(world.grid.cell_size_in_meters),
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
