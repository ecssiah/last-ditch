use crate::{
    interface::renderer::{
        texture::texture_layer_index::TextureLayerIndex, world_renderer::cell_rect::CellRect,
    },
    simulation::state::world::grid::Direction,
};

#[derive(Clone, PartialEq)]
pub struct SectorQuad {
    pub direction: Direction,
    pub cell_rect: CellRect,
    pub texture_layer_index: TextureLayerIndex,
    pub uv_array: [[f32; 2]; 4],
}

impl SectorQuad {
    pub fn new(
        direction: Direction,
        cell_rect: CellRect,
        texture_layer_index: TextureLayerIndex,
        uv_array: [[f32; 2]; 4],
    ) -> Self {
        Self {
            direction,
            cell_rect,
            texture_layer_index,
            uv_array,
        }
    }
}
