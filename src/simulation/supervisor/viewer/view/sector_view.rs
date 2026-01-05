use crate::simulation::{
    state::world::{block::Block, grid, sector::Sector},
    supervisor::viewer::view::StructureView,
};
use ultraviolet::Vec3;

#[derive(Clone)]
pub struct SectorView {
    pub sector_index: usize,
    pub version: u64,
    pub world_position: Vec3,
    pub block_vec: Vec<Option<Block>>,
    pub structure_view_vec: Vec<StructureView>,
}

impl SectorView {
    pub fn new_from_sector(sector: &Sector) -> Self {
        let structure_view_vec = sector
            .structure_vec
            .iter()
            .map(|structure| StructureView::new_from_structure(structure))
            .collect();

        Self {
            sector_index: sector.sector_index,
            version: sector.version,
            world_position: grid::grid_position_to_world_position(sector.grid_position),
            block_vec: sector.block_vec.clone(),
            structure_view_vec,
        }
    }
}
