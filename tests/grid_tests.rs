use glam::Vec3;
use last_ditch::simulation::{physics::aabb::AABB, world::grid, BLOCK_SIZE, EPSILON_COLLISION};

#[test]
fn overlapping_aabb_at_xo_yo_zo() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);
    let aabb_list_control = Vec::from([AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size)]);

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xp_yp_zp() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(0.5, 0.5, 0.5), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(1.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 1.0, 0.0), block_size),
        AABB::new(Vec3::new(1.0, 1.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 1.0), block_size),
        AABB::new(Vec3::new(1.0, 0.0, 1.0), block_size),
        AABB::new(Vec3::new(0.0, 1.0, 1.0), block_size),
        AABB::new(Vec3::new(1.0, 1.0, 1.0), block_size),
    ];

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xp_yp_zn() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(0.5, 0.5, -0.5), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        AABB::new(Vec3::new(0.0, 0.0, -1.0), block_size),
        AABB::new(Vec3::new(1.0, 0.0, -1.0), block_size),
        AABB::new(Vec3::new(0.0, 1.0, -1.0), block_size),
        AABB::new(Vec3::new(1.0, 1.0, -1.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(1.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 1.0, 0.0), block_size),
        AABB::new(Vec3::new(1.0, 1.0, 0.0), block_size),
    ];

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xp_yn_zp() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(0.5, -0.5, 0.5), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        AABB::new(Vec3::new(0.0, -1.0, 0.0), block_size),
        AABB::new(Vec3::new(1.0, -1.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(1.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, -1.0, 1.0), block_size),
        AABB::new(Vec3::new(1.0, -1.0, 1.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 1.0), block_size),
        AABB::new(Vec3::new(1.0, 0.0, 1.0), block_size),
    ];

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xp_yn_zn() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(0.5, -0.5, -0.5), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        AABB::new(Vec3::new(0.0, -1.0, -1.0), block_size),
        AABB::new(Vec3::new(1.0, -1.0, -1.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, -1.0), block_size),
        AABB::new(Vec3::new(1.0, 0.0, -1.0), block_size),
        AABB::new(Vec3::new(0.0, -1.0, 0.0), block_size),
        AABB::new(Vec3::new(1.0, -1.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(1.0, 0.0, 0.0), block_size),
    ];

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xn_yp_zp() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(-0.5, 0.5, 0.5), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        AABB::new(Vec3::new(-1.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(-1.0, 1.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 1.0, 0.0), block_size),
        AABB::new(Vec3::new(-1.0, 0.0, 1.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 1.0), block_size),
        AABB::new(Vec3::new(-1.0, 1.0, 1.0), block_size),
        AABB::new(Vec3::new(0.0, 1.0, 1.0), block_size),
    ];

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xn_yp_zn() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(-0.5, 0.5, -0.5), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        AABB::new(Vec3::new(-1.0, 0.0, -1.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, -1.0), block_size),
        AABB::new(Vec3::new(-1.0, 1.0, -1.0), block_size),
        AABB::new(Vec3::new(0.0, 1.0, -1.0), block_size),
        AABB::new(Vec3::new(-1.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(-1.0, 1.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 1.0, 0.0), block_size),
    ];

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xn_yn_zp() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(-0.5, -0.5, 0.5), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        AABB::new(Vec3::new(-1.0, -1.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, -1.0, 0.0), block_size),
        AABB::new(Vec3::new(-1.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(-1.0, -1.0, 1.0), block_size),
        AABB::new(Vec3::new(0.0, -1.0, 1.0), block_size),
        AABB::new(Vec3::new(-1.0, 0.0, 1.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, 1.0), block_size),
    ];

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}

#[test]
fn overlapping_aabb_at_xn_yn_zn() {
    let block_size = Vec3::splat(BLOCK_SIZE);

    let aabb_test = AABB::new(Vec3::new(-0.5, -0.5, -0.5), block_size);

    let aabb_list_test = grid::get_overlapping_aabb_list(&aabb_test);

    let aabb_list_control = vec![
        AABB::new(Vec3::new(0.0, 0.0, 0.0), block_size),
        AABB::new(Vec3::new(-1.0, -1.0, -1.0), block_size),
        AABB::new(Vec3::new(0.0, -1.0, -1.0), block_size),
        AABB::new(Vec3::new(-1.0, 0.0, -1.0), block_size),
        AABB::new(Vec3::new(0.0, 0.0, -1.0), block_size),
        AABB::new(Vec3::new(-1.0, -1.0, 0.0), block_size),
        AABB::new(Vec3::new(0.0, -1.0, 0.0), block_size),
        AABB::new(Vec3::new(-1.0, 0.0, 0.0), block_size),
    ];

    let list_matches =
        AABB::approx_aabb_set_eq(&aabb_list_test, &aabb_list_control, EPSILON_COLLISION);

    assert!(list_matches, "AABB list does not match control");
}
