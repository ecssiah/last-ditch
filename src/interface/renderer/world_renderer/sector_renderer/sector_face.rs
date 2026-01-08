use crate::{
    interface::asset_manager::layer_index::LayerIndex, simulation::state::world::grid::Direction,
};
use ultraviolet::Vec3;

#[derive(Clone, PartialEq)]
pub struct SectorFace {
    pub world_position: Vec3,
    pub direction: Direction,
    pub layer_index: LayerIndex,
}

impl SectorFace {
    pub fn new(world_position: Vec3, direction: Direction, layer_index: LayerIndex) -> Self {
        Self {
            world_position,
            direction,
            layer_index,
        }
    }
}
