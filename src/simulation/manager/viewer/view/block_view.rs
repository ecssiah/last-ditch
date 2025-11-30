use crate::simulation::{manager::viewer::face_mask::FaceMask, state::world::block};

#[derive(Clone, Debug)]
pub struct BlockView {
    pub cell_id: usize,
    pub block_kind: block::Kind,
    pub face_mask: FaceMask,
}
