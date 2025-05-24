use glam::Vec3;
use last_ditch::simulation::{
    consts::*,
    physics::aabb::AABB,
    world::{block, grid},
};
use std::f32::EPSILON;

struct OverlappingAABBTestCase {
    description: String,
    aabb: AABB,
    expected_aabb_list: Vec<AABB>,
}

#[test]
fn xo_yo_zo() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XoYoZo.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![block::Block::get_aabb(0, 0, 0)],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn xp_yp_zp() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XpYpZp.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![
            block::Block::get_aabb(0, 0, 0),
            block::Block::get_aabb(1, 0, 0),
            block::Block::get_aabb(0, 1, 0),
            block::Block::get_aabb(1, 1, 0),
            block::Block::get_aabb(0, 0, 1),
            block::Block::get_aabb(1, 0, 1),
            block::Block::get_aabb(0, 1, 1),
            block::Block::get_aabb(1, 1, 1),
        ],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "{:?}", test_case.description);
}

#[test]
fn xp_yp_zn() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XpYpZn.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![
            block::Block::get_aabb(0, 0, -1),
            block::Block::get_aabb(1, 0, -1),
            block::Block::get_aabb(0, 1, -1),
            block::Block::get_aabb(1, 1, -1),
            block::Block::get_aabb(0, 0, 0),
            block::Block::get_aabb(1, 0, 0),
            block::Block::get_aabb(0, 1, 0),
            block::Block::get_aabb(1, 1, 0),
        ],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "{:?}", test_case.description);
}

#[test]
fn xp_yn_zp() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XpYnZp.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![
            block::Block::get_aabb(0, -1, 0),
            block::Block::get_aabb(1, -1, 0),
            block::Block::get_aabb(0, 0, 0),
            block::Block::get_aabb(1, 0, 0),
            block::Block::get_aabb(0, -1, 1),
            block::Block::get_aabb(1, -1, 1),
            block::Block::get_aabb(0, 0, 1),
            block::Block::get_aabb(1, 0, 1),
        ],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "{:?}", test_case.description);
}

#[test]
fn xp_yn_zn() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XpYnZn.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![
            block::Block::get_aabb(0, -1, -1),
            block::Block::get_aabb(1, -1, -1),
            block::Block::get_aabb(0, 0, -1),
            block::Block::get_aabb(1, 0, -1),
            block::Block::get_aabb(0, -1, 0),
            block::Block::get_aabb(1, -1, 0),
            block::Block::get_aabb(0, 0, 0),
            block::Block::get_aabb(1, 0, 0),
        ],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "{:?}", test_case.description);
}

#[test]
fn xn_yp_zp() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XnYpZp.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![
            block::Block::get_aabb(-1, 0, 0),
            block::Block::get_aabb(0, 0, 0),
            block::Block::get_aabb(-1, 1, 0),
            block::Block::get_aabb(0, 1, 0),
            block::Block::get_aabb(-1, 0, 1),
            block::Block::get_aabb(0, 0, 1),
            block::Block::get_aabb(-1, 1, 1),
            block::Block::get_aabb(0, 1, 1),
        ],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "{:?}", test_case.description);
}

#[test]
fn xn_yp_zn() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XnYpZn.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![
            block::Block::get_aabb(-1, 0, -1),
            block::Block::get_aabb(0, 0, -1),
            block::Block::get_aabb(-1, 1, -1),
            block::Block::get_aabb(0, 1, -1),
            block::Block::get_aabb(-1, 0, 0),
            block::Block::get_aabb(0, 0, 0),
            block::Block::get_aabb(-1, 1, 0),
            block::Block::get_aabb(0, 1, 0),
        ],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "{:?}", test_case.description);
}

#[test]
fn xn_yn_zp() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XnYnZp.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![
            block::Block::get_aabb(-1, -1, 0),
            block::Block::get_aabb(0, -1, 0),
            block::Block::get_aabb(-1, 0, 0),
            block::Block::get_aabb(0, 0, 0),
            block::Block::get_aabb(-1, -1, 1),
            block::Block::get_aabb(0, -1, 1),
            block::Block::get_aabb(-1, 0, 1),
            block::Block::get_aabb(0, 0, 1),
        ],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "{:?}", test_case.description);
}

#[test]
fn xn_yn_zn() {
    let test_case = OverlappingAABBTestCase {
        description: String::from(""),
        aabb: AABB::new(
            grid::Direction::XnYnZn.offset().as_vec3() * 0.5,
            Vec3::splat(BLOCK_SIZE),
        ),
        expected_aabb_list: vec![
            block::Block::get_aabb(0, 0, 0),
            block::Block::get_aabb(-1, -1, -1),
            block::Block::get_aabb(0, -1, -1),
            block::Block::get_aabb(-1, 0, -1),
            block::Block::get_aabb(0, 0, -1),
            block::Block::get_aabb(-1, -1, 0),
            block::Block::get_aabb(0, -1, 0),
            block::Block::get_aabb(-1, 0, 0),
        ],
    };

    let aabb_list = grid::overlapping_aabb_list(&test_case.aabb);

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_case.expected_aabb_list, EPSILON);

    assert!(is_equal, "{:?}", test_case.description);
}
