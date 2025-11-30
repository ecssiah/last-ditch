use crate::simulation::manager::viewer::BlockView;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_id: usize,
    pub version: u64,
    pub world_position: Vec3,
    pub block_view_vec: Vec<Option<BlockView>>,
}
