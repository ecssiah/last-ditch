use crate::simulation::manager::viewer::view::CellView;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct SectorView {
    pub sector_id: usize,
    pub version: u64,
    pub world_position: Vec3,
    pub cell_view_vec: Vec<CellView>,
}
