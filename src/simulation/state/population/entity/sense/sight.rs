use crate::{simulation::state::world::grid::Grid, utils::ld_math::rotor3_ext::Rotor3Ext};
use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Copy, Debug, Default)]
pub struct Sight {
    pub position: Vec3,
    pub eye_offset: Vec3,
    pub rotor: Rotor3,
    pub horizontal_fov: f32,
    pub vertical_fov: f32,
    pub range_in_meters: f32,
    pub range_in_sectors: i32,
}

impl Sight {
    pub fn new() -> Self {
        let position = Vec3::zero();
        let eye_offset = Vec3::zero();
        let rotor = Rotor3::identity();
        let horizontal_fov = 180.0;
        let vertical_fov = 60.0;
        let range_in_meters = 12.0;
        let range_in_sectors = 1;

        Self {
            position,
            eye_offset,
            rotor,
            horizontal_fov,
            vertical_fov,
            range_in_meters,
            range_in_sectors,
        }
    }

    pub fn set_world_position(world_position: Vec3, sight: &mut Sight) {
        sight.position = world_position + sight.eye_offset;
    }

    pub fn set_rotation(yaw: f32, pitch: f32, sight: &mut Sight) {
        let yaw = yaw.to_radians();
        let pitch = pitch.to_radians();

        sight.rotor =
            Rotor3::from_euler_angles(0.0, 0.0, -yaw) * Rotor3::from_euler_angles(0.0, pitch, 0.0);
    }

    pub fn set_range(range: f32, grid: &Grid, sight: &mut Sight) {
        sight.range_in_meters = range;
        sight.range_in_sectors =
            ((range - grid.sector_radius_in_meters) / grid.sector_size_in_meters).ceil() as i32;
    }

    pub fn contains(sight: &Sight, point: Vec3) -> bool {
        let to_point = point - sight.position;
        let distance = to_point.mag();

        if distance > sight.range_in_meters {
            return false;
        }

        let dot = to_point.normalized().dot(Rotor3Ext::forward(sight.rotor));

        let angle = dot.acos();

        angle <= (sight.horizontal_fov * 0.5).to_radians()
    }
}
