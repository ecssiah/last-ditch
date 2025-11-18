use crate::simulation::{
    self, constructor,
    state::{
        physics::aabb::AABB,
        world::{
            cell::Cell,
            grid::{self, Grid},
            World,
        },
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
        let aabb_vec = Grid::cells_overlapping(&world.grid, self.aabb);

        let is_equal = AABB::approx_set_eq(&aabb_vec, &self.expected_aabb_vec, EPSILON);

        assert!(is_equal, "{:?}", self.description);
    }
}

#[test]
fn directions() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::run(simulation_kind, &mut world);

    let test_cases = vec![
        OverlappingAABBCase {
            description: String::from("XOYOZO"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XOYOZO.offset()) * 0.5,
                Vec3::broadcast(world.grid.cell_size_in_meters),
            ),
            expected_aabb_vec: vec![Cell::aabb(0, 0, 0, &world.grid)],
        },
        OverlappingAABBCase {
            description: String::from("XPYPZP"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XPYPZP.offset()) * 0.5,
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
            description: String::from("XPYPZN"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XPYPZN.offset()) * 0.5,
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
            description: String::from("XPYNZP"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XPYNZP.offset()) * 0.5,
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
            description: String::from("XPYNZN"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XPYNZN.offset()) * 0.5,
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
            description: String::from("XPYNZN"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XPYNZN.offset()) * 0.5,
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
            description: String::from("XNYPZP"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XNYPZP.offset()) * 0.5,
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
            description: String::from("XNYPZN"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XNYPZN.offset()) * 0.5,
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
            description: String::from("XNYNZP"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XNYNZP.offset()) * 0.5,
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
            description: String::from("XNYNZN"),
            aabb: AABB::new(
                Vec3::from(grid::Direction::XNYNZN.offset()) * 0.5,
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
