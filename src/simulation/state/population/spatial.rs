use crate::simulation::state::{
    physics::aabb::AABB,
    world::grid::{self},
};
use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Spatial {
    pub world_position: Vec3,
    pub rotation_xy: f32,
    pub rotor: Rotor3,
    pub sector_id: usize,
    pub sector_updated: bool,
    pub size: Vec3,
    pub body: AABB,
}

impl Spatial {
    pub fn new() -> Self {
        Self {
            world_position: Vec3::default(),
            rotation_xy: 0.0,
            sector_id: 0,
            sector_updated: false,
            size: Vec3::default(),
            body: AABB::default(),
            rotor: Rotor3::default(),
        }
    }

    pub fn set_world_position(world_position: Vec3, spatial: &mut Spatial) {
        spatial.world_position = world_position;
        spatial.body.set_bottom_center(world_position);
    }

    pub fn set_size(size: Vec3, spatial: &mut Spatial) {
        spatial.size = size;
        spatial.body.set_size(size);
    }

    pub fn set_rotation(rotation_xy: f32, spatial: &mut Spatial) {
        let rotation_xy_radians = rotation_xy.to_radians();

        spatial.rotor = Rotor3::from_rotation_xy(rotation_xy_radians);
    }

    pub fn update_sector_id(spatial: &mut Spatial) {
        let sector_id = grid::world_position_to_sector_id(spatial.world_position);

        if sector_id != spatial.sector_id {
            spatial.sector_updated = true;
            spatial.sector_id = sector_id;
        }
    }
}
