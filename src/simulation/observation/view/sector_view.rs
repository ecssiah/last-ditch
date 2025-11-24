use crate::simulation::{observation::view::{BlockView}, state::world::sector};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_id: sector::ID,
    pub version: u64,
    pub world_position: Vec3,
    pub radius: f32,
    pub block_view_vec: Vec<Option<BlockView>>,
}

impl SectorView {
    pub fn new() -> Self {
        Self {
            sector_id: sector::ID::MAX,
            version: 0,
            world_position: Vec3::zero(),
            radius: 0.0,
            block_view_vec: Vec::new(),
        }
    }
}
