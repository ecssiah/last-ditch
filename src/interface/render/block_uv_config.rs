use crate::{
    interface::render,
    simulation::block::{self, Direction},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct BlockUVConfig {
    pub kind: block::Kind,
    pub face_uvs: HashMap<String, (u32, u32)>,
}

impl TryFrom<BlockUVConfig> for render::BlockUV {
    type Error = String;

    fn try_from(block_uv_config: BlockUVConfig) -> Result<Self, Self::Error> {
        let mut face_uvs = HashMap::new();

        for (key, value) in block_uv_config.face_uvs {
            let direction = match key.as_str() {
                "XP" => Direction::XP,
                "XN" => Direction::XN,
                "YP" => Direction::YP,
                "YN" => Direction::YN,
                "ZP" => Direction::ZP,
                "ZN" => Direction::ZN,
                _ => return Err(format!("Invalid direction: {}", key)),
            };
            face_uvs.insert(direction, [value.0, value.1]);
        }
        Ok(render::BlockUV {
            kind: block_uv_config.kind,
            face_uvs,
        })
    }
}
