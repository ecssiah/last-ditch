use crate::{
    simulation::constants::{
        JUDGE_DEFAULT_RADIUS_Z, SECTOR_RADIUS_IN_METERS, SECTOR_SIZE_IN_METERS,
    },
    utils::ldmath::rotor3_ext,
};
use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Debug)]
pub struct Sight {
    pub local_position: Vec3,
    pub world_position: Vec3,
    pub rotation_xy: f32,
    pub rotation_yz: f32,
    pub rotor: Rotor3,
    pub horizontal_fov: f32,
    pub vertical_fov: f32,
    pub range_in_meters: f32,
    pub range_in_sectors: i32,
}

impl Sight {
    pub fn new() -> Self {
        let local_position = Vec3::new(0.0, 0.0, 0.9 * (2.0 * JUDGE_DEFAULT_RADIUS_Z));
        let world_position = Vec3::zero();
        let rotation_xy = 0.0;
        let rotation_yz = 0.0;
        let rotor = Rotor3::identity();
        let horizontal_fov = 180.0;
        let vertical_fov = 60.0;
        let range_in_meters = 100.0;
        let range_in_sectors = Self::calculate_range_in_sectors(range_in_meters);

        Self {
            world_position,
            local_position,
            rotation_xy,
            rotation_yz,
            rotor,
            horizontal_fov,
            vertical_fov,
            range_in_meters,
            range_in_sectors,
        }
    }

    pub fn get_forward(sight: &Self) -> Vec3 {
        sight.rotor * Vec3::unit_y()
    }

    pub fn set_local_position(local_position: Vec3, sight: &mut Self) {
        sight.local_position = local_position;
    }

    pub fn set_world_position(world_position: Vec3, sight: &mut Self) {
        sight.world_position = world_position;
    }

    pub fn set_rotation(rotation_xy: f32, rotation_yz: f32, sight: &mut Self) {
        sight.rotation_xy = rotation_xy;
        sight.rotation_yz = rotation_yz;

        let rotor_xy = rotor3_ext::from_rotation_xy_deg(rotation_xy);
        let rotor_yz = rotor3_ext::from_rotation_yz_deg(rotation_yz);

        sight.rotor = rotor_xy * rotor_yz;
    }

    pub fn set_range(range: f32, sight: &mut Self) {
        sight.range_in_meters = range;
        sight.range_in_sectors = Self::calculate_range_in_sectors(range);
    }

    fn calculate_range_in_sectors(range: f32) -> i32 {
        ((range - SECTOR_RADIUS_IN_METERS) / SECTOR_SIZE_IN_METERS).ceil() as i32
    }

    pub fn contains(sight: &Self, point: Vec3) -> bool {
        let to_point = point - sight.world_position;
        let distance = to_point.mag();

        if distance > sight.range_in_meters {
            return false;
        }

        let dot = to_point.normalized().dot(rotor3_ext::forward(sight.rotor));

        let angle = dot.acos();

        angle <= (sight.horizontal_fov * 0.5).to_radians()
    }
}

impl Default for Sight {
    fn default() -> Self {
        Self::new()
    }
}
