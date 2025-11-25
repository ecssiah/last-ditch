use crate::simulation::{observation::view::BlockView, state::world::sector};
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_id: sector::ID,
    pub version: u64,
    pub world_position: Vec3,
    pub radius: f32,
    pub block_view_vec: Vec<Option<BlockView>>,
}
