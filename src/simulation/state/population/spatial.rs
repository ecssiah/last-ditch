use crate::simulation::{
    constants::PITCH_LIMIT,
    state::{physics::aabb::AABB, world::{grid::Grid, sector}},
};
use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Spatial {
    pub world_position: Vec3,
    pub sector_id: sector::ID,
    pub sector_updated: bool,
    pub size: Vec3,
    pub body: AABB,
    pub yaw: f32,
    pub pitch: f32,
    pub rotor: Rotor3,
}

impl Spatial {
    pub fn new() -> Self {
        Self {
            world_position: Vec3::default(),
            sector_id: sector::ID(0),
            sector_updated: false,
            size: Vec3::default(),
            body: AABB::default(),
            yaw: 0.0,
            pitch: 0.0,
            rotor: Rotor3::default(),
        }
    }

    pub fn set_world_position(world_position: Vec3, spatial: &mut Spatial) {
        spatial.world_position = world_position;
        spatial.body.set_bottom_center(world_position.x, world_position.y, world_position.z)
    }

    pub fn set_size(size: Vec3, spatial: &mut Spatial) {
        spatial.size = size;
        spatial.body.set_size(size);
    }

    pub fn set_rotation(yaw: f32, pitch: f32, spatial: &mut Spatial) {
        spatial.yaw = yaw.to_radians();
        spatial.pitch = pitch.to_radians();
        spatial.pitch = spatial.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

        spatial.rotor = Rotor3::from_euler_angles(0.0, 0.0, -spatial.yaw)
            * Rotor3::from_euler_angles(0.0, spatial.pitch, 0.0);
    }

    pub fn update_sector_id(grid: &Grid, spatial: &mut Spatial) {
        let sector_id = Grid::world_position_to_sector_id(spatial.world_position, grid);

        if sector_id != spatial.sector_id {
            spatial.sector_updated = true;
            spatial.sector_id = sector_id;
        }
    }
}
