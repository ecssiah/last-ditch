pub mod block_info;
pub mod block_kind;

pub use block_kind::BlockKind;

use crate::{simulation::state::world::grid::direction_set::DirectionSet, utils::ldmath::FloatBox};

#[derive(Clone, Debug)]
pub struct Block {
    pub block_kind: BlockKind,
    pub float_box_vec: Vec<FloatBox>,
    pub exposure_set: DirectionSet,
}

impl Block {
    pub fn new(block_kind: &BlockKind) -> Self {
        let float_box_vec = Vec::new();
        let exposure_set = DirectionSet::EMPTY;

        Self {
            block_kind: block_kind.clone(),
            float_box_vec,
            exposure_set,
        }
    }
}
