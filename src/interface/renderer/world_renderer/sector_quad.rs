use crate::{
    interface::renderer::{
        texture::texture_location::TextureLocation, world_renderer::cell_rect::CellRect,
    },
    simulation::state::world::grid::Direction,
};

#[derive(Clone, PartialEq)]
pub struct SectorQuad {
    pub direction: Direction,
    pub cell_rect: CellRect,
    pub texture_location: TextureLocation,
    pub uv_array: [[f32; 2]; 4],
}

impl SectorQuad {
    pub fn new(
        direction: &Direction,
        cell_rect: &CellRect,
        texture_location: &TextureLocation,
        uv_array: &[[f32; 2]; 4],
    ) -> Self {
        Self {
            direction: direction.clone(),
            cell_rect: cell_rect.clone(),
            texture_location: texture_location.clone(),
            uv_array: uv_array.clone(),
        }
    }
}
