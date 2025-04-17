use crate::simulation::{self};
use glam::Vec2;

pub struct TextureAtlas {
    pub label: String,
    pub tile_size: u32,
    pub width: u32,
    pub height: u32,
}

impl TextureAtlas {
    pub fn new(label: String, tile_size: u32, width: u32, height: u32) -> TextureAtlas {
        let texture_atlas = TextureAtlas {
            label,
            tile_size,
            width,
            height,
        };

        texture_atlas
    }

    pub fn get_uv_coords(&self, tile_x: u32, tile_y: u32) -> Vec<Vec2> {
        let block_size = simulation::consts::BLOCK_SIZE as u32;

        let u_min = (tile_x * self.tile_size) as f32 / self.width as f32;
        let v_min = (tile_y * self.tile_size) as f32 / self.height as f32;
        let u_max = ((tile_x + block_size) * self.tile_size) as f32 / self.width as f32;
        let v_max = ((tile_y + block_size) * self.tile_size) as f32 / self.height as f32;

        Vec::from([
            Vec2::new(u_min, v_max),
            Vec2::new(u_max, v_max),
            Vec2::new(u_max, v_min),
            Vec2::new(u_min, v_min),
        ])
    }
}
