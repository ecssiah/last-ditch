use crate::simulation::{manager::viewer::face_mask::FaceMask, state::world::object::block};

#[derive(Clone, Debug)]
pub struct BlockView {
    pub block_kind: block::Kind,
    pub face_mask: FaceMask,
}
