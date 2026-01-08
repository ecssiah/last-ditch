use ultraviolet::Vec3;

#[derive(Clone, PartialEq)]
pub struct BlockQuad {
    pub position_array: [Vec3; 4],
    pub normal_array: [Vec3; 4],
    pub uv_array: [Vec3; 4],
}

#[derive(Clone, PartialEq)]
pub struct BlockAsset {
    pub block_quad_vec: Vec<BlockQuad>,
}
