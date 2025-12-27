use crate::{
    simulation::{
        constants::CELL_RADIUS_IN_METERS,
        state::world::{block::Block, grid, object::Object},
    },
    utils::ldmath::FloatBox,
};
use ultraviolet::{IVec3, Vec3};

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_index: usize,
    pub sector_index: usize,
    pub grid_position: IVec3,
    pub block: Option<Block>,
    pub object: Option<Object>,
}

impl Cell {
    #[inline]
    pub fn get_float_box(grid_position: IVec3) -> FloatBox {
        let world_position = grid::grid_position_to_world_position(grid_position);
        let radius = Vec3::broadcast(CELL_RADIUS_IN_METERS as f32);

        FloatBox::new(world_position, radius)
    }
}
