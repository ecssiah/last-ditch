use crate::simulation::manager::viewer::view::{BlockView, ObjectView};
use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct CellView {
    pub cell_index: usize,
    pub grid_position: IVec3,
    pub block_view: Option<BlockView>,
    pub object_view: Option<ObjectView>,
}

impl CellView {
    pub fn new() -> Self {
        Self {
            cell_index: 0,
            grid_position: IVec3::zero(),
            block_view: None,
            object_view: None,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
