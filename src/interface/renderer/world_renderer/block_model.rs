use crate::interface::renderer::world_renderer::block_quad::BlockQuad;

#[derive(Clone, Debug)]
pub struct BlockModel {
    pub primitive_array: &'static [BlockQuad],
}
