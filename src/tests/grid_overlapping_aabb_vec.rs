use crate::simulation::{
    self,
    consts::*,
    state::{
        physics::aabb::AABB,
        world::{
            block::Block,
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
    pub fn check(&self) {
        let aabb_vec = Grid::blocks_overlapping(self.aabb);

        let is_equal = AABB::approx_set_eq(&aabb_vec, &self.expected_aabb_vec, EPSILON);

        assert!(is_equal, "{:?}", self.description);
    }
}

#[test]
fn directions() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        OverlappingAABBCase {
            description: String::from("XoYoZo"),
            aabb: AABB::new(
                grid::Direction::XoYoZo.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![Block::aabb(0, 0, 0)],
        },
        OverlappingAABBCase {
            description: String::from("XpYpZp"),
            aabb: AABB::new(
                grid::Direction::XpYpZp.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(0, 0, 0),
                Block::aabb(1, 0, 0),
                Block::aabb(0, 1, 0),
                Block::aabb(1, 1, 0),
                Block::aabb(0, 0, 1),
                Block::aabb(1, 0, 1),
                Block::aabb(0, 1, 1),
                Block::aabb(1, 1, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XpYpZn"),
            aabb: AABB::new(
                grid::Direction::XpYpZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(0, 0, -1),
                Block::aabb(1, 0, -1),
                Block::aabb(0, 1, -1),
                Block::aabb(1, 1, -1),
                Block::aabb(0, 0, 0),
                Block::aabb(1, 0, 0),
                Block::aabb(0, 1, 0),
                Block::aabb(1, 1, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XpYnZp"),
            aabb: AABB::new(
                grid::Direction::XpYnZp.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(0, -1, 0),
                Block::aabb(1, -1, 0),
                Block::aabb(0, 0, 0),
                Block::aabb(1, 0, 0),
                Block::aabb(0, -1, 1),
                Block::aabb(1, -1, 1),
                Block::aabb(0, 0, 1),
                Block::aabb(1, 0, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XpYnZn"),
            aabb: AABB::new(
                grid::Direction::XpYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(0, -1, -1),
                Block::aabb(1, -1, -1),
                Block::aabb(0, 0, -1),
                Block::aabb(1, 0, -1),
                Block::aabb(0, -1, 0),
                Block::aabb(1, -1, 0),
                Block::aabb(0, 0, 0),
                Block::aabb(1, 0, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XpYnZn"),
            aabb: AABB::new(
                grid::Direction::XpYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(0, -1, -1),
                Block::aabb(1, -1, -1),
                Block::aabb(0, 0, -1),
                Block::aabb(1, 0, -1),
                Block::aabb(0, -1, 0),
                Block::aabb(1, -1, 0),
                Block::aabb(0, 0, 0),
                Block::aabb(1, 0, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XnYpZp"),
            aabb: AABB::new(
                grid::Direction::XnYpZp.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(-1, 0, 0),
                Block::aabb(0, 0, 0),
                Block::aabb(-1, 1, 0),
                Block::aabb(0, 1, 0),
                Block::aabb(-1, 0, 1),
                Block::aabb(0, 0, 1),
                Block::aabb(-1, 1, 1),
                Block::aabb(0, 1, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XnYpZn"),
            aabb: AABB::new(
                grid::Direction::XnYpZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(-1, 0, -1),
                Block::aabb(0, 0, -1),
                Block::aabb(-1, 1, -1),
                Block::aabb(0, 1, -1),
                Block::aabb(-1, 0, 0),
                Block::aabb(0, 0, 0),
                Block::aabb(-1, 1, 0),
                Block::aabb(0, 1, 0),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XnYnZp"),
            aabb: AABB::new(
                grid::Direction::XnYnZp.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(-1, -1, 0),
                Block::aabb(0, -1, 0),
                Block::aabb(-1, 0, 0),
                Block::aabb(0, 0, 0),
                Block::aabb(-1, -1, 1),
                Block::aabb(0, -1, 1),
                Block::aabb(-1, 0, 1),
                Block::aabb(0, 0, 1),
            ],
        },
        OverlappingAABBCase {
            description: String::from("XnYnZn"),
            aabb: AABB::new(
                grid::Direction::XnYnZn.offset().as_vec3() * 0.5,
                Vec3::splat(BLOCK_SIZE),
            ),
            expected_aabb_vec: vec![
                Block::aabb(0, 0, 0),
                Block::aabb(-1, -1, -1),
                Block::aabb(0, -1, -1),
                Block::aabb(-1, 0, -1),
                Block::aabb(0, 0, -1),
                Block::aabb(-1, -1, 0),
                Block::aabb(0, -1, 0),
                Block::aabb(-1, 0, 0),
            ],
        },
    ];

    for case in test_cases {
        case.check();
    }
}
