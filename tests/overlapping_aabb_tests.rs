use glam::Vec3;
use last_ditch::simulation::{
    consts::*,
    physics::aabb::AABB,
    world::{block, grid},
};
use std::f32::EPSILON;

#[test]
fn at_xo_yo_zo() {
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![block::Block::get_aabb(0, 0, 0)];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn at_xp_yp_zp() {
    let aabb = AABB::new(Vec3::new(0.5, 0.5, 0.5), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![
        block::Block::get_aabb(0, 0, 0),
        block::Block::get_aabb(1, 0, 0),
        block::Block::get_aabb(0, 1, 0),
        block::Block::get_aabb(1, 1, 0),
        block::Block::get_aabb(0, 0, 1),
        block::Block::get_aabb(1, 0, 1),
        block::Block::get_aabb(0, 1, 1),
        block::Block::get_aabb(1, 1, 1),
    ];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn at_xp_yp_zn() {
    let aabb = AABB::new(Vec3::new(0.5, 0.5, -0.5), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![
        block::Block::get_aabb(0, 0, -1),
        block::Block::get_aabb(1, 0, -1),
        block::Block::get_aabb(0, 1, -1),
        block::Block::get_aabb(1, 1, -1),
        block::Block::get_aabb(0, 0, 0),
        block::Block::get_aabb(1, 0, 0),
        block::Block::get_aabb(0, 1, 0),
        block::Block::get_aabb(1, 1, 0),
    ];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn at_xp_yn_zp() {
    let aabb = AABB::new(Vec3::new(0.5, -0.5, 0.5), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![
        block::Block::get_aabb(0, -1, 0),
        block::Block::get_aabb(1, -1, 0),
        block::Block::get_aabb(0, 0, 0),
        block::Block::get_aabb(1, 0, 0),
        block::Block::get_aabb(0, -1, 1),
        block::Block::get_aabb(1, -1, 1),
        block::Block::get_aabb(0, 0, 1),
        block::Block::get_aabb(1, 0, 1),
    ];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn at_xp_yn_zn() {
    let aabb = AABB::new(Vec3::new(0.5, -0.5, -0.5), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![
        block::Block::get_aabb(0, -1, -1),
        block::Block::get_aabb(1, -1, -1),
        block::Block::get_aabb(0, 0, -1),
        block::Block::get_aabb(1, 0, -1),
        block::Block::get_aabb(0, -1, 0),
        block::Block::get_aabb(1, -1, 0),
        block::Block::get_aabb(0, 0, 0),
        block::Block::get_aabb(1, 0, 0),
    ];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn at_xn_yp_zp() {
    let aabb = AABB::new(Vec3::new(-0.5, 0.5, 0.5), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![
        block::Block::get_aabb(-1, 0, 0),
        block::Block::get_aabb(0, 0, 0),
        block::Block::get_aabb(-1, 1, 0),
        block::Block::get_aabb(0, 1, 0),
        block::Block::get_aabb(-1, 0, 1),
        block::Block::get_aabb(0, 0, 1),
        block::Block::get_aabb(-1, 1, 1),
        block::Block::get_aabb(0, 1, 1),
    ];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn at_xn_yp_zn() {
    let aabb = AABB::new(Vec3::new(-0.5, 0.5, -0.5), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![
        block::Block::get_aabb(-1, 0, -1),
        block::Block::get_aabb(0, 0, -1),
        block::Block::get_aabb(-1, 1, -1),
        block::Block::get_aabb(0, 1, -1),
        block::Block::get_aabb(-1, 0, 0),
        block::Block::get_aabb(0, 0, 0),
        block::Block::get_aabb(-1, 1, 0),
        block::Block::get_aabb(0, 1, 0),
    ];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn at_xn_yn_zp() {
    let aabb = AABB::new(Vec3::new(-0.5, -0.5, 0.5), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![
        block::Block::get_aabb(-1, -1, 0),
        block::Block::get_aabb(0, -1, 0),
        block::Block::get_aabb(-1, 0, 0),
        block::Block::get_aabb(0, 0, 0),
        block::Block::get_aabb(-1, -1, 1),
        block::Block::get_aabb(0, -1, 1),
        block::Block::get_aabb(-1, 0, 1),
        block::Block::get_aabb(0, 0, 1),
    ];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}

#[test]
fn at_xn_yn_zn() {
    let aabb = AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::splat(BLOCK_SIZE));
    let aabb_list = grid::get_overlapping_aabb_list(&aabb);

    let test_aabb_list = vec![
        block::Block::get_aabb(0, 0, 0),
        block::Block::get_aabb(-1, -1, -1),
        block::Block::get_aabb(0, -1, -1),
        block::Block::get_aabb(-1, 0, -1),
        block::Block::get_aabb(0, 0, -1),
        block::Block::get_aabb(-1, -1, 0),
        block::Block::get_aabb(0, -1, 0),
        block::Block::get_aabb(-1, 0, 0),
    ];

    let is_equal = AABB::approx_set_eq(&aabb_list, &test_aabb_list, EPSILON);

    assert!(is_equal, "AABB list does not match control");
}
