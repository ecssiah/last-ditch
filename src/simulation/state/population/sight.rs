use crate::{
    simulation::constants::{SECTOR_RADIUS_IN_METERS, SECTOR_SIZE_IN_METERS},
    utils::ld_math::rotor3_ext,
};
use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Copy, Debug, Default)]
pub struct Sight {
    pub world_position: Vec3,
    pub relative_position: Vec3,
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
        let world_position = Vec3::zero();
        let relative_position = Vec3::zero();
        let rotation_xy = 0.0;
        let rotation_yz = 0.0;
        let rotor = Rotor3::identity();
        let horizontal_fov = 180.0;
        let vertical_fov = 60.0;
        let range_in_meters = 12.0;
        let range_in_sectors = 1;

        Self {
            world_position,
            relative_position,
            rotation_xy,
            rotation_yz,
            rotor,
            horizontal_fov,
            vertical_fov,
            range_in_meters,
            range_in_sectors,
        }
    }

    pub fn get_forward(sight: &Sight) -> Vec3 {
        sight.rotor * Vec3::unit_y()
    }

    pub fn set_relative_position(relative_position: Vec3, sight: &mut Sight) {
        sight.relative_position = relative_position;
    }

    pub fn set_world_position(world_position: Vec3, sight: &mut Sight) {
        sight.world_position = world_position;
    }

    pub fn set_rotation(rotation_xy: f32, rotation_yz: f32, sight: &mut Sight) {
        sight.rotation_xy = rotation_xy;
        sight.rotation_yz = rotation_yz;

        let rotation_xy_radians = rotation_xy.to_radians();
        let rotation_yz_radians = rotation_yz.to_radians();

        let rotor_xy = Rotor3::from_rotation_xy(rotation_xy_radians);
        let rotor_yz = Rotor3::from_rotation_yz(rotation_yz_radians);

        sight.rotor = rotor_xy * rotor_yz;
    }

    pub fn set_range(range: f32, sight: &mut Sight) {
        sight.range_in_meters = range;
        sight.range_in_sectors =
            ((range - SECTOR_RADIUS_IN_METERS) / SECTOR_SIZE_IN_METERS).ceil() as i32;
    }

    pub fn contains(sight: &Sight, point: Vec3) -> bool {
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
