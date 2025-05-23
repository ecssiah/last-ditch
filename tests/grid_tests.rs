use glam::Vec3;
use last_ditch::simulation::{
    physics::aabb::AABB,
    world::{block::Block, grid},
    BLOCK_SIZE,
};
use std::f32::EPSILON;

#[test]
fn overlapping_aabb_at_xo_yo_zo() {
    let aabb_test = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);
    let aabb_list_control = vec![Block::get_aabb(0, 0, 0)];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xp_yp_zp() {
    let aabb_test = AABB::new(Vec3::new(0.5, 0.5, 0.5), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        Block::get_aabb(0, 0, 0),
        Block::get_aabb(1, 0, 0),
        Block::get_aabb(0, 1, 0),
        Block::get_aabb(1, 1, 0),
        Block::get_aabb(0, 0, 1),
        Block::get_aabb(1, 0, 1),
        Block::get_aabb(0, 1, 1),
        Block::get_aabb(1, 1, 1),
    ];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xp_yp_zn() {
    let aabb_test = AABB::new(Vec3::new(0.5, 0.5, -0.5), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        Block::get_aabb(0, 0, -1),
        Block::get_aabb(1, 0, -1),
        Block::get_aabb(0, 1, -1),
        Block::get_aabb(1, 1, -1),
        Block::get_aabb(0, 0, 0),
        Block::get_aabb(1, 0, 0),
        Block::get_aabb(0, 1, 0),
        Block::get_aabb(1, 1, 0),
    ];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xp_yn_zp() {
    let aabb_test = AABB::new(Vec3::new(0.5, -0.5, 0.5), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        Block::get_aabb(0, -1, 0),
        Block::get_aabb(1, -1, 0),
        Block::get_aabb(0, 0, 0),
        Block::get_aabb(1, 0, 0),
        Block::get_aabb(0, -1, 1),
        Block::get_aabb(1, -1, 1),
        Block::get_aabb(0, 0, 1),
        Block::get_aabb(1, 0, 1),
    ];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xp_yn_zn() {
    let aabb_test = AABB::new(Vec3::new(0.5, -0.5, -0.5), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        Block::get_aabb(0, -1, -1),
        Block::get_aabb(1, -1, -1),
        Block::get_aabb(0, 0, -1),
        Block::get_aabb(1, 0, -1),
        Block::get_aabb(0, -1, 0),
        Block::get_aabb(1, -1, 0),
        Block::get_aabb(0, 0, 0),
        Block::get_aabb(1, 0, 0),
    ];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xn_yp_zp() {
    let aabb_test = AABB::new(Vec3::new(-0.5, 0.5, 0.5), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        Block::get_aabb(-1, 0, 0),
        Block::get_aabb(0, 0, 0),
        Block::get_aabb(-1, 1, 0),
        Block::get_aabb(0, 1, 0),
        Block::get_aabb(-1, 0, 1),
        Block::get_aabb(0, 0, 1),
        Block::get_aabb(-1, 1, 1),
        Block::get_aabb(0, 1, 1),
    ];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xn_yp_zn() {
    let aabb_test = AABB::new(Vec3::new(-0.5, 0.5, -0.5), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        Block::get_aabb(-1, 0, -1),
        Block::get_aabb(0, 0, -1),
        Block::get_aabb(-1, 1, -1),
        Block::get_aabb(0, 1, -1),
        Block::get_aabb(-1, 0, 0),
        Block::get_aabb(0, 0, 0),
        Block::get_aabb(-1, 1, 0),
        Block::get_aabb(0, 1, 0),
    ];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xn_yn_zp() {
    let aabb_test = AABB::new(Vec3::new(-0.5, -0.5, 0.5), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        Block::get_aabb(-1, -1, 0),
        Block::get_aabb(0, -1, 0),
        Block::get_aabb(-1, 0, 0),
        Block::get_aabb(0, 0, 0),
        Block::get_aabb(-1, -1, 1),
        Block::get_aabb(0, -1, 1),
        Block::get_aabb(-1, 0, 1),
        Block::get_aabb(0, 0, 1),
    ];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xn_yn_zn() {
    let aabb_test = AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::splat(BLOCK_SIZE));

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        Block::get_aabb(0, 0, 0),
        Block::get_aabb(-1, -1, -1),
        Block::get_aabb(0, -1, -1),
        Block::get_aabb(-1, 0, -1),
        Block::get_aabb(0, 0, -1),
        Block::get_aabb(-1, -1, 0),
        Block::get_aabb(0, -1, 0),
        Block::get_aabb(-1, 0, 0),
    ];

    let list_matches = AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON);

    assert!(list_matches, "AABB list does not match control");
}
